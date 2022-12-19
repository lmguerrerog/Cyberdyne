#![allow(unused)]

use std::io;

#[derive(Debug,PartialEq)]
pub struct User {
    pub id: String,
    pub dni: String,
    pub sub_names: String,
    pub name: String,
    pub gender: String,
    pub nationality: String,
    pub birt_date: String,
    pub support_number: String,
    pub expired_date: String,
    pub can: String,
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

fn get_index(user_id: &str) -> usize {
    let index = &*&user_id.parse::<usize>().unwrap()-1;
    return index;
}

fn create_user_1() -> User {
    User {
        id: String::from("1"),
        dni: String::from("99999999R"),
        sub_names: String::from("ESPAÑOLA ESPAÑOLA"),
        name: String::from("CARMEN"),
        gender: String::from("F"),
        nationality: String::from("ESP"),
        birt_date: String::from("01/01/1980"),
        support_number: String::from("BAA000589"),
        expired_date: String::from("01/01/2025"),
        can: String::from("987654"),
    }
}

fn create_user_2() -> User {
    User {
        id: String::from("2"),
        dni: String::from("88888888R"),
        sub_names: String::from("ESPAÑOL ESPAÑOL"),
        name: String::from("CARLOS"),
        gender: String::from("M"),
        nationality: String::from("ESP"),
        birt_date: String::from("01/01/1990"),
        support_number: String::from("BAA000985"),
        expired_date: String::from("01/01/2026"),
        can: String::from("456789"),
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_create_user() {
        let user = create_user_1();
        assert!(create_user(&user).is_ok());
    }

    #[test]
    pub fn test_read_user() {
        let user = create_user_1();
        assert!(read_user(&user.id).is_ok());
    }

    #[test]
    pub fn test_update_user() {
        let user = create_user_1();
        assert!(update_user(&user).is_ok());
    }

    #[test]
    pub fn test_delete_user_by_id() {
        let user = create_user_1();
        assert!(delete_user_by_id(&user.id).is_ok());
    }

    #[test]
    pub fn test_delete_user() {
        let user = create_user_1();
        assert!(delete_user(&user).is_ok());
    }

    #[test]
    pub fn test_find_user() {
        let user = create_user_1();
        assert!(find_user(&user).is_ok());
    }

    #[test]
    fn test_new_user() {
        let user1 = create_user_1();
        assert_eq!(user1.id, String::from("1"));
        assert_eq!(user1.dni, String::from("99999999R"));
        assert_eq!(user1.sub_names, String::from("ESPAÑOLA ESPAÑOLA"));
        assert_eq!(user1.name, String::from("CARMEN"));
        assert_eq!(user1.gender, String::from("F"));
        assert_eq!(user1.nationality, String::from("ESP"));
        assert_eq!(user1.birt_date, String::from("01/01/1980"));
        assert_eq!(user1.support_number, String::from("BAA000589"));
        assert_eq!(user1.expired_date, String::from("01/01/2025"));
        assert_eq!(user1.can, String::from("987654"));
        let user2 = create_user_2();
        assert_eq!(user2.id, String::from("2"));
        assert_eq!(user2.dni, String::from("88888888R"));
        assert_eq!(user2.sub_names, String::from("ESPAÑOL ESPAÑOL"));
        assert_eq!(user2.name, String::from("CARLOS"));
        assert_eq!(user2.gender, String::from("M"));
        assert_eq!(user2.nationality, String::from("ESP"));
        assert_eq!(user2.birt_date, String::from("01/01/1990"));
        assert_eq!(user2.support_number, String::from("BAA000985"));
        assert_eq!(user2.expired_date, String::from("01/01/2026"));
        assert_eq!(user2.can, String::from("456789"));
    }

    #[test]
    fn vectors() {
        let mut vec: Vec<User> = Vec::new();
        let mut user1 = create_user_1();
        let mut user2 = create_user_2();
        let mut user1_vector = create_user_1();
        let mut user2_vector = create_user_2();
        match create_user(&user1) {
            Ok(())=>{
                vec.push(user1_vector);
                println!("create : {:?}", vec.get(get_index(&user1.id)));
            },
            _=>eprintln!("error")
        }
        assert_eq!(&user1, &vec[get_index(&user1.id)]);
        match create_user(&user2) {
            Ok(())=>{
                vec.push(user2_vector);
                println!("create : {:?}", vec.get(get_index(&user2.id)));
            },
            _=>eprintln!("error")
        }
        assert_eq!(&user2, &vec[get_index(&user2.id)]);
        match read_user(&user1.id) {
            Ok(())=>println!("read : {:?}",vec.get(get_index(&user1.id))),
            _=>eprintln!("error")
        }
        assert_eq!(&user1.id, &vec[get_index(&user1.id)].id);
        match find_user(&user2) {
            Ok(())=>println!("find : {:?}", vec.get(get_index(&user2.id))),
            _=>eprintln!("error")
        }
        assert_eq!(&user2.id, &vec[get_index(&user2.id)].id);
        match update_user(&user1) {
            Ok(())=>{
                *&mut vec[get_index(&user1.id)].name = String::from("MARI CARMEN");
                println!("update : {:?}", vec.get(get_index(&user1.id)));
            },
            _=>eprintln!("error")
        }
        assert_eq!(&String::from("MARI CARMEN"), &vec[get_index(&user1.id)].name);
        match update_user(&user2) {
            Ok(())=>{
                *&mut vec[get_index(&user2.id)].name = String::from("JOSE CARLOS");
                println!("update : {:?}", vec.get(get_index(&user2.id)));
            },
            _=>eprintln!("error")
        }
        assert_eq!(&String::from("JOSE CARLOS"), &vec[get_index(&user2.id)].name);
        match delete_user_by_id(&user2.id) {
            Ok(())=>println!("delete : {:?}", vec.remove(get_index(&user2.id))),
            _=>eprintln!("error")
        }
        assert_eq!(vec.len(),1);
        match delete_user(&user1) {
            Ok(())=>println!("delete : {:?}", vec.remove(get_index(&user1.id))),
            _=>eprintln!("error")
        }
        assert_eq!(vec.len(),0);
    }
}