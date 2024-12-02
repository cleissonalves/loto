use std::collections::HashMap;

use crate::{
    client,
    data::{Jogo, Sorteio},
    sys_tmp,
    util::{self, formatar_dinheiro},
};
use anyhow::Result;
use colored::{ColoredString, Colorize};

pub fn consultar_numeros(jogo: &Jogo, apostas: &Vec<&[u8]>) -> Result<()> {
    let sorteio = client::fetch_latest(&jogo)?;

    let show_aposta_id = apostas.len() > 1;

    for (index, aposta) in apostas.iter().enumerate() {
        let (acertos, output) = verificar_aposta(aposta, &sorteio);

        if show_aposta_id {
            print!("Aposta #{:2}:  ", index + 1);
        } else {
            print!("Aposta:  ");
        }
        for s in output {
            print!(" {}", s);
        }
        let premiado = jogo.get_prize_matches().contains(&acertos);
        if premiado {
            let out = format!(" -> Acertos: {} APOSTA PREMIADA! 🎉", acertos).to_string();
            println!("{}", out.bright_green());
        } else {
            println!(" -> Acertos: {}", acertos);
        }
    }

    println!("=============================================");

    print_sorteio_info(&sorteio);

    Ok(())
}

pub fn consultar(jogo: &Jogo) -> Result<()> {
    let sorteio = client::fetch_latest(&jogo)?;

    print_sorteio_info(&sorteio);

    Ok(())
}

fn print_sorteio_info(sorteio: &Sorteio) {
    let mut output = "Sorteio: ".to_string();
    for n in sorteio.numeros.iter() {
        let n_string = format!(" {:02}", n).to_string();
        output.push_str(&n_string);
    }
    println!("{}", output.yellow());
    println!("Data: {}", sorteio.data);
    println!("Concurso: {}", sorteio.concurso);
    println!("Premiacoes: {}", sorteio.premiacoes());
    if sorteio.acumulou {
        let output = format!(
            "Acumulou ~ {}",
            util::formatar_dinheiro(&sorteio.valor_acumulado)
        )
        .to_string();
        println!("{}", output.bright_blue());
    }
    println!("Prox. Sorteio: {}", sorteio.data_prox_sorteiro);
}

fn verificar_aposta(aposta: &[u8], sorteio: &Sorteio) -> (u8, Vec<ColoredString>) {
    let mut acertos = 0;
    let mut output: Vec<ColoredString> = Vec::new();
    let mut aposta = aposta.to_vec();
    aposta.sort();

    for n in aposta {
        let n_string = format!("{:02}", n).to_string();
        if sorteio.numeros.contains(&n) {
            acertos += 1;
            output.push(n_string.bright_green());
        } else {
            output.push(n_string.white());
        }
    }

    (acertos, output)
}

fn analisar_sorteio(aposta: &[u8], sorteio: &Sorteio) -> (u8, Vec<ColoredString>) {
    let mut matches = 0;
    let mut output: Vec<ColoredString> = Vec::new();
    let mut aposta = aposta.to_vec();
    aposta.sort();

    for n in &sorteio.numeros {
        let n_string = format!("{:02}", n).to_string();
        if aposta.contains(&n) {
            matches += 1;
            output.push(n_string.bright_green());
        } else {
            output.push(n_string.white());
        }
    }

    let match_prize_str = format!("{} ", matches);
    let match_prize = &sorteio
        .premiacoes
        .iter()
        .filter(|p| p.descricao.starts_with(&match_prize_str))
        .map(|p| formatar_dinheiro(&p.premio))
        .collect::<Vec<String>>()
        .join(", ");

    let data = &sorteio.data;
    let concurso = &sorteio.concurso;

    output.push(format!(" ({}, {})", data, concurso).white());
    output.push(format!(" {}", match_prize).bright_green());

    (matches, output)
}

fn analisar_sorteios(jogo: &Jogo, aposta: &[u8], sorteios: &Vec<Sorteio>) {
    let matches: Vec<(u8, Vec<ColoredString>)> = sorteios
        .iter()
        .map(|s| analisar_sorteio(&aposta, &s))
        .collect();

    let prize_matches = jogo.get_prize_matches();
    let mut matches: Vec<&(u8, Vec<ColoredString>)> = matches
        .iter()
        .filter(|m| prize_matches.contains(&m.0))
        .collect();

    matches.sort_by(|a, b| b.0.cmp(&a.0));

    for n in jogo.get_prize_matches() {
        let sorteios_com_n_acertos: Vec<Vec<ColoredString>> = matches
            .iter()
            .filter(|s| s.0 == n)
            .map(|s| s.1.clone())
            .collect();

        if sorteios_com_n_acertos.len() == 0 {
            continue;
        }

        println!("[{} acertos: {} apostas]", n, sorteios_com_n_acertos.len());

        for s in sorteios_com_n_acertos {
            for m in s {
                print!("{} ", m);
            }
            print!("\n");
        }
        print!("\n");
    }

    print!("Total de premiacoes: {}\n", matches.len());
}

pub fn historico(jogo: &Jogo, quantidade: usize, atualizar: bool) -> Result<()> {
    let mut sorteios: Vec<Sorteio>;

    let is_update_needed = sys_tmp::is_update_needed(&jogo);

    if atualizar || is_update_needed.unwrap_or(true) {
        let content = client::fetch_all(&jogo)?;
        sys_tmp::save_json(&content)?;
    }

    if let Ok(value) = sys_tmp::load_json(&jogo) {
        sorteios = value;
    } else {
        let content = client::fetch_all(&jogo)?;
        sys_tmp::save_json(&content)?;
        sorteios = sys_tmp::load_json(&jogo)?;
    }

    if quantidade > 0 {
        sorteios = sorteios.into_iter().take(quantidade).collect();
    }

    sorteios.reverse();
    imprimir_sorteios(&sorteios);
    imprimir_numeros_mais_sorteados(&sorteios, 10);

    println!("\nPara mais resultados adicione '--quantidade <valor>' ou '-q <valor>'.");
    // println!("Para atualizar os dados use '--atualizar' ou '-a'.\n");

    Ok(())
}

fn imprimir_sorteios(sorteios: &Vec<Sorteio>) {
    for s in sorteios {
        let nums_str = s
            .numeros
            .iter()
            .map(|n| format!("{:02}", n).to_string())
            .collect::<Vec<String>>()
            .join(" ");

        let acumulou = if s.acumulou {
            format!(
                "(Acumulou ~ {})",
                util::formatar_dinheiro(&s.valor_acumulado)
            )
            .to_string()
        } else {
            "💰️".to_string()
        };

        println!(
            "{} ({}) : {} {}\n  {} {}\n",
            s.data,
            s.concurso,
            nums_str,
            acumulou.bright_blue(),
            "Premios: ".bright_green(),
            s.premiacoes()
        )
    }
}

pub fn get_numeros_e_ocorrencias(sorteios: &Vec<Sorteio>) -> Vec<(u8, u32)> {
    let mut ocorrencias: HashMap<u8, u32> = HashMap::new();

    for s in sorteios {
        for &n in &s.numeros {
            *ocorrencias.entry(n).or_insert(0) += 1;
        }
    }

    let mut ocorrencias: Vec<(u8, u32)> = ocorrencias.into_iter().collect();

    // Sort by the u32 (second value) in decrescent order
    ocorrencias.sort_by(|a, b| b.1.cmp(&a.1));

    ocorrencias
}

pub fn imprimir_numeros_mais_sorteados(sorteios: &Vec<Sorteio>, quantidade: usize) {
    let ocorrencias = get_numeros_e_ocorrencias(&sorteios);
    let ocorrencias = ocorrencias.iter().take(quantidade);

    let mut output = Vec::new();
    for (number, count) in ocorrencias {
        output.push(format!("{}", number).yellow());
        output.push(format!("(x{}) ", count).white());
    }

    print!("Numeros Mais Sorteados: ");

    for cs in output {
        print!("{}", cs);
    }

    println!();
}

pub fn analisar(jogo: &Jogo, numeros: &[u8], quantidade: usize, atualizar: bool) -> Result<()> {
    let mut sorteios: Vec<Sorteio>;

    let is_update_needed = sys_tmp::is_update_needed(&jogo);

    if atualizar || is_update_needed.unwrap_or(true) {
        let content = client::fetch_all(&jogo)?;
        sys_tmp::save_json(&content)?;
    }

    if let Ok(value) = sys_tmp::load_json(&jogo) {
        sorteios = value;
    } else {
        let content = client::fetch_all(&jogo)?;
        sys_tmp::save_json(&content)?;
        sorteios = sys_tmp::load_json(&jogo)?;
    }

    if quantidade > 0 {
        sorteios = sorteios.into_iter().take(quantidade).collect();
    }

    sorteios.reverse();

    analisar_sorteios(&jogo, &numeros, &sorteios);

    println!("\nPara mais resultados, adicione a opcao '--quantidade <valor>' ou '-q <valor>'.");
    println!("Para atualizar os dados use '--atualizar' ou '-a'.\n");

    Ok(())
}
