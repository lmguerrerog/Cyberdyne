use sqlite::{ColumnIndex, Error as SqliteError};

#[derive(Debug)]
pub struct Usuario {
    pub(crate) id: i32,
    pub(crate) apellidos: String,
    pub(crate) nombre: String,
    pub(crate) sexo: String,
    pub(crate) nacionalidad: String,
    pub(crate) nacimiento: String,
}

impl Usuario {

    pub fn select_by_id (&self, conn: &sqlite::Connection) -> Result<Usuario, SqliteError> {

        let mut stmt = conn.prepare("SELECT * FROM usuarios WHERE id = ?").unwrap();
        stmt.bind((1, self.id.to_string().as_str())).unwrap();
        stmt.next().unwrap();
        let usuario = Usuario {
            id: stmt.read::<i64, _>("id").unwrap() as i32,
            apellidos: stmt.read::<String, _>("apellidos").unwrap(),
            nombre: stmt.read::<String, _>("nombre").unwrap(),
            sexo: stmt.read::<String, _>("sexo").unwrap(),
            nacionalidad: stmt.read::<String, _>("nacionalidad").unwrap(),
            nacimiento: stmt.read::<String, _>("f_nacimiento").unwrap(),
        };

        Ok(usuario)
    }

    pub fn insert_into_db(&self, conn: &sqlite::Connection) -> Result<(), SqliteError> {
        // Conectarse a la base de datos y preparar la instrucción SQL para insertar un usuario
        //let conn = sqlite::open(":memory:").unwrap();
        let mut stmt = conn.prepare("INSERT INTO usuarios (apellidos, nombre, sexo, nacionalidad, f_nacimiento) VALUES (?, ?, ?, ?, ?)").unwrap();
        stmt.bind((1, self.apellidos.as_str())).unwrap();
        stmt.bind((2, self.nombre.as_str())).unwrap();
        stmt.bind((3, self.sexo.as_str())).unwrap();
        stmt.bind((4, self.nacionalidad.as_str())).unwrap();
        stmt.bind((5, self.nacimiento.as_str())).unwrap();
        stmt.next().unwrap();
        Ok(())
    }

    pub fn delete_from_db(&self, conn: &sqlite::Connection) -> Result<(), SqliteError> {
        // Conectarse a la base de datos y preparar la instrucción SQL para insertar un usuario
        //let conn = sqlite::open(":memory:").unwrap();
        let mut stmt = conn.prepare("DELETE FROM usuarios WHERE id = ?").unwrap();
        stmt.bind((1, self.id.to_string().as_str())).unwrap();
        stmt.next().unwrap();
        Ok(())
    }

    pub fn update_db(&self, conn: &sqlite::Connection) -> Result<(), SqliteError> {
        // Conectarse a la base de datos y preparar la instrucción SQL para insertar un usuario
        //let conn = sqlite::open(":memory:").unwrap();
        let mut stmt = conn.prepare("UPDATE usuarios SET apellidos = ?, nombre = ?, sexo = ?, nacionalidad = ?, f_nacimiento = ? WHERE id = ?").unwrap();
        stmt.bind((1, self.apellidos.as_str())).unwrap();
        stmt.bind((2, self.nombre.as_str())).unwrap();
        stmt.bind((3, self.sexo.as_str())).unwrap();
        stmt.bind((4, self.nacionalidad.as_str())).unwrap();
        stmt.bind((5, self.nacimiento.as_str())).unwrap();
        stmt.bind((6, self.id.to_string().as_str())).unwrap();
        stmt.next().unwrap();
        Ok(())
    }
}