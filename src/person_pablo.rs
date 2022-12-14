/// Esta es la estructura de un objeto Persona. De momento, tiene los atributos nombre y edad.
pub struct Person {
    pub name: String,
    pub age: u8,
}

impl Person {
    /// Esta función crea una persona.
    ///
    /// # Ejemplos
    ///
    /// ```
    /// let mut p = Person::new(String::from("John"), 30);
    /// assert_eq!(p.name, "John");
    /// assert_eq!(p.age, 30);
    /// ```
    pub fn new(name: String, age: u8) -> Person {
        Person { name, age }
    }

    /// Esta función lee una persona.
    ///
    /// # Ejemplos
    ///
    /// ```
    /// let mut p = Person::new(String::from("John"), 30);
    /// p.read();
    /// ```
    pub fn read(&self) {
        println!("Name: {}", self.name);
        println!("Age: {}", self.age);
    }

    /// Esta función edita una persona.
    ///
    /// # Ejemplos
    ///
    /// ```
    /// let mut p = Person::new(String::from("John"), 30);
    /// p.update(String::from("Jane"), 32);
    /// assert_eq!(p.name, "Jane");
    /// assert_eq!(p.age, 32);
    /// ```
    pub fn update(&mut self, name: String, age: u8) {
        self.name = name;
        self.age = age;
    }

    /// Esta función borra una persona.
    ///
    /// # Ejemplos
    ///
    /// ```
    /// let mut p = Person::new(String::from("John"), 30);
    /// p.delete();
    /// assert_eq!(p.name, "");
    /// assert_eq!(p.age, 0);
    /// ```
    pub fn delete(&mut self) {
        self.name = String::new();
        self.age = 0;
    }
}

/// Esta función realiza las operaciones CRUD de una persona: crea, edita, lee y borra.
///
/// # Ejemplos
///
/// ```
/// let mut p = Person::new(String::from("John"), 30);
/// p.read();
/// assert_eq!(p.name, "John");
/// assert_eq!(p.age, 30);
/// p.update(String::from("Jane"), 32);
/// p.read();
/// assert_eq!(p.name, "Jane");
/// assert_eq!(p.age, 32);
/// p.delete();
/// p.read();
/// assert_eq!(p.name, "");
/// assert_eq!(p.age, 0);
/// ```
pub fn run() {
    let mut p = Person::new(String::from("John"), 30);
    p.read();
    p.update(String::from("Jane"), 32);
    p.read();
    p.delete();
    p.read();
}
