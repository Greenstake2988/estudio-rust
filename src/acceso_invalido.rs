use std::io;

fn main() {
    let a = [1, 2, 3, 4, 5];

    println!("Por favor dame el index del arreglo, del 0 al 4 o ERROR");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Fallo al leer la linea");

    let index: usize = index
        .trim()
        .parse()
        .expect("EL index no es un numero");

    let element = a[index];

    println!("El valor del del elemento en el index {index} es: {element}");
}
