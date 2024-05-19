use crate::{errors::{Error, GeneralError}, AppState};

pub enum Role {
    Customer,
}

impl Role {
    pub fn as_str(&self) -> &str {
        match self {
            Role::Customer => "CUSTOMER",
        }
    }
}

#[derive(Debug)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub password: String,
    pub role: String
}

pub struct UserDto {
    pub email: String,
    pub password: String,
    pub role: Role
}

pub struct ReturnId {
    pub id: i32
}

impl User {
    pub async fn get_by_email(app_state: &AppState, email: &str) -> Result<User, Error> {
        let user = sqlx::query_as!(
            User,
            "SELECT * FROM users WHERE email = $1",
            email
        )
            .fetch_one(&app_state.pool)
            .await
            .map_err(|_| Error::GeneralError(GeneralError::Unauthorized))?;

        Ok(user)
    }

    pub async fn create_user(app_state: &AppState, user: UserDto) -> Result<ReturnId, Error> {
        let result = sqlx::query_as!(
            ReturnId,
            "INSERT INTO users (email, password, role) VALUES ($1, $2, $3) RETURNING id",
            user.email, user.password, user.role.as_str()
        )
            .fetch_one(&app_state.pool)
            .await
            .map_err(|_| Error::GeneralError(GeneralError::Unhandled("Creation failed for user".to_owned())))?;

        Ok(result)
    }
}
