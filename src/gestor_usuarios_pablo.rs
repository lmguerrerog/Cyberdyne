use std::error::Error;

#[derive(Debug)]
enum UserError {
    CreationError,
    ReadError,
    UpdateError,
    DeleteErrorById,
    DeleteErrorByUser,
    DatabaseError,
}

impl std::fmt::Display for UserError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            UserError::CreationError => write!(f, "Failed to create user: {:?}", self),
            UserError::ReadError => write!(f, "Failed to read user from database: {:?}", self),
            UserError::UpdateError => write!(f, "Failed to update user: {:?}", self),
            UserError::DeleteErrorById => write!(f, "Failed to delete user by ID: {:?}", self),
            UserError::DeleteErrorByUser => write!(f, "Failed to delete user: {:?}", self),
            UserError::DatabaseError => write!(f, "Failed to connect with Database: {:?}", self),
        }
    }
}

impl std::fmt::Display for UserError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.description())
    }
}

fn create_user(user: User) -> Result<User, UserError::CreationError> {
    create_user_in_database(user)?;
    Ok(user)
}

fn read_user(id: &str) -> Result<User, UserError::ReadError> {
    let user = read_user_from_database(id)?;
    Ok(user)
}

fn update_user(id: &str, user: User) -> Result<User, UserError::DeleteErrorById> {
    update_user_in_database(id, user)?;
    Ok(user)
}

fn delete_user_by_id(id: &str) -> Result<User, UserError::DeleteErrorById> {
    let user = delete_user_from_database(id)?;
    Ok(user)
}

fn delete_user(user: User) -> Result<User, UserError::DeleteErrorByUser> {
    delete_user_from_database(user)?;
    Ok(user)
}
