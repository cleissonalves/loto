use anyhow::{anyhow, bail, Result};
use std::io::{self, BufRead, BufReader};

pub fn vec_u8_from_str_ignore_err(input: &str) -> Vec<u8> {
    input
        .split([' ', '-', ','])
        .map(|s| s.trim())
        .filter(|s| s.is_empty() == false)
        .filter_map(|s| s.parse::<u8>().ok())
        .collect()
}

pub fn vec_u8_from_str(input: &str) -> Result<Vec<u8>> {
    input
        .split([' ', '-', ','])
        .map(|s| s.trim())
        .filter(|s| s.is_empty() == false)
        .map(|s| {
            s.parse::<u8>()
                .map_err(|_| anyhow!("Numero invalido '{}'", s))
        })
        .collect()
}

pub fn vstrings_to_vu8s(input: &Vec<String>) -> Result<Vec<Vec<u8>>> {
    let mut vecs_u8: Vec<Vec<u8>> = Vec::new();

    for st in input {
        let vu8 = vec_u8_from_str(st)?;
        vecs_u8.push(vu8);
    }

    Ok(vecs_u8)
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
