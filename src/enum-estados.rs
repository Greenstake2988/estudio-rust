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


fn main(){
    valor_en_pesos(Moneda::Veinte(UsEstado::Alaska));
}
