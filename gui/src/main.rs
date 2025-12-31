use eframe::egui;
use rfd::FileDialog;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex, atomic::{AtomicUsize, Ordering}};
use std::thread;
use anyhow::{Context, Error};
use std::fs::{self, File};
use std::io::{Seek, SeekFrom, Write};
use rayon::prelude::*;

use iso2god::executable::TitleInfo;
use iso2god::{game_list, god, iso};

#[derive(Debug, PartialEq, Eq, Clone, Copy, Default)]
enum TrimMode {
    #[default]
    FromEnd,
    None,
}

impl ToString for TrimMode {
    fn to_string(&self) -> String {
        match self {
            TrimMode::FromEnd => "Trim Unused Space (From End)".to_owned(),
            TrimMode::None => "Do Not Trim".to_owned(),
        }
    }
}

struct ConversionStatus {
    is_running: bool,
    stage: String, // "Idle", "Metadata", "Writing Parts", "Calculating Hash", "Done"
    progress_current: usize,
    progress_total: usize,
    error: Option<String>,
    success: bool,
}

impl Default for ConversionStatus {
    fn default() -> Self {
        Self {
            is_running: false,
            stage: "Idle".to_owned(),
            progress_current: 0,
            progress_total: 0,
            error: None,
            success: false,
        }
    }
}

struct Iso2GodApp {
    source_iso: Option<PathBuf>,
    dest_dir: Option<PathBuf>,
    game_title: String,
    trim_mode: TrimMode,
    
    status: Arc<Mutex<ConversionStatus>>,
}

impl Default for Iso2GodApp {
    fn default() -> Self {
        Self {
            source_iso: None,
            dest_dir: None,
            game_title: String::new(),
            trim_mode: TrimMode::default(),
            status: Arc::new(Mutex::new(ConversionStatus::default())),
        }
    }
}

impl Iso2GodApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // Initialize rayon thread pool globally if needed, 
        // though rayon initializes lazily by default. 
        // But iso2god binary did it explicitly.
        // We can just let it be.
        Self::default()
    }

    fn start_conversion(&mut self) {
        if self.source_iso.is_none() || self.dest_dir.is_none() {
            return;
        }

        let source_iso_path = self.source_iso.clone().unwrap();
        let dest_dir_path = self.dest_dir.clone().unwrap();
        let game_title_arg = if self.game_title.trim().is_empty() {
            None
        } else {
            Some(self.game_title.clone())
        };
        let trim_mode = self.trim_mode;
        let status_arc = self.status.clone();

        {
            let mut status = status_arc.lock().unwrap();
            status.is_running = true;
            status.stage = "Starting...".to_owned();
            status.error = None;
            status.success = false;
            status.progress_current = 0;
            status.progress_total = 0;
        }

        thread::spawn(move || {
            let result = convert_process(
                source_iso_path,
                dest_dir_path,
                game_title_arg,
                trim_mode,
                status_arc.clone(),
            );

            let mut status = status_arc.lock().unwrap();
            status.is_running = false;
            match result {
                Ok(_) => {
                    status.stage = "Conversion Complete!".to_owned();
                    status.success = true;
                }
                Err(e) => {
                    status.stage = "Error".to_owned();
                    status.error = Some(e.to_string());
                }
            }
        });
    }
}

impl eframe::App for Iso2GodApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let status_arc = self.status.clone();
        let status = status_arc.lock().unwrap();

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Iso2God GUI");
            ui.add_space(10.0);

            // Inputs
            ui.group(|ui| {
                ui.label("Source ISO:");
                ui.horizontal(|ui| {
                    if let Some(path) = &self.source_iso {
                        ui.monospace(path.to_string_lossy());
                    } else {
                        ui.label("(None selected)");
                    }
                    if !status.is_running && ui.button("Select ISO...").clicked() {
                        if let Some(path) = FileDialog::new()
                            .add_filter("ISO", &["iso", "ISO"])
                            .pick_file() {
                            self.source_iso = Some(path);
                        }
                    }
                });

                ui.add_space(5.0);

                ui.label("Destination Directory:");
                ui.horizontal(|ui| {
                    if let Some(path) = &self.dest_dir {
                        ui.monospace(path.to_string_lossy());
                    } else {
                        ui.label("(None selected)");
                    }
                    if !status.is_running && ui.button("Select Folder...").clicked() {
                        if let Some(path) = FileDialog::new().pick_folder() {
                            self.dest_dir = Some(path);
                        }
                    }
                });

                ui.add_space(5.0);

                ui.label("Game Title (Optional):");
                ui.text_edit_singleline(&mut self.game_title);

                ui.add_space(5.0);

                ui.label("Trim Mode:");
                egui::ComboBox::from_id_salt("trim_mode")
                    .selected_text(self.trim_mode.to_string())
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut self.trim_mode, TrimMode::FromEnd, TrimMode::FromEnd.to_string());
                        ui.selectable_value(&mut self.trim_mode, TrimMode::None, TrimMode::None.to_string());
                    });
            });

            ui.add_space(20.0);

            // Actions and Status
            if status.is_running {
                ui.label(format!("Status: {}", status.stage));
                if status.progress_total > 0 {
                    let progress = status.progress_current as f32 / status.progress_total as f32;
                    ui.add(egui::ProgressBar::new(progress).show_percentage());
                    ui.label(format!("Part {}/{}", status.progress_current, status.progress_total));
                } else {
                    ui.spinner();
                }
            } else {
                if let Some(err) = &status.error {
                    ui.colored_label(egui::Color32::RED, format!("Error: {}", err));
                }
                if status.success {
                    ui.colored_label(egui::Color32::GREEN, "Success!");
                }

                ui.add_space(10.0);
                
                let can_start = self.source_iso.is_some() && self.dest_dir.is_some();
                if ui.add_enabled(can_start, egui::Button::new("Convert")).clicked() {
                    // Drop lock before starting to avoid deadlock (though start_conversion re-locks, it's fine)
                    drop(status); 
                    self.start_conversion();
                    return; // Return early to avoid re-locking status in this frame if we wanted
                }
            }
        });
        
        // Request repaint if running to animate spinner/progress
        if status_arc.lock().unwrap().is_running {
            ctx.request_repaint();
        }
    }
}

fn main() -> Result<(), eframe::Error> {
    env_logger::init();
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([600.0, 400.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Iso2God GUI",
        options,
        Box::new(|cc| Ok(Box::new(Iso2GodApp::new(cc)))),
    )
}

// --- Conversion Logic (Adapted from iso2god-rs/src/bin/iso2god.rs) ---

fn convert_process(
    source_iso_path: PathBuf,
    dest_dir_path: PathBuf,
    game_title_arg: Option<String>,
    trim_mode: TrimMode,
    status_arc: Arc<Mutex<ConversionStatus>>,
) -> Result<(), Error> {
    
    // Update status: Extracting Metadata
    {
        let mut s = status_arc.lock().unwrap();
        s.stage = "Extracting ISO metadata...".to_owned();
    }

    let source_iso_file = File::open(&source_iso_path).context("error opening source ISO file")?;
    let source_iso_file_meta = fs::metadata(&source_iso_path).context("error reading source ISO file metadata")?;

    let mut source_iso = iso::IsoReader::read(source_iso_file).context("error reading source ISO")?;
    let title_info = TitleInfo::from_image(&mut source_iso).context("error reading image executable")?;

    let exe_info = title_info.execution_info;
    let content_type = title_info.content_type;

    let data_size = if trim_mode == TrimMode::FromEnd {
        source_iso.get_max_used_prefix_size()
    } else {
        let root_offset = source_iso.volume_descriptor.root_offset;
        source_iso_file_meta.len() - root_offset
    };

    let block_count = data_size.div_ceil(god::BLOCK_SIZE);
    let part_count = block_count.div_ceil(god::BLOCKS_PER_PART);

    let file_layout = god::FileLayout::new(&dest_dir_path, &exe_info, content_type);

    {
        let mut s = status_arc.lock().unwrap();
        s.stage = "Clearing data directory...".to_owned();
    }
    ensure_empty_dir(&file_layout.data_dir_path()).context("error clearing data directory")?;

    {
        let mut s = status_arc.lock().unwrap();
        s.stage = "Writing parts...".to_owned();
        s.progress_total = part_count as usize;
        s.progress_current = 0;
    }

    let progress_counter = AtomicUsize::new(0);

    // We can use rayon's par_iter, but we need to update progress.
    // We can update the Arc<Mutex> occasionally or use another atomic for tracking and sync it to status.
    
    // NOTE: rayon::ThreadPoolBuilder might not be configured here, using default.
    // If user has problems, they might need single thread, but for now we assume defaults work.

    (0..part_count).into_par_iter().try_for_each(|part_index| {
        let mut iso_data_volume = File::open(&source_iso_path)?;
        iso_data_volume.seek(SeekFrom::Start(source_iso.volume_descriptor.root_offset))?;

        let part_file_path = file_layout.part_file_path(part_index);
        
        // Ensure directory for part file exists (should be handled by ensure_empty_dir logic mostly, but part files are in subdirs?)
        // god::FileLayout::data_dir_path() creates the base.
        // Let's check if we need to create parent dirs for parts.
        if let Some(parent) = part_file_path.parent() {
            fs::create_dir_all(parent)?;
        }

        let part_file = File::options()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&part_file_path)
            .context("error creating part file")?;

        god::write_part(iso_data_volume, part_index, part_file)
            .context("error writing part file")?;

        let cur = 1 + progress_counter.fetch_add(1, Ordering::Relaxed);
        
        // Update status occasionally
        if let Ok(mut s) = status_arc.try_lock() {
            s.progress_current = cur;
        }

        Ok::<_, anyhow::Error>(())
    })?;

    // Final progress update
    {
        let mut s = status_arc.lock().unwrap();
        s.progress_current = part_count as usize;
        s.stage = "Calculating MHT hash chain...".to_owned();
    }

    let mut mht = read_part_mht(&file_layout, part_count - 1).context("error reading part file MHT")?;

    for prev_part_index in (0..part_count - 1).rev() {
        let mut prev_mht = read_part_mht(&file_layout, prev_part_index).context("error reading part file MHT")?;

        prev_mht.add_hash(&mht.digest());

        write_part_mht(&file_layout, prev_part_index, &prev_mht)
            .context("error writing part file MHT")?;

        mht = prev_mht;
    }

    let last_part_size = fs::metadata(file_layout.part_file_path(part_count - 1))
        .map(|m| m.len())
        .context("error reading part file")?;

    {
        let mut s = status_arc.lock().unwrap();
        s.stage = "Writing CON header...".to_owned();
    }

    let mut con_header = god::ConHeaderBuilder::new()
        .with_execution_info(&exe_info)
        .with_block_counts(block_count as u32, 0)
        .with_data_parts_info(
            part_count as u32,
            last_part_size + (part_count - 1) * god::BLOCK_SIZE * 0xa290,
        )
        .with_content_type(content_type)
        .with_mht_hash(&mht.digest());

    let found_title = game_list::find_title_by_id(exe_info.title_id);
    let title_to_use = game_title_arg.or(found_title);
    
    if let Some(t) = title_to_use {
         con_header = con_header.with_game_title(&t);
    }

    let con_header = con_header.finalize();

    let mut con_header_file = File::options()
        .write(true)
        .create(true)
        .truncate(true)
        .open(file_layout.con_header_file_path())
        .context("cannot open con header file")?;

    con_header_file
        .write_all(&con_header)
        .context("error writing con header file")?;

    Ok(())
}

fn ensure_empty_dir(path: &Path) -> Result<(), Error> {
    if path.exists() {
        fs::remove_dir_all(path)?;
    };
    fs::create_dir_all(path)?;
    Ok(())
}

fn read_part_mht(file_layout: &god::FileLayout, part_index: u64) -> Result<god::HashList, Error> {
    let part_file = file_layout.part_file_path(part_index);
    let mut part_file = File::options().read(true).open(part_file)?;
    god::HashList::read(&mut part_file)
}

fn write_part_mht(
    file_layout: &god::FileLayout,
    part_index: u64,
    mht: &god::HashList,
) -> Result<(), Error> {
    let part_file = file_layout.part_file_path(part_index);
    let mut part_file = File::options().write(true).open(part_file)?;
    mht.write(&mut part_file)?;
    Ok(())
}
