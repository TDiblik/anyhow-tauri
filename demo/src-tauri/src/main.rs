#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use anyhow_tauri::{IntoEmptyTAResult, IntoTAResult};

fn function_that_throws() -> anyhow::Result<()> {
    anyhow::bail!("Simulating a possible throw")
}

fn function_that_succeeds() -> anyhow::Result<String> {
    Ok("this function succeeds".to_string())
}

#[tauri::command]
fn test() -> anyhow_tauri::TAResult<String> {
    Ok("No error thrown.".into())
}

#[tauri::command]
fn test_anyhow_success() -> anyhow_tauri::TAResult<String> {
    function_that_succeeds().into_ta_result()

    // could also be written as
    // Ok(function_that_succeeds()?)
}

#[tauri::command]
fn test_throw() -> anyhow_tauri::TAResult<String> {
    function_that_throws()?;

    Ok("this should never trigger".to_owned())
}

#[tauri::command]
fn test_pure_err_conversion() -> anyhow_tauri::TAResult<String> {
    let _some_empty_err = anyhow::anyhow!("some err").into_ta_empty_result();
    anyhow::anyhow!("Showcase of the .into_ta_result()").into_ta_result()
}

#[tauri::command]
fn test_bail() -> anyhow_tauri::TAResult<String> {
    anyhow_tauri::bail!("Showcase of the .bail!()")
}

#[tauri::command]
fn test_ensure() -> anyhow_tauri::TAResult<String> {
    anyhow_tauri::ensure!(1 == 2); // this should throw

    Ok("this should never trigger".to_owned())
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            test,
            test_anyhow_success,
            test_throw,
            test_pure_err_conversion,
            test_bail,
            test_ensure
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
