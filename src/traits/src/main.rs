
pub trait Resumen {
    fn resumir(&self) -> String;
}

pub struct NuevoArticulo {
    pub encabezado: String,
    pub ubicación: String,
    pub autor: String,
    pub contenido: String,
}

impl Resumen for NuevoArticulo {
    fn resumir(&self) -> String {
            format!("{}, por {} ({})", self.encabezado, self.autor, self.ubicación)
    }
}

pub struct Tweet {
    pub nombre_usuario: String,
    pub contenido: String,
    pub respuesta: bool,
    pub retweet: bool,
}

impl Resumen for Tweet {
    fn resumir(&self) -> String {
        format!("{}: {}", self.nombre_usuario, self.contenido)
    }
}

fn main() {

    let tweet = Tweet {
        nombre_usuario: String::from("libros_digitales_de_caballos"),
        contenido: String::from(
            "SI ya conoces ala gentes",
        ),
        respuesta: false,
        retweet: false,
    };

    println!("1 nuevo tweet: {}", tweet.resumir());
}
