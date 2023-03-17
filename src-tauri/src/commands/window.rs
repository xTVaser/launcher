use std::path::Path;

use log::info;
use tauri::Manager;

use super::CommandError;

#[tauri::command]
pub async fn close_splashscreen(window: tauri::Window) -> Result<(), CommandError> {
  // Close splashscreen
  if let Some(splashscreen) = window.get_window("splashscreen") {
    info!("closing the splash screen");
    splashscreen.close()?;
  }
  // Show main window
  if let Some(main_window) = window.get_window("main") {
    info!("opening the main window");
    main_window.show()?;
  }
  Ok(())
}

#[tauri::command]
pub async fn open_dir_in_os(directory: String) -> Result<(), CommandError> {
  let folder_path = Path::new(&directory);

  if !folder_path.exists() {
    return Err(CommandError::OSOperation(format!(
      "Can't open folder '{}', doesn't exist",
      folder_path.display()
    )));
  }

  crate::util::os::open_dir_in_os(folder_path.to_string_lossy().into_owned())
    .map_err(|_| CommandError::OSOperation(format!("Unable to go to open folder in OS")))?;
  Ok(())
}
