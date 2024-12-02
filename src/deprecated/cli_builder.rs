// This file is unfinished and deprecated and is kept as an example
// Please use the cli_derive.rs file instead.
#![deprecated]

use crate::commands::{consultar, consultar_numeros};
use crate::{data::Jogo, parsers};
use anyhow::{anyhow, Result};
use clap::{Arg, Command, ValueEnum};

// #[derive(Debug, Display)]
// #[strum(serialize_all = "lowercase")]
#[derive(Debug, ValueEnum, Clone)]
pub enum Acao {
    Consultar,
    Analisar,
}

pub struct Cli {
    pub action: Acao,
    pub jogo: Jogo,
    pub numeros: Option<Vec<u8>>,
}

impl Cli {
    pub fn parse() -> Result<Self> {
        let acao_id = "acao";
        let jogo_id = "jogo";
        let aposta_id = "aposta";

        let acao_arg = Arg::new(acao_id)
            .help("<consultar, analisar>")
            .required(true)
            .index(1);

        let jogo_arg = Arg::new(jogo_id)
            .help("<lotofacil, lotomania, megasena>")
            .required(true)
            .index(2);

        let aposta_arg = Arg::new(aposta_id)
            .help("Numeros da aposta. Ex. 04-08-15-16-23-42")
            .required(false)
            .index(3);

        let matches = Command::new("loto")
			.version("0.1")
			.author("cleissonalves.dev@outlook.com")
			.about("Aplicacao CLI para consulta e analise de resultados de sorteios das loterias da Caixa.")
			.arg(acao_arg)
			.arg(jogo_arg)
			.arg(aposta_arg)
			.after_help("EXEMPLOS:\nloto consultar megasena\nloto consultar megasena 04-08-15-16-23-42\nloto consultar megasena < input.txt > output.txt\n\n*Os numeros de uma aposta podem ser separados por espaco, ',' ou '-'.\n*Arquivos de input devem conter em cada linha uma sequencia de numeros referentes a uma aposta.")
			.get_matches();

        let action_str = matches
            .get_one::<String>(acao_id)
            .cloned()
            .unwrap_or("consultar".to_string());
        let jogo_str = matches
            .get_one::<String>(jogo_id)
            .cloned()
            .unwrap_or("megasena".to_string());
        let numbers_str = matches.get_one::<String>(aposta_id).cloned();

        let action = Acao::from_str(&action_str, true)
            .map_err(|_| anyhow!("Acao invalida. ({})", action_str))?;

        let jogo = Jogo::from_str(&jogo_str, true)
            .map_err(|_| anyhow!("Jogo invalido. ({})", jogo_str))?;

        if let Some(numbers_str) = numbers_str {
            let numeros = parsers::vec_u8_from_str(&numbers_str)?;

            jogo.validar(&numeros)?;

            return Ok(Cli {
                action,
                jogo,
                numeros: Some(numeros),
            });
        }

        Ok(Cli {
            action,
            jogo,
            numeros: None,
        })
    }
}

pub fn run() -> Result<()> {
    let args = Cli::parse()?;

    if let Some(a) = args.numeros {
        // the numbers are passed as argument
        match args.action {
            Acao::Consultar => consultar_numeros(&args.jogo, &vec![a.as_slice()])?,
            Acao::Analisar => todo!(),
        }
    } else if atty::isnt(atty::Stream::Stdin) {
        // the stdin does not come directly from the terminal, but a file redirection (<)
        let numbers_from_file = parsers::vec_u8_from_buffer()?;
        let numbers_refs: Vec<&[u8]> = numbers_from_file.iter().map(|v| v.as_slice()).collect();
        match args.action {
            Acao::Consultar => consultar_numeros(&args.jogo, &numbers_refs)?,
            Acao::Analisar => todo!(),
        }
    } else {
        // there is no numbers input
        match args.action {
            Acao::Consultar => consultar(&args.jogo)?,
            Acao::Analisar => todo!(),
        }
    }

    Ok(())
}
