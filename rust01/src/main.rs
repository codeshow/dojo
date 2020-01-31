#![allow(dead_code)]

fn jokenpo(p1: &str, p2: &str) -> &'static str {

    match (p1, p2) {
        ("pedra", "papel") | ("papel", "pedra") => "papel",
        ("pedra", "tesoura") | ("tesoura", "pedra") => "pedra",
        ("tesoura", "papel") | ("papel", "tesoura") => "tesoura",
        (_, _) => "empatou"
    }
}

#[test]
fn test_pedra_papel() {
    assert_eq!(jokenpo("pedra",  "pedra"), "empatou");


    assert_eq!(jokenpo("pedra",  "pedra"), "empatou");
    assert_eq!(jokenpo("tesoura",  "tesoura"), "empatou");
    assert_eq!(jokenpo("papel",  "papel"), "empatou");

    assert_eq!(jokenpo("pedra",  "papel"), "papel");
    assert_eq!(jokenpo("papel",  "pedra"), "papel");

    assert_eq!(jokenpo("pedra",  "tesoura"), "pedra");
    assert_eq!(jokenpo("tesoura",  "pedra"), "pedra");

    assert_eq!(jokenpo("tesoura", "papel"), "tesoura");
    assert_eq!(jokenpo("papel", "tesoura"), "tesoura");
}


fn main() {}
