use loto::{cli_derive, commands, data::Jogo};

#[allow(dead_code)]
fn tmp_test() {
    let apostas: Vec<&[u8]> = vec![
        &[01, 12, 34, 45, 50, 55],
        &[01, 13, 38, 40, 50, 57],
        &[01, 12, 34, 40, 50, 60],
        &[38, 52, 4, 45, 02, 55],
    ];

    _ = commands::consultar_numeros(&Jogo::Megasena, &apostas);
}

fn main() {
    let application = cli_derive::run();
    // let application = deprecated::cli_builder::run();

    if let Err(e) = application {
        eprintln!("{e}");
    }
}
