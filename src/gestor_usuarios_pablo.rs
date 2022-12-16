#![allow(unused)]

use std::io;

pub struct User{}

pub fn create_user(user: &User) -> Result<(), io::Error> {
    Ok(())
}

pub fn read_user(id: &str) -> Result<(), io::Error> {
    Ok(())
}

pub fn update_user(id: &str, user: &User) -> Result<(), io::Error> {
    Ok(())
}

pub fn delete_user_by_id(id: &str) -> Result<(), io::Error> {
    Ok(())
}

pub fn delete_user(user: &User) -> Result<(), io::Error> {
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_create_user() {
        let user = User {};
        assert!(create_user(&user).is_ok());
    }

    #[test]
    pub fn test_read_user() {
        let id = "1";
        assert!(read_user(id).is_ok());
    }

    #[test]
    pub fn test_update_user() {
        let id = "1";
        let user = User {};
        assert!(update_user(id,&user).is_ok());
    }

    #[test]
    pub fn test_delete_user_by_id() {
        let id = "1";
        assert!(delete_user_by_id(id).is_ok());
    }

    #[test]
    pub fn test_delete_user() {
        let user = User {};
        assert!(delete_user(&user).is_ok());
    }
}