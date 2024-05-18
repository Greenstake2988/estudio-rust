// Esta esctrucutra no se puede imprmir
struct NoSePuedeImprimir(i32);

// El derive atributo crea automaticamente la implementacion
// requerida para hacer que struct sea imprimipmen con fmt::Debug
#[derive(Debug)]
struct DebugImprimible(i32);

// Toda la biblioteca std es automaticamente imprimible con {:?}


// Derive  fmt::Debug implementaicon para Structure,
// Structure es is a Structure que contiene un i32
#[derive(Debug)]
struct Structure(i32);

// Pon a `Structure` dentro de una struct `Deep`.
// Volviendola imprimible tambien.
#[derive(Debug)]
struct Deep(Structure);

fn main() {
    //imprimit con {:?} es similar a {}
    println!("{:?} meses en un año", 12);
    println!("{1:?} {0:?} es el {actor:?} nombre.",
            "Slater",
            "Christian",
            actor="actors's");
    
    // `Structure` se puede imprimir!
    println!("Ahora {:?} imprimira", Structure(3));
    
    // El problema es con `derive` es que no tienes control sobre como imprimible
    // que tal que solo quiero que muestr el 7
    println!("Ahora {:?} imprimira", Deep(Structure(7)));
}
