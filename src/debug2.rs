/* Entonces fmt::Debug hacer que las cosas se puedan imprimir
 pero sacrificas elegancia Rust tambine provee "una impresion bonita"
 con {:#?}*/

#[derive(Debug)]
struct Persona<'a> {
    nombre: &'a str,
    edad: u8
}

fn main() {
    let nombre = "Pedro";
    let edad = 27;
    let pedro = Persona {nombre, edad};

    // Impresion Bonita
    println!("{:#?}", pedro);
}

// Puedes implementar tu prpopio impresion de debug
