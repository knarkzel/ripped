#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use anyhow::{Context, Error};
use eframe::{
    egui::{style::Spacing, *},
    epi, NativeOptions,
};
use fehler::throws;
use glob::glob;
use rusqlite::Connection;
use std::path::PathBuf;

enum Theme {
    Dark,
    Light,
}

#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "persistence", serde(default))]
struct State {
    connection: Connection,
    folder: String,
    theme: Theme,
    include_subfolders: bool,
}

impl State {
    #[throws]
    fn new() -> Self {
        Self {
            folder: String::from("/home/odd/.slippi"),
            connection: Connection::open_in_memory()?,
            theme: Theme::Light,
            include_subfolders: false,
        }
    }

    #[throws]
    fn replays(&self) -> Vec<PathBuf> {
        let path = match self.include_subfolders {
            true => format!("{}/**/*.slp", self.folder),
            false => format!("{}/*.slp", self.folder),
        };
        glob::glob(&path)?.flatten().collect()
    }

    fn toggle_theme(&mut self) {
        self.theme = match self.theme {
            Theme::Dark => Theme::Light,
            Theme::Light => Theme::Dark,
        };
    }

    fn load_theme(&self, ctx: &CtxRef) {
        match self.theme {
            Theme::Dark => ctx.set_visuals(Visuals::dark()),
            Theme::Light => ctx.set_visuals(Visuals::light()),
        }
    }
}

impl epi::App for State {
    fn name(&self) -> &str {
        "ripped"
    }

    fn setup(&mut self, ctx: &CtxRef, _: &epi::Frame, _: Option<&dyn epi::Storage>) {
        #[cfg(feature = "persistence")]
        if let Some(storage) = storage {
            *self = epi::get_value(storage, epi::APP_KEY).unwrap_or_default()
        }

        // Proper font
        let mut fonts = FontDefinitions::default();
        for font in fonts.family_and_size.iter_mut() {
            let family = font.1 .0;
            *font.1 = (family, 32.0);
        }
        ctx.set_fonts(fonts);

        // Theme
        self.load_theme(ctx);
    }

    #[cfg(feature = "persistence")]
    fn save(&mut self, storage: &mut dyn epi::Storage) {
        epi::set_value(storage, epi::APP_KEY, self);
    }

    fn update(&mut self, ctx: &CtxRef, _: &epi::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            // Replay
            ui.label("SLP Replay Directory");
            ui.horizontal(|ui| {
                ui.text_edit_singleline(&mut self.folder);
                if ui.button("🗀 Select folder").clicked() {
                    if let Some(path) = rfd::FileDialog::new().pick_folder() {
                        self.folder = path.display().to_string();
                    }
                }
            });

            // Theme
            if ui.button("Toggle theme").clicked() {
                self.toggle_theme();
                self.load_theme(ctx);
            }

            // Subfolders
            ui.checkbox(&mut self.include_subfolders, "Include subfolders");
        });
    }
}

fn main() {
    color_backtrace::install();
    let state = State::new().expect("Failure when creating state");
    eframe::run_native(Box::new(state), NativeOptions::default());
}
