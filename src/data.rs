use crate::util;
use anyhow::{bail, Result};
use clap::ValueEnum;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::{collections::HashSet, ops::RangeInclusive};
use strum::{Display, EnumString};

#[derive(Serialize, Deserialize, Debug, Display, Clone, Copy, ValueEnum, EnumString)]
#[strum(serialize_all = "lowercase")] // for internal usage
#[serde(rename_all = "lowercase")] // for parsing json
pub enum Jogo {
    Lotofacil,
    Lotomania,
    Megasena,
    Quina,
}

fn are_all_distinct<T: Eq + std::hash::Hash>(vec: &[T]) -> bool {
    let set: HashSet<_> = vec.iter().collect();
    set.len() == vec.len()
}

impl Jogo {
    pub fn get_prize_matches(&self) -> Vec<u8> {
        match self {
            Jogo::Lotofacil => vec![11, 12, 13, 14, 15],
            Jogo::Lotomania => vec![0, 15, 16, 17, 18, 19, 20],
            Jogo::Megasena => vec![4, 5, 6],
            Jogo::Quina => vec![2, 3, 4, 5],
        }
    }

    fn get_quantity_range(&self) -> RangeInclusive<usize> {
        match self {
            Jogo::Lotofacil => 15..=18,
            Jogo::Lotomania => 50..=50,
            Jogo::Megasena => 6..=20,
            Jogo::Quina => 5..=15,
        }
    }
    fn get_numbers_range(&self) -> RangeInclusive<usize> {
        match self {
            Jogo::Lotofacil => 1..=25,
            Jogo::Lotomania => 0..=99,
            Jogo::Megasena => 1..=60,
            Jogo::Quina => 1..=80,
        }
    }
    pub fn validar(&self, numeros: &[u8]) -> Result<()> {
        if are_all_distinct(numeros) == false {
            bail!("Numeros invalidos. Contem valores repetidos.");
        }

        let quantity_range = self.get_quantity_range();
        let numbers_range = self.get_numbers_range();

        if quantity_range.contains(&numeros.len()) == false {
            bail!(
                "Quantidade invalida de numeros selecionados ({}). Escolha entre {} e {} numeros.",
                numeros.len(),
                quantity_range.start(),
                quantity_range.end()
            );
        }

        if numeros
            .iter()
            .any(|n| numbers_range.contains(&(*n as usize)) == false)
        {
            bail!(
                "Numeros invalidos selecionados. Escolha valores entre {} e {}.",
                numbers_range.start(),
                numbers_range.end()
            );
        }

        Ok(())
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Premiacao {
    pub descricao: String,
    pub ganhadores: u32,
    #[serde(rename = "valorPremio")]
    pub premio: f32,
}

impl std::fmt::Display for Premiacao {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} ({}, {})",
            self.descricao,
            self.ganhadores,
            util::formatar_dinheiro(&self.premio)
        )
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Sorteio {
    #[serde(rename = "loteria")]
    pub jogo: Jogo,
    #[serde(rename = "dezenas")]
    #[serde(deserialize_with = "deserialize_u8_from_strings")]
    #[serde(serialize_with = "serialize_u8_to_strings")]
    pub numeros: Vec<u8>,
    pub concurso: u32,
    pub acumulou: bool,
    #[serde(rename = "valorEstimadoProximoConcurso")]
    pub valor_acumulado: f32,
    pub premiacoes: Vec<Premiacao>,
    pub data: String,
    #[serde(rename = "dataProximoConcurso")]
    pub data_prox_sorteiro: String,
}

impl Sorteio {
    pub fn premiacoes(&self) -> String {
        let lista_premiacoes: Vec<String> = self
            .premiacoes
            .iter()
            .filter(|p| p.ganhadores > 0)
            .map(|p| format!("{}", p))
            .collect();
        lista_premiacoes.join("; ")
    }
}

fn serialize_u8_to_strings<S>(vec: &[u8], serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    // Convert each u8 to a String (in base 10)
    let strings: Vec<String> = vec.iter().map(|&x| x.to_string()).collect();

    // Serialize the Vec<String>
    serializer.serialize_some(&strings)
}

// Custom deserialization function
fn deserialize_u8_from_strings<'de, D>(deserializer: D) -> Result<Vec<u8>, D::Error>
where
    D: Deserializer<'de>,
{
    // First, we deserialize the field as Vec<string>
    let s: Vec<String> = Vec::deserialize(deserializer)?;

    // Try to decode the strings into a Vec<u8>
    let mut results = Vec::new();
    for value in s {
        let number = value.parse::<u8>().map_err(serde::de::Error::custom)?;
        results.push(number);
    }

    Ok(results)
}
