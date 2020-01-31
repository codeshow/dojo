#![allow(dead_code)]

fn jokenpo(p1: &str, p2: &str) -> &str {
    match (p1.as_ref(), p2.as_ref()) {
        ("pedra", "papel") | ("papel", "pedra") => "papel".to_string(),
        ("pedra", "tesoura") | ("tesoura", "pedra") => "pedra".to_string(),
        ("tesoura", "papel") | ("papel", "tesoura") => "tesoura".to_string(),
        (_, _) => "empatou".to_string()
    }
}

#[test]
fn test_pedra_papel() {
    assert_eq!(jokenpo("pedra",  "pedra"), "empatou");


    assert_eq!(jokenpo("pedra".to_string(),  "pedra".to_string()), "empatou".to_string());
    assert_eq!(jokenpo("tesoura".to_string(),  "tesoura".to_string()), "empatou".to_string());
    assert_eq!(jokenpo("papel".to_string(),  "papel".to_string()), "empatou".to_string());

    assert_eq!(jokenpo("pedra".to_string(),  "papel".to_string()), "papel".to_string());
    assert_eq!(jokenpo("papel".to_string(),  "pedra".to_string()), "papel".to_string());

    assert_eq!(jokenpo("pedra".to_string(),  "tesoura".to_string()), "pedra".to_string());
    assert_eq!(jokenpo("tesoura".to_string(),  "pedra".to_string()), "pedra".to_string());

    assert_eq!(jokenpo("tesoura".to_string(), "papel".to_string()), "tesoura".to_string());
    assert_eq!(jokenpo("papel".to_string(), "tesoura".to_string()), "tesoura".to_string());
}


fn main() {}
