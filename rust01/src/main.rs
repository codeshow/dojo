#![allow(dead_code)]

#[derive(Debug, PartialEq, Eq)]
enum Player {
    Pedra,
    Papel,
    Tesoura
}

#[derive(Debug, PartialEq, Eq)]
enum Resultado {
    Ganhou(Player),
    Empatou
}

fn jokenpo(p1: Player, p2: Player) -> Resultado {
    match (p1, p2) {
        (Player::Pedra, Player::Papel) | (Player::Papel, Player::Pedra) => Resultado::Ganhou(Player::Papel),
        (Player::Pedra, Player::Tesoura) | (Player::Tesoura, Player::Pedra) => Resultado::Ganhou(Player::Pedra),
        (Player::Tesoura, Player::Papel) | (Player::Papel, Player::Tesoura) => Resultado::Ganhou(Player::Tesoura),
        (_, _) => Resultado::Empatou
    }
}

#[test]
fn test_pedra_papel() {
    assert_eq!(jokenpo(Player::Pedra, Player::Pedra), Resultado::Empatou);


    assert_eq!(jokenpo(Player::Pedra,  Player::Pedra), Resultado::Empatou);
    assert_eq!(jokenpo(Player::Tesoura,  Player::Tesoura), Resultado::Empatou);
    assert_eq!(jokenpo(Player::Papel,  Player::Papel), Resultado::Empatou);

    assert_eq!(jokenpo(Player::Pedra,  Player::Papel), Resultado::Ganhou(Player::Papel));
    assert_eq!(jokenpo(Player::Papel,  Player::Pedra), Resultado::Ganhou(Player::Papel));

    assert_eq!(jokenpo(Player::Pedra,  Player::Tesoura), Resultado::Ganhou(Player::Pedra));
    assert_eq!(jokenpo(Player::Tesoura,  Player::Pedra), Resultado::Ganhou(Player::Pedra));

    assert_eq!(jokenpo(Player::Tesoura, Player::Papel), Resultado::Ganhou(Player::Tesoura));
    assert_eq!(jokenpo(Player::Papel, Player::Tesoura), Resultado::Ganhou(Player::Tesoura));
}


fn main() {}
