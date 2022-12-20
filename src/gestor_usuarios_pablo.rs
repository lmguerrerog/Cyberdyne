#![allow(unused)]

use std::io;

use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime, Utc};
use chrono::format::ParseError;

#[derive(Debug,PartialEq,Eq)]
pub enum Gender{
    Masculin,
    Feminine,
    Other
}

#[derive(Debug,PartialEq,Eq)]
pub struct User {
    pub id: String,
    pub dni: String,
    pub sub_names: String,
    pub name: String,
    pub gender: Gender,
    pub nationality: String,
    pub birt_date: Option<NaiveDate>,
    pub support_number: String,
    pub expired_date: Option<NaiveDate>,
    pub can: u32,
}

pub fn create_user(user: &User) -> Result<(), io::Error> {
    Ok(())
}

pub fn read_user(id: &str) -> Result<(), io::Error> {
    Ok(())
}

pub fn update_user(user: &User) -> Result<(), io::Error> {
    Ok(())
}

pub fn delete_user_by_id(id: &str) -> Result<(), io::Error> {
    Ok(())
}

pub fn delete_user(user: &User) -> Result<(), io::Error> {
    Ok(())
}

pub fn find_user(user: &User) -> Result<(), io::Error> {
    Ok(())
}

/// Los siguiente tres métodos son privados porque son principalmente para test

fn get_index(user_id: &str) -> usize {
    user_id.parse::<usize>().unwrap()-1
}

fn create_user_1() -> User {
    let birt_date: Option<NaiveDate> = NaiveDate::from_ymd_opt(1980, 1, 1);
    let expired_date: Option<NaiveDate> = NaiveDate::from_ymd_opt(2025, 1, 1);
    User {
        id: String::from("1"),
        dni: String::from("99999999R"),
        sub_names: String::from("León Cruz"),
        name: String::from("Carmen"),
        gender: Gender::Feminine,
        nationality: String::from("ESP"),
        birt_date,
        support_number: String::from("BAA000589"),
        expired_date,
        can: 987654,
    }
}

fn create_user_2() -> User {
    let birt_date: Option<NaiveDate> = NaiveDate::from_ymd_opt(1990, 1, 1);
    let expired_date: Option<NaiveDate> = NaiveDate::from_ymd_opt(2026, 1, 1);
    User {
        id: String::from("2"),
        dni: String::from("88888888R"),
        sub_names: String::from("Díaz Cano"),
        name: String::from("Carlos"),
        gender: Gender::Masculin,
        nationality: String::from("ESP"),
        birt_date,
        support_number: String::from("BAA000985"),
        expired_date,
        can: 456789,
    }
}

/// Para imprimir mensajes por consola al hacer el test: cargo test -- --nocapture

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {

        /// Se crean los dos usuarios de pruebas y se le asignan los atributos

        let user1 = create_user_1();
        let user2 = create_user_2();

        /// Se comprueba que no dan errores los siguientes métodos:
        /// create, read, find, update, delete_by_id, delete_by_user

        assert!(create_user(&user1).is_ok());
        assert!(read_user(&user1.id).is_ok());
        assert!(find_user(&user1).is_ok());
        assert!(update_user(&user1).is_ok());
        assert!(delete_user_by_id(&user1.id).is_ok());
        assert!(delete_user(&user1).is_ok());

        /// Se crea el vector que va a alojar los usuarios
        /// Se crean los usuarios que se van a añadir al vector

        let mut vec: Vec<User> = Vec::new();
        let mut user1_vector = create_user_1();
        let mut user2_vector = create_user_2();

        /// Se añade los dos usuarios con create_user

        match create_user(&user1) {
            Ok(()) => {
                vec.push(user1_vector);
                let index = get_index(&user1.id);
                let user_option = vec.get(index);
                match user_option {
                    Some(user) => println!("create_user(1)"),
                    None => eprintln!("User not found"),
                }
            }
            _ => eprintln!("Error"),
        }
        assert_eq!(&user1, &vec[get_index(&user1.id)]);
        match create_user(&user2) {
            Ok(()) => {
                vec.push(user2_vector);
                let index = get_index(&user2.id);
                let user_option = vec.get(index);
                match user_option {
                    Some(user) => println!("create_user(2)"),
                    None => eprintln!("User not found"),
                }
            }
            _ => eprintln!("Error"),
        }
        assert_eq!(&user2, &vec[get_index(&user2.id)]);

        /// Se lee el primer usuario con read_user

        match read_user(&user1.id) {
            Ok(()) => {
                let index = get_index(&user1.id);
                let user_option = vec.get(index);
                match user_option {
                    Some(user) => println!("read_user(1)   : {:?}", user),
                    None => eprintln!("User not found"),
                }
            }
            _ => eprintln!("Error"),
        }
        assert_eq!(&user1.id, &vec[get_index(&user1.id)].id);

        /// Se lee el segundo usuario con find_user

        match find_user(&user2) {
            Ok(()) => {
                let index = get_index(&user2.id);
                let user_option = vec.get(index);
                match user_option {
                    Some(user) => println!("find_user(2)   : {:?}", user),
                    None => eprintln!("User not found"),
                }
            }
            _ => eprintln!("Error"),
        }
        assert_eq!(&user2.id, &vec[get_index(&user2.id)].id);

        /// Se actualiza el DNI del primer usuario con update_user

        match update_user(&user1) {
            Ok(()) => {
                let index = get_index(&user1.id);
                *&mut vec[index].dni = String::from("00000000B");
                let user_option = vec.get(index);
                match user_option {
                    Some(user) => println!("update_user(1) : {:?}", user),
                    None => eprintln!("User not found"),
                }
            }
            _ => eprintln!("Error"),
        }
        assert_eq!(&String::from("00000000B"), &vec[get_index(&user1.id)].dni);

        /// Se borra el segundo usuario con delete_user_by_id

        match delete_user_by_id(&user2.id) {
            Ok(())=>{
                vec.remove(get_index(&user2.id));
                println!("delete_user_by_id(2)");
            },
            _=>eprintln!("Error")
        }
        assert_eq!(vec.len(),1);

        /// Se borra el primer usuario con delete_user

        match delete_user(&user1) {
            Ok(())=>{
                vec.remove(get_index(&user1.id));
                println!("delete_user(1)");
            },
            _=>eprintln!("Error")
        }
        assert_eq!(vec.len(),0);
    }
}