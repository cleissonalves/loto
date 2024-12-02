use crate::commands;
use crate::{data, parsers};
use anyhow::{Ok, Result};
use clap::{command, Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "loto",
    about = "Aplicacao CLI para consulta e analise de resultados de sorteios das loterias da Caixa.\nAPI: https://github.com/guto-alves/loterias-api",
    author = "cleissonalves.dev@outlook.com",
    after_help = "EXEMPLOS:\nloto consultar megasena\nloto consultar megasena 04-08-15-16-23-42\nloto consultar megasena < input.txt\nloto consultar megasena < input.txt > output.txt\n\nloto historico megasena\nloto historico megasena > output.txt\nloto historico megasena --quantidate 50\n\nloto analisar megasena 04-08-15-16-23-42\nloto analisar megasena 04-08-15-16-23-42 > output.txt\nloto analisar megasena 04-08-15-16-23-42 -q 50\n\nPara ajuda em cada tipo de operacao, use --help.\nloto consultar --help\nloto historico --help\nloto analisar --help\n\n* Os numeros de uma aposta podem ser separados por ',' ou '-'.\n* '< input.txt' carrega apostas para o comando consultar.\n* Arquivos de input devem conter em cada linha uma sequencia de numeros referentes a uma aposta.\n* '> output.txt' exporta o resultado de qualquer comando.\n* Atualizacao de dados feita a cada 24h."
)]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    #[clap(about = "Consultar ultimo sorteio e comparar com aposta(s).")]
    Consultar {
        jogo: data::Jogo,
        #[clap(help = "Opcional: Numeros de uma aposta. Ex. 04-08-15-16-23-42")]
        numeros: Option<String>,
    },
    #[clap(about = "Mostrar historico de sorteios.")]
    Historico {
        jogo: data::Jogo,
        #[clap(
            long,
            short,
            default_value_t = 20,
            help = "Valor '0' inclui todos os sorteios"
        )]
        quantidade: usize,
        // with default_value_t, there is no need for Option
        // #[clap(long, short, default_value_t = false, help = "Automatico a cada 24h")]
        // atualizar: bool,
    },
    #[clap(about = "Analisar uma aposta em sorteios passados.")]
    Analisar {
        jogo: data::Jogo,
        #[clap(help = "Numeros de uma aposta. Ex. 04-08-15-16-23-42")]
        numeros: String,
        #[clap(
            long,
            short,
            default_value_t = 0,
            help = "Valor '0' inclui todos os sorteios"
        )]
        quantidade: usize,
        // with default_value_t, there is no need for Option
        // #[clap(long, short, default_value_t = false, help = "Automatico a cada 24h")]
        // atualizar: bool,
    },
}

pub fn run() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Consultar { jogo, numeros } => consultar(&jogo, &numeros)?,
        Commands::Historico { jogo, quantidade } => commands::historico(&jogo, quantidade)?,
        Commands::Analisar {
            jogo,
            numeros,
            quantidade,
        } => analisar(&jogo, &numeros, quantidade)?,
    }

    Ok(())
}

fn consultar(jogo: &data::Jogo, numeros: &Option<String>) -> Result<()> {
    match numeros {
        Some(nums) => {
            let numeros = parsers::vec_u8_from_str(&nums)?;
            jogo.validar(&numeros)?;
            commands::consultar_numeros(&jogo, &vec![numeros.as_slice()])?;
        }
        None => {
            if atty::isnt(atty::Stream::Stdin) {
                // the stdin does not come directly from the terminal, but a file redirection (<)
                let numbers_from_file = parsers::vec_u8_from_buffer()?;
                let numbers_refs: Vec<&[u8]> =
                    numbers_from_file.iter().map(|v| v.as_slice()).collect();
                commands::consultar_numeros(&jogo, &numbers_refs)?;
            } else {
                commands::consultar(&jogo)?;
            }
        }
    }
    Ok(())
}

fn analisar(jogo: &data::Jogo, numeros: &String, quantidade: usize) -> Result<()> {
    let numeros = parsers::vec_u8_from_str(&numeros)?;
    jogo.validar(&numeros)?;
    commands::analisar(&jogo, numeros.as_slice(), quantidade)?;
    Ok(())
}
