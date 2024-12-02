// This file is unfinished and deprecated and is kept as an example
// Please use the sys_tmp.rs file instead.
#![deprecated]

use crate::data::{Jogo, Sorteio};
use anyhow::{Context, Result};
use std::{
    fs::{metadata, File},
    io::{Read, Write},
    path::Path,
    time::{Duration, SystemTime},
};

pub const APP_NAME: &'static str = "loto";

pub fn save_as_json(sorteios: &Vec<Sorteio>) -> Result<()> {
    let json_data = serde_json::to_string(sorteios)?;

    let jogo = sorteios.first().context("Lista de sorteios vazia.")?.jogo;
    let filename = format!("{}.json", jogo);

    let mut file = File::create(filename)?;
    file.write_all(json_data.as_bytes())?;

    Ok(())
}

pub fn load_from_json(jogo: &Jogo) -> Result<Vec<Sorteio>> {
    let filepath = format!("{}.json", jogo);

    // Open the file
    let mut file = File::open(filepath)?;

    // Read into String
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    // Deserialize
    let sorteios: Vec<Sorteio> = serde_json::from_str(&contents)?;

    Ok(sorteios)
}

pub fn is_update_needed(jogo: &Jogo) -> bool {
    let filepath = format!("{}.json", jogo);
    let was_updated = was_file_recently_modified(&filepath).unwrap_or(false);

    if was_updated == false {
        return true;
    }

    return false;
}

fn was_file_recently_modified(filepath: &str) -> Result<bool> {
    let path = Path::new(filepath);

    let file_metadata =
        metadata(path).context(format!("Error retrieving metadata for: {}", filepath))?;

    let modified_time = file_metadata
        .modified()
        .context(format!("Error retrieving modified time for : {}", filepath))?;

    let current_time = SystemTime::now();
    let duration_since_file_modified = current_time
        .duration_since(modified_time)
        .context("Time went backwards")?;

    let hours = 12;
    // hour = 3600 seconds
    let duration_s = hours * 60 * 60;
    if duration_since_file_modified <= Duration::new(duration_s, 0) {
        return Ok(true);
    }

    Ok(false)
}
