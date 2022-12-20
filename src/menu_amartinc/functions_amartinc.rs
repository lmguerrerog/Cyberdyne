use sqlite::{ColumnIndex, Error as SqliteError};
use sqlite::State;
use prettytable::{Table, Row, Cell, row};
use std::io;
use regex::Regex;
use crate::menu_amartinc::functions_amartinc::usuarios_amartinc::Usuario;

pub mod usuarios_amartinc;

pub fn createTable(conn: &sqlite::Connection) {

    conn.execute("CREATE TABLE IF NOT EXISTS usuarios (
            id INTEGER PRIMARY KEY,
            apellidos VARCHAR NOT NULL,
            nombre VARCHAR NOT NULL,
            sexo VARCHAR NOT NULL,
            nacionalidad VARCHAR NOT NULL,
            f_nacimiento DATE NOT NULL)").unwrap();

}

pub fn select_user(conn: &sqlite::Connection) {

    // Create the table
    let mut table = Table::new();
    table.add_row(row!["ID", "Apellidos", "Nombre", "Sexo", "Nacionalidad", "Fecha nacimiento"]);

    let select = "SELECT * FROM usuarios";
    let mut statement = conn.prepare(select).unwrap();

    println!("Usuarios encontrados:");
    let mut contador = 0;
    while let Ok(State::Row) = statement.next() {
        /*println!("* Usuario con id = {}", statement.read::<i64, _>("id").unwrap());
        println!("- nombre = {}", statement.read::<String, _>("nombre").unwrap());
        println!("- apellidos = {}", statement.read::<String,_>("apellidos").unwrap());
        println!("- sexo = {}", statement.read::<String, _>("sexo").unwrap());
        println!("- nacionalidad = {}", statement.read::<String, _>("nacionalidad").unwrap());
        println!("- nacimiento = {}", statement.read::<String, _>("f_nacimiento").unwrap());*/
        table.add_row(row![
            statement.read::<i64, _>("id").unwrap(),
            statement.read::<String, _>("apellidos").unwrap(),
            statement.read::<String, _>("nombre").unwrap(),
            statement.read::<String, _>("sexo").unwrap(),
            statement.read::<String, _>("nacionalidad").unwrap(),
            statement.read::<String, _>("f_nacimiento").unwrap()
        ]);

        contador = contador+1;
    }
    table.printstd();

    if contador == 1 {
        println!("Se ha encontrado sólo {} usuario", contador);
    } else {
        println!("Se han encontrado {} usuarios", contador);
    }

}

pub fn add_user(conn: &sqlite::Connection) -> Result<String, &str> {
    let mut usuario = Usuario {
        id: 0,
        apellidos: String::new(),
        nombre: String::new(),
        sexo: String::new(),
        nacionalidad:String::new(),
        nacimiento: String::new(),
    };

    println!("Introduzca los datos del nuevo usuario:");
    println!("- Apellidos:");
    let mut user_apellidos = String::new();
    io::stdin().read_line(&mut user_apellidos).expect("Error al leer los apellidos.");
    usuario.apellidos = user_apellidos.replace("\n","");
    if usuario.apellidos.trim().is_empty() {
        return Err("Es obligatorio indicar los apellidos.");
    }

    println!("- Nombre:");
    let mut user_nombre = String::new();
    io::stdin().read_line(&mut user_nombre).expect("Error al leer el nombre.");
    usuario.nombre = user_nombre.replace("\n","");
    if usuario.nombre.trim().is_empty() {
        return Err("Es obligatorio indicar el nombre.");
    }
    println!("- Sexo:");
    let mut user_sexo = String::new();
    io::stdin().read_line(&mut user_sexo).expect("Error al leer el sexo.");
    usuario.sexo = user_sexo.replace("\n","");
    if usuario.sexo.trim().is_empty() {
        return Err("Es obligatorio indicar el sexo.");
    } else {

        if usuario.sexo.trim().len() == 1 {
            let re = Regex::new(r"^[MF]$").unwrap();

            if !re.is_match(usuario.sexo.trim()) {
                return Err("El sexo debe ser F o M.");
            }
        } else {
            return Err("El sexo debe ser un sólo carácter, F o M.");
        }
    }
    println!("- Nacionalidad:");
    let mut user_nacionalidad = String::new();
    io::stdin().read_line(&mut user_nacionalidad).expect("Error al leer la nacionalidad.");
    usuario.nacionalidad = user_nacionalidad.replace("\n","");
    if usuario.nacionalidad.trim().is_empty() {
        return Err("Es obligatorio indicar la nacionalidad.");
    }
    println!("- Fecha de nacimiento:");
    let mut user_nacimiento = String::new();
    io::stdin().read_line(&mut user_nacimiento).expect("Error al leer la fecha de nacimiento.");
    usuario.nacimiento = user_nacimiento.replace("\n","");
    if usuario.nacimiento.trim().is_empty() {
        return Err("Es obligatorio indicar la fecha de nacimiento.");
    } else {
        let re = Regex::new(r"^\d{2}/\d{2}/\d{4}$").unwrap();

        if !re.is_match(usuario.nacimiento.trim()) {
            return Err("El fecha de nacimiento debe tener el formato dd/mm/aaaa.");
        }
    }
   // usuario = validate(usuario);

    usuario.insert_into_db(&conn).unwrap();

    Ok("Usuario insertado correctamente.".to_string())
}

pub fn edit_user(conn: &sqlite::Connection) -> Result<String, &str> {
    let mut usuario = Usuario {
        id: 0,
        apellidos: String::new(),
        nombre: String::new(),
        sexo: String::new(),
        nacionalidad:String::new(),
        nacimiento: String::new(),
    };

    println!("Introduzca el id del usuario a editar:");
    let mut user_id = String::new();
    io::stdin().read_line(&mut user_id).expect("Error al leer el id.");
    usuario.id = user_id.replace("\n","").parse::<i32>().unwrap();
    if usuario.id == 0 {
        return Err("Es obligatorio indicar el id.");
    } else {
        usuario = usuario.select_by_id(&conn).unwrap();

        // Create the table
        let mut table = Table::new();
        table.add_row(row!["ID", "Apellidos", "Nombre", "Sexo", "Nacionalidad", "Fecha nacimiento"]);

        table.add_row(row![
            usuario.id,
            usuario.apellidos,
            usuario.nombre,
            usuario.sexo,
            usuario.nacionalidad,
            usuario.nacimiento
        ]);
        table.printstd();

        println!("Introduzca los nuevos datos del usuario:");

        println!("¿Quieres actualizar los apellidos? (s/n)");
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let input = input.trim();

        if input == "s" || input == "S" {
            println!("- Apellidos:");
            let mut user_apellidos = String::new();
            io::stdin().read_line(&mut user_apellidos).expect("Error al leer los apellidos.");
            usuario.apellidos = user_apellidos.replace("\n","");
            if usuario.apellidos.trim().is_empty() {
                return Err("Es obligatorio indicar los apellidos.");
            }
        }

        println!("¿Quieres actualizar el nombre? (s/n)");
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let input = input.trim();

        if input == "s" || input == "S" {
            println!("- Nombre:");
            let mut user_nombre = String::new();
            io::stdin().read_line(&mut user_nombre).expect("Error al leer el nombre.");
            usuario.nombre = user_nombre.replace("\n","");
            if usuario.nombre.trim().is_empty() {
                return Err("Es obligatorio indicar el nombre.");
            }
        }

        println!("¿Quieres actualizar el sexo? (s/n)");
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let input = input.trim();

        if input == "s" || input == "S" {
            println!("- Sexo:");
            let mut user_sexo = String::new();
            io::stdin().read_line(&mut user_sexo).expect("Error al leer el sexo.");
            usuario.sexo = user_sexo.replace("\n","");
            if usuario.sexo.trim().is_empty() {
                return Err("Es obligatorio indicar el sexo.");
            } else {

                if usuario.sexo.trim().len() == 1 {
                    let re = Regex::new(r"^[MF]$").unwrap();

                    if !re.is_match(usuario.sexo.trim()) {
                        return Err("El sexo debe ser F o M.");
                    }
                } else {
                    return Err("El sexo debe ser un sólo carácter, F o M.");
                }
            }
        }

        println!("¿Quieres actualizar la nacionalidad? (s/n)");
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let input = input.trim();

        if input == "s" || input == "S" {
            println!("- Nacionalidad:");
            let mut user_nacionalidad = String::new();
            io::stdin().read_line(&mut user_nacionalidad).expect("Error al leer la nacionalidad.");
            usuario.nacionalidad = user_nacionalidad.replace("\n","");
            if usuario.nacionalidad.trim().is_empty() {
                return Err("Es obligatorio indicar la nacionalidad.");
            }
        }

        println!("¿Quieres actualizar la fecha de nacimiento? (s/n)");
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let input = input.trim();

        if input == "s" || input == "S" {
            println!("- Fecha de nacimiento:");
            let mut user_nacimiento = String::new();
            io::stdin().read_line(&mut user_nacimiento).expect("Error al leer la fecha de nacimiento.");
            usuario.nacimiento = user_nacimiento.replace("\n","");
            if usuario.nacimiento.trim().is_empty() {
                return Err("Es obligatorio indicar la fecha de nacimiento.");
            } else {
                let re = Regex::new(r"^\d{2}/\d{2}/\d{4}$").unwrap();

                if !re.is_match(usuario.nacimiento.trim()) {
                    return Err("La fecha de nacimiento debe tener el formato YYYY-MM-DD.");
                }
            }
        }
        usuario.update_db(&conn).unwrap();
    }
    Ok("Usuario editado correctamente.".to_string())
}


pub fn delete_user(conn: &sqlite::Connection) -> Result<String, &str> {
    let mut usuario = Usuario {
        id: 0,
        apellidos: String::new(),
        nombre: String::new(),
        sexo: String::new(),
        nacionalidad:String::new(),
        nacimiento: String::new(),
    };

    println!("Introduzca el id del usuario a eliminar:");
    let mut user_id = String::new();
    io::stdin().read_line(&mut user_id).expect("Error al leer el id.");
    usuario.id = user_id.replace("\n","").parse::<i32>().unwrap();
    if usuario.id == 0 {
        return Err("Es obligatorio indicar el id.");
    }

    usuario.delete_from_db(&conn).unwrap();

    Ok("Usuario eliminado correctamente.".to_string())
}

fn validate(mut usuario: Usuario) -> Result<Usuario, &'static str> {

    println!("Introduzca los datos del nuevo usuario:");
    println!("- Apellidos:");
    let mut user_apellidos = String::new();
    io::stdin().read_line(&mut user_apellidos).expect("Error al leer los apellidos.");
    usuario.apellidos = user_apellidos.replace("\n","");
    if usuario.apellidos.trim().is_empty() {
        return Err("Es obligatorio indicar los apellidos.");
    }

    println!("- Nombre:");
    let mut user_nombre = String::new();
    io::stdin().read_line(&mut user_nombre).expect("Error al leer el nombre.");
    usuario.nombre = user_nombre.replace("\n","");
    if usuario.nombre.trim().is_empty() {
        return Err("Es obligatorio indicar el nombre.");
    }
    println!("- Sexo:");
    let mut user_sexo = String::new();
    io::stdin().read_line(&mut user_sexo).expect("Error al leer el sexo.");
    usuario.sexo = user_sexo.replace("\n","");
    if usuario.sexo.trim().is_empty() {
        return Err("Es obligatorio indicar el sexo.");
    } else {

        if usuario.sexo.trim().len() == 1 {
            let re = Regex::new(r"^[MF]$").unwrap();

            if !re.is_match(usuario.sexo.trim()) {
                return Err("El sexo debe ser F o M.");
            }
        } else {
            return Err("El sexo debe ser un sólo carácter, F o M.");
        }
    }
    println!("- Nacionalidad:");
    let mut user_nacionalidad = String::new();
    io::stdin().read_line(&mut user_nacionalidad).expect("Error al leer la nacionalidad.");
    usuario.nacionalidad = user_nacionalidad.replace("\n","");
    if usuario.nacionalidad.trim().is_empty() {
        return Err("Es obligatorio indicar la nacionalidad.");
    }
    println!("- Fecha de nacimiento:");
    let mut user_nacimiento = String::new();
    io::stdin().read_line(&mut user_nacimiento).expect("Error al leer la fecha de nacimiento.");
    usuario.nacimiento = user_nacimiento.replace("\n","");
    if usuario.nacimiento.trim().is_empty() {
        return Err("Es obligatorio indicar la fecha de nacimiento.");
    } else {
        let re = Regex::new(r"^\d{2}/\d{2}/\d{4}$").unwrap();

        if !re.is_match(usuario.nacimiento.trim()) {
            return Err("El fecha de nacimiento debe tener el formato dd/mm/aaaa.");
        }
    }

    Ok(usuario)
}