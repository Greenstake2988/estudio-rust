fn el_mas_grande(lista: &[i32]) -> &i32 {
    let mut el_mas_grande = &lista[0];

    for item in lista {
        if item > el_mas_grande {
            el_mas_grande = item;
        }
    }

    el_mas_grande
}



fn main() {
    let lista_de_numeros = vec![34, 50, 25, 100, 65];

    let resultado = el_mas_grande(&lista_de_numeros);
    println!("El numero mas grande es {}", resultado);

    let lista_de_numeros = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let resultado = el_mas_grande(&lista_de_numeros);
    println!("El numero mas grande es {}", resultado);
}
