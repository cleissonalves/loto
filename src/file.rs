use crate::data::{Jogo, Sorteio};
use anyhow::{Context, Result};
use std::{
    env,
    fs::{self, metadata, File},
    io::{Read, Write},
    path::{Path, PathBuf},
    time::{Duration, SystemTime},
};

pub const APP_NAME: &str = "loto";
pub const UPDATE_HOURS_INTERVAL: u64 = 24;

// USES SYSTEM TEMP FOLDER
// On Windows: C:\Users\YourUser\AppData\Local\Temp\loto\<filename>.json
// On Linux: /tmp/my_app/<filename>.json
pub fn save_json(sorteios: &Vec<Sorteio>) -> Result<()> {
    let json_data = serde_json::to_string(sorteios)?;

    let jogo = sorteios.first().context("Lista de sorteios vazia.")?.jogo;
    let filepath = get_filepath(&jogo)?;

    let mut file = File::create(&filepath)?;
    file.write_all(json_data.as_bytes())?;

    Ok(())
}

// USES SYSTEM TEMP FOLDER
// On Windows: C:\Users\YourUser\AppData\Local\Temp\loto\<filename>.json
// On Linux: /tmp/loto/<filename>.json
pub fn load_json(jogo: &Jogo) -> Result<Vec<Sorteio>> {
    let filepath = get_filepath(jogo)?;

    // Open the file
    let mut file = File::open(&filepath)?;

    // Read into String
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    // Deserialize
    let sorteios: Vec<Sorteio> = serde_json::from_str(&contents)?;

    Ok(sorteios)
}

pub fn get_filepath(jogo: &Jogo) -> Result<PathBuf> {
    let filename = format!("{}.json", jogo);
    let filepath = PathBuf::from(&filename);

    Ok(filepath)
}

pub fn get_filepath_sys_tmp(jogo: &Jogo) -> Result<PathBuf> {
    let filename = format!("{}.json", jogo);

    // Get the system's temporary directory
    let temp_dir = env::temp_dir();

    // Create the path to a subfolder with the app's name in the temp directory
    let app_temp_dir = temp_dir.join(APP_NAME);

    // Create the subfolder if it doesn't exist
    fs::create_dir_all(&app_temp_dir)?;

    // Construct the path to the file inside the app's folder
    let filepath = app_temp_dir.join(&filename);

    Ok(filepath)
}

pub fn is_update_needed(jogo: &Jogo) -> Result<bool> {
    let filepath = get_filepath(jogo)?;
    let was_updated = was_file_recently_modified(&filepath).unwrap_or(false);

    if was_updated == false {
        return Ok(true);
    }

    Ok(false)
}

fn was_file_recently_modified(filepath: &PathBuf) -> Result<bool> {
    let path = Path::new(filepath);

    let file_metadata =
        metadata(path).context(format!("Error retrieving metadata for: {}", path.display()))?;

    let modified_time = file_metadata.modified().context(format!(
        "Error retrieving modified time for : {}",
        path.display()
    ))?;

    let current_time = SystemTime::now();
    let duration_since_file_modified = current_time
        .duration_since(modified_time)
        .context("Time went backwards")?;

    // hour = 60m * 60s = 3600s
    let duration_s = UPDATE_HOURS_INTERVAL * 3600;
    if duration_since_file_modified <= Duration::new(duration_s, 0) {
        return Ok(true);
    }

    Ok(false)
}
