use tauri::{
  plugin::{Builder, TauriPlugin},
  Manager, Runtime,
};

pub use models::*;

#[cfg(desktop)]
mod desktop;
#[cfg(mobile)]
mod mobile;

mod commands;
mod error;
mod models;

pub use error::{Error, Result};

#[cfg(desktop)]
use desktop::Lingua;
#[cfg(mobile)]
use mobile::Lingua;

/// Extensions to [`tauri::App`], [`tauri::AppHandle`] and [`tauri::Window`] to access the lingua APIs.
pub trait LinguaExt<R: Runtime> {
  fn lingua(&self) -> &Lingua<R>;
}

impl<R: Runtime, T: Manager<R>> crate::LinguaExt<R> for T {
  fn lingua(&self) -> &Lingua<R> {
    self.state::<Lingua<R>>().inner()
  }
}

/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
  Builder::new("lingua")
    .invoke_handler(tauri::generate_handler![
      commands::ping,
      commands::create_detector_for_all_languages,
      commands::create_detector_for_languages,
      commands::detect_language,
      commands::compute_language_confidence,
      commands::compute_language_confidence_values,
    ])
    .setup(|app, api| {
      #[cfg(mobile)]
      let lingua = mobile::init(app, api)?;
      #[cfg(desktop)]
      let lingua = desktop::init(app, api)?;
      app.manage(lingua);
      Ok(())
    })
    .build()
}
