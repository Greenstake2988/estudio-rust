fn main() {
    imprimir_medidas_etiquetadas(5,'h');
    
    let x = cinco();

    let x = mas_uno(x);

    println!("El valor de x es: {x}");
}

fn imprimir_medidas_etiquetadas(valor: i32, etiqueta_medida: char) {
    println!("La medicion es: {valor}{etiqueta_medida}");
}

fn cinco() -> i32 {
    5
}

fn mas_uno(x: i32) -> i32 {
    x + 1
}
