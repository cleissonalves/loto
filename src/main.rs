use loto::{cli_derive, commands, data::Jogo};

#[allow(dead_code)]
fn tmp_test() {
    let mut apostas: Vec<&[u8]> = Vec::new();
    apostas.push(&[01, 12, 34, 45, 50, 55]);
    apostas.push(&[01, 13, 38, 40, 50, 57]);
    apostas.push(&[01, 12, 34, 40, 50, 60]);
    apostas.push(&[38, 52, 4, 45, 02, 55]);

    _ = commands::consultar_numeros(&Jogo::Megasena, &apostas);
}

fn main() {
    let application = cli_derive::run();
    // let application = deprecated::cli_builder::run();

    if let Err(e) = application {
        eprintln!("{e}");
    }
}
