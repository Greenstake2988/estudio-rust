#![allow(unused)]
fn main() {
    // Importamos via `use` the `fmt` module para hacerlo accesible
    use std::fmt;

    // Definimos la struct por el cual fmt::Display sera implementado
    // es to es una tupla llama Estructura que contiene un i32
    struct Estructura(i32);

    // Para usar {} como marcador, el `trait` fmt::Display tiene
    // que ser implementado manualmente para este tipo
    impl fmt::Display for Estructura {
        // Este trait requiere fmt con la signatura exacta
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            //
            write!(f, "{}", self.0)
        }
    }
}
