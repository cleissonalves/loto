pub fn formatar_dinheiro(valor: &f32) -> String {
    if *valor > 1_000_000.0 {
        format!("R$ {:.0} milhoes", valor / 1_000_000.0)
    } else if *valor > 1_000.0 {
        format!("R$ {:.0} mil", valor / 1_000.0)
    } else {
        format!("R$ {:.0}", valor)
    }
}

// pub fn is_update_needed(jogo: &data::Jogo) -> bool {
// 	let filepath = format!("{}.json", jogo);
// 	let was_updated = was_file_recently_modified(&filepath).unwrap_or(false);

// 	if was_updated == false {
// 		return true;
// 	}

// 	return false;
// }

// fn was_file_recently_modified(filepath: &str) -> Result<bool> {
// 	let path = Path::new(filepath);

// 	let file_metadata =
// 		metadata(path).context(format!("Error retrieving metadata for: {}", filepath))?;

// 	let modified_time = file_metadata
// 		.modified()
// 		.context(format!("Error retrieving modified time for : {}", filepath))?;

// 	let current_time = SystemTime::now();
// 	let duration_since_file_modified = current_time
// 		.duration_since(modified_time)
// 		.context("Time went backwards")?;

// 	let hours = 12;
// 	// hour = 3600 seconds
// 	let duration_s = hours * 60 * 60;
// 	if duration_since_file_modified <= Duration::new(duration_s, 0) {
// 		return Ok(true);
// 	}

// 	Ok(false)
// }
