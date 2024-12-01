use anyhow::{anyhow, bail, Result};
use std::io::{self, BufRead, BufReader};

pub fn vec_u8_from_str_ignore_err(input: &str) -> Vec<u8> {
	input
		.split(|c| c == ' ' || c == '-' || c == ',')
		.map(|s| s.trim())
		.filter(|s| s.is_empty() == false)
		.filter_map(|s| s.parse::<u8>().ok())
		.collect()
}

pub fn vec_u8_from_str(input: &str) -> Result<Vec<u8>> {
	input
		.split(|c| c == ' ' || c == '-' || c == ',')
		.map(|s| s.trim())
		.filter(|s| s.is_empty() == false)
		.map(|s| {
			s.parse::<u8>()
				.map_err(|_| anyhow!("Numero invalido '{}'", s))
		})
		.collect()
}

pub fn vec_u8_from_buffer() -> Result<Vec<Vec<u8>>> {
	let stdin = io::stdin();
	let reader = BufReader::new(stdin);

	let mut numbers: Vec<Vec<u8>> = Vec::new();

	for (index, line) in reader.lines().enumerate() {
		match line {
			Ok(line_content) => {
				let ns = vec_u8_from_str(&line_content)?;
				numbers.push(ns);
			}
			Err(e) => bail!("Erro ao ler linha {}: {}", index, e),
		}
	}

	Ok(numbers)
}
