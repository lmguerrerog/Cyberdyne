use std::io;
pub mod functions_amartinc;
pub mod connection_amartinc;

pub fn menu_aplicacion() {
    // Obtener la conexión de la base de datos en memoria de sqlite
    let connection = connection_amartinc::getConnection();

    // Crear la tabla de usuarios
    functions_amartinc::createTable(&connection);

    // Crear el menú de la aplicación
    let mut input = String::new();

    loop {

        println!("============================================================");
        println!("Menú aplicación Cyberdine");
        println!("============================================================");
        println!("Seleccione una opción:");
        println!("1) Consulta usuarios");
        println!("2) Alta de usuario");
        println!("3) Editar usuario");
        println!("4) Borrar usuario");
        println!("5) Salir");
        println!("============================================================");

        input.clear();
        io::stdin().read_line(&mut input).expect("Error al leer la línea");

        let input: i32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match input {
            1 => functions_amartinc::select_user(&connection),
            2 => functions_amartinc::add_user(&connection).and_then(|s| Ok(println!("{}", s))).unwrap_or_else(|e| eprintln!("{}", e)),
            3 => functions_amartinc::edit_user(&connection).and_then(|s| Ok(println!("{}", s))).unwrap_or_else(|e| eprintln!("{}", e)),
            4 => functions_amartinc::delete_user(&connection).and_then(|s| Ok(println!("{}", s))).unwrap_or_else(|e| eprintln!("{}", e)),
            5 => break,
            _ => println!("Opción no válida"),
        }
    }

}