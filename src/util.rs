pub fn formatar_dinheiro(valor: &f32) -> String {
    if *valor > 1_000_000.0 {
        format!("R$ {:.0} milhoes", valor / 1_000_000.0)
    } else if *valor > 1_000.0 {
        format!("R$ {:.0} mil", valor / 1_000.0)
    } else {
        format!("R$ {:.0}", valor)
    }
}
