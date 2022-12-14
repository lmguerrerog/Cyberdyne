/// Esta es la estructura de un objeto Persona. De momento, tiene los atributos nombre y edad.
pub struct Person {
    pub name: String,
    pub age: u8,
}

impl Person {
    /// Esta función crea una persona.
    pub fn new(name: String, age: u8) -> Person {
        Person { name, age }
    }

    /// Esta función lee una persona.
    pub fn read(&self) {
        println!("Name: {}", self.name);
        println!("Age: {}", self.age);
    }

    /// Esta función edita una persona.
    pub fn update(&mut self, name: String, age: u8) {
        self.name = name;
        self.age = age;
    }

    /// Esta función borra una persona.
    pub fn delete(&mut self) {
        self.name = String::new();
        self.age = 0;
    }
}

/// Esta función realiza las operaciones CRUD de una persona: crea, edita, lee y borra.
pub fn run() {
    let mut p = Person::new(String::from("John"), 30);
    p.read();
    p.update(String::from("Jane"), 32);
    p.read();
    p.delete();
    p.read();
}

/// Test para comprobar que los métodos CRUD del objeto Persona funcionan correctamente.
#[test]
fn test_main() {
    // Crear una nueva persona
    let mut p = Person::new(String::from("John"), 30);

    // Leer los datos de la persona
    p.read();

    // Comprobar si los datos leídos son los esperados
    assert_eq!(p.name, "John");
    assert_eq!(p.age, 30);

    // Actualizar los datos de la persona
    p.update(String::from("Jane"), 32);

    // Leer los datos actualizados de la persona
    p.read();

    // Comprobar si los datos leídos son los esperados después de la actualización
    assert_eq!(p.name, "Jane");
    assert_eq!(p.age, 32);

    // Eliminar los datos de la persona
    p.delete();

    // Leer los datos de la persona después de haber sido eliminados
    p.read();

    // Comprobar si los datos leídos son los esperados después de la eliminación
    assert_eq!(p.name, "");
    assert_eq!(p.age, 0);
}
