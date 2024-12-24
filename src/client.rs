use crate::data::{Jogo, Sorteio};
use anyhow::{bail, Context, Ok, Result};
use std::fs::File;

// https://github.com/guto-alves/loterias-api
pub const API_PATH: &str = "https://loteriascaixa-api.herokuapp.com/api/";

pub fn fetch_latest(jogo: &Jogo) -> Result<Sorteio> {
    let request_url = format!("{}{}/latest", API_PATH, jogo);

    let response = minreq::get(request_url).send().context("Network Request")?;

    if response.status_code != 200 {
        bail!("{}: {}", response.status_code, response.reason_phrase);
    }

    let content = response.as_str().context("Parsing Response to json str")?;

    let value: Sorteio =
        serde_json::from_str(content).context("Deserializating json str to Sorteio")?;

    Ok(value)
}

pub fn fetch_all(jogo: &Jogo) -> Result<Vec<Sorteio>> {
    let request_url = format!("{}{}", API_PATH, jogo);

    let response = minreq::get(request_url)
        .send()
        .context("Network Request Error")?;

    if response.status_code != 200 {
        bail!("{}: {}", response.status_code, response.reason_phrase);
    }

    let content = response.as_str().context("Parsing Response to json str")?;

    let values: Vec<Sorteio> =
        serde_json::from_str(content).context("Deserializating json str to Vec<Sorteio>")?;

    Ok(values)
}

pub fn download_json(jogo: &Jogo) -> Result<()> {
    let request_url = format!("{}{}", API_PATH, jogo);

    let response = minreq::get(request_url).send().context("Network Request")?;

    if response.status_code != 200 {
        bail!("{}: {}", response.status_code, response.reason_phrase);
    }

    let content = response.as_str().context("Parsing Response to json str")?;

    let json: serde_json::Value =
        serde_json::from_str(content).context("Deserializing json str to serde_json::Value")?;

    let mut file = File::create(format!("{jogo}.json")).context("Opening File")?;

    serde_json::to_writer(&mut file, &json).context("Writing to File")?;

    Ok(())
}

// pub fn save_as_json(sorteios: &Vec<Sorteio>) -> Result<()> {
//     if let Some(record) = sorteios.first() {
//         let json_data = serde_json::to_string(sorteios)?;
//         let filename = format!("{}.json", record.jogo);
//         let mut file = File::create(filename)?;
//         file.write_all(json_data.as_bytes())?;
//         return Ok(());
//     }
//     Ok(())
// }

// pub fn load_from_json(jogo: &Jogo) -> Result<Vec<Sorteio>> {
//     let filepath = format!("{}.json", jogo);

//     // Open the file
//     let mut file = File::open(filepath)?;

//     // Read into String
//     let mut contents = String::new();
//     file.read_to_string(&mut contents)?;

//     // Deserialize
//     let sorteios: Vec<Sorteio> = serde_json::from_str(&contents)?;

//     Ok(sorteios)
// }
