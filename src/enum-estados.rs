#[derive(Debug)] // so we can inspect the state in a minute
enum UsEstado {
    Alabama,
    Alaska,
    // --snip--
}


enum Moneda {
    Uno,
    Cinco,
    Diez,
    Veinte(UsEstado),
}


fn valor_en_pesos(moneda: Moneda) -> u8 {
    match moneda {
        Moneda::Uno => 1,
        Moneda::Cinco => 5,
        Moneda::Diez => 10,
        Moneda::Veinte(estado) => {
            println!("Moneda de 20 del estado de {:?}", estado);
            20
        }
    }
}

fn mas_uno(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn main(){
    valor_en_pesos(Moneda::Veinte(UsEstado::Alaska));
    let cinco = Some(5);
    let seis = mas_uno(cinco);
    let none = mas_uno(None);
}


