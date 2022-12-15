use std::io;

pub fn aplicacion() {
    // Creación de las variables para dibujar el menú.
    let linea = "-";
    let igual = "=";
    let menu = "| Aplicación entrada datos usuario |";
    let opcion = "| Selecciona opción a introducir   |";

    // Implementamos la interfaz `Debug` para la estructura `Persona`
    // con los atributos `nombre`, `apellido1`, `apellido2`, `fecha`,
    // `sexo` y `dni`.
    struct Persona {
        nombre: String,
        apellido1: String,
        apellido2: String,
        fecha: String,
        sexo: String,
        dni: String,
    }

    // Creamos una función para imprimir el menú.
    // Esta función no recibe ningún parámetro y no devuelve nada.
    let imprimir_menu = || {
        println!("{}", igual.repeat(36));
        println!("{}", menu);
        println!("{}", igual.repeat(36));
        println!("{}", opcion);
        println!("{}", linea.repeat(36));
        println!("{}", "1. Nombre");
        println!("{}", "2. Primer apellido");
        println!("{}", "3. Segundo apellido");
        println!("{}", "4. Fecha");
        println!("{}", "5. Sexo");
        println!("{}", "6. DNI");
        println!("{}", "7. Imprimir persona");
        println!("{}", "8. Salir");
        println!("{}", linea.repeat(33));
    };

    // Creamos una función para leer la entrada del usuario.
    // Esta función no recibe ningún parámetro y devuelve un `String`.
    // El `String` devuelto es la entrada del usuario.
    struct Datos {
        persona: Persona,
    }

    // Creamos una función para leer la entrada del usuario.
    // Se asigna a cada variable de la estructura `Persona` el valor.
    let mut datos = Datos {
        persona: Persona {
            nombre: String::new(),
            apellido1: String::new(),
            apellido2: String::new(),
            fecha: String::new(),
            sexo: String::new(),
            dni: String::new(),
        },
    };

    // Creamos un bucle `loop` para que el programa se repita.
    loop {
        // Imprimimos el menú.
        imprimir_menu();
        // Creamos una persona vacía.
        let _persona = Persona {
            nombre: String::new(),
            apellido1: String::new(),
            apellido2: String::new(),
            fecha: String::new(),
            sexo: String::new(),
            dni: String::new(),
        };

        // Leemos la opción del usuario.
        // Creamos una variable `entrada` de tipo `String` vacía.
        // Leemos la entrada del usuario y la guardamos en la variable `entrada`.
        let mut entrada = String::new();
        io::stdin()
            .read_line(&mut entrada)
            .expect("Fallo en lectura");

        // Creamos una variable `opcion` de tipo `u32` vacía.
        // Convertimos la entrada a un número entero y la guardamos en la variable `opcion`.
        // Si la conversión falla, se imprime un mensaje de error y se vuelve a pedir la opción.
        // Si la conversión es correcta, continúa.
        let entrada: i32 = match entrada.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // Creamos un `match` para comprobar la opción del usuario.
        // Realizamos la acción correspondiente a la opción seleccionada.
        // Cada número corresponde a una opción.
        match entrada {
            1 => {
                println!("Introduce el nombre");
                io::stdin()
                    .read_line(&mut datos.persona.nombre)
                    .expect("Fallo al leer la línea");
            }
            2 => {
                println!("Introduce el primer apellido");
                io::stdin()
                    .read_line(&mut datos.persona.apellido1)
                    .expect("Fallo al leer la línea");
            }
            3 => {
                println!("Introduce el segundo apellido");
                io::stdin()
                    .read_line(&mut datos.persona.apellido2)
                    .expect("Fallo al leer la línea");
            }
            4 => {
                println!("Introduce la fecha");
                io::stdin()
                    .read_line(&mut datos.persona.fecha)
                    .expect("Fallo al leer la línea");
            }
            5 => {
                println!("Introduce el sexo");
                io::stdin()
                    .read_line(&mut datos.persona.sexo)
                    .expect("Fallo al leer la línea");
            }
            6 => {
                println!("Introduce el DNI");
                io::stdin()
                    .read_line(&mut datos.persona.dni)
                    .expect("Fallo al leer la línea");
            }
            7 => {
                println!("Imprimiendo la persona");
                if !datos.persona.nombre.is_empty()
                    && !datos.persona.apellido1.is_empty()
                    && !datos.persona.apellido2.is_empty()
                    && !datos.persona.fecha.is_empty()
                    && !datos.persona.sexo.is_empty()
                    && !datos.persona.dni.is_empty()
                {
                    println!(
                        "Los datos introducidos son: \t
                        Nombre: {} \t\t\t\t\t\t Primer apellido: {} \t\t\t\t\t\t Segundo apellido: {} \t
                        Fecha de nacimiento: {} \t\t\t\t\t\t Sexo: {} \t\t\t\t\t\t DNI: {}",
                        datos.persona.nombre,
                        datos.persona.apellido1,
                        datos.persona.apellido2,
                        datos.persona.fecha,
                        datos.persona.sexo,
                        datos.persona.dni
                    );
                } else {
                    println!("No se han introducido todos los datos");
                }
            }
            8 => {
                println!("Saliendo del programa");
                break;
            }
            _ => {
                println!("Opción inválida. Por favor, selecciona una opción del menú.");
            }
        }
    }
}
