
struct Color(i32, i32, i32);
struct Punto(i32, i32, i32);


struct Usuario {
    _activo: bool,
    nombre_usuario: String,
    _email: String,
    _contador_de_sesion: u64,
}

fn main() {
    let usuario1 = creador_usuario(String::from("oscar.va"),
                                   String::from("0skr.vazquez@.com"));
    
    let usuario2 = Usuario {
        _activo: false,
        ..usuario1
    };
    
    // Error el espacio en memorio de usuario1.nombre_usuario se movio
    // a usuario2.nombre_usuario
    //println!("Nombre de Usuario 1: {}", usuario1.nombre_usuario);
    
    println!("Nombre de Usuario 2: {}", usuario2.nombre_usuario);

    let _negro = Color(0, 0, 0);
    let _origen = Punto(0, 0, 0);

}




fn creador_usuario(nombre_usuario: String, _email: String) -> Usuario {
    Usuario {
        _activo: true,
        nombre_usuario,
        _email,
        _contador_de_sesion: 1,
    }
}
