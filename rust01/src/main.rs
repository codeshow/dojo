#![allow(dead_code)]

#[derive(Debug, PartialEq)]
enum Object {
    Pedra,
    Papel,
    Tesoura
}

impl Object {
    fn ganha_de(&self) -> Object {
        match self {
            Object::Pedra => Object::Tesoura,
            Object::Tesoura => Object::Papel,
            Object::Papel => Object::Pedra
        }
    }
}

fn jokenpo<'a>(p1: &'a Object, p2: &'a Object) -> Option<&'a Object> {
   if p1 == p2 {
       None
   } else if &p1.ganha_de() == p2 {
       Some(&p1)
   } else {
       Some(&p2)
   }
}

#[test]
fn test_pedra_papel() {  

    let pedra = Object::Pedra;
    let papel = Object::Papel;
    let tesoura = Object::Tesoura;

    assert_eq!(jokenpo(&pedra, &papel), Some(&papel));
    assert_eq!(jokenpo(&pedra, &tesoura), Some(&pedra));
    assert_eq!(jokenpo(&pedra, &pedra), None);

    assert_eq!(jokenpo(&pedra, &tesoura), Some(&pedra));
    assert_eq!(jokenpo(&tesoura, &pedra), Some(&pedra));


    assert_eq!(jokenpo(&pedra,  &pedra), None);
    assert_eq!(jokenpo(&tesoura,  &tesoura), None);
    assert_eq!(jokenpo(&papel,  &papel), None);

    assert_eq!(jokenpo(&pedra,  &papel), Some(&papel));
    assert_eq!(jokenpo(&papel,  &pedra), Some(&papel));

    assert_eq!(jokenpo(&pedra,  &tesoura), Some(&pedra));
    assert_eq!(jokenpo(&tesoura,  &pedra), Some(&pedra));

    assert_eq!(jokenpo(&tesoura, &papel), Some(&tesoura));
    assert_eq!(jokenpo(&papel, &tesoura), Some(&tesoura));
}


fn main() {}
