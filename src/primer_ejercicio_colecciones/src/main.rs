use std::collections::HashMap;

fn main() {
    // Creamos la lista con numeros
    let mut lista = [3, 2, 6, 7, 5, 1, 10, 4, 2];

    println!("La media de la lista es {}", media(&mut lista));
    println!("La moda de la lista es {}", moda(& lista));
}

fn media(numeros: &mut [i32]) -> i32 {
    numeros.sort();
    let media = numeros.len() / 2;
    numeros[media]
}

fn moda(numeros: &[i32]) -> i32 {
    let mut ocurrencias = HashMap::new();

    for &num in numeros {
        let cuantas = ocurrencias.entry(num).or_insert(0);
        *cuantas += 1;
    }

    println!("{:?}", ocurrencias);
    45
}
