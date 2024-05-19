use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::Serialize;

pub enum Error {
    GeneralError(GeneralError<String>),
    FileUploadError(FileUploadError),
}

pub enum GeneralError<String> {
    NotFound,
    BadRequest,
    Unauthorized,
    AlreadyExists,
    Unhandled(String),
}

pub enum FileUploadError {
    MultipartParseFailed,
    FileNotCreated,
    FileNotWritten,
}

#[derive(Serialize)]
struct ErrorMessage {
    message: String
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        match self {
            Error::GeneralError(error) => {
                match error {
                    GeneralError::NotFound => (
                        StatusCode::NOT_FOUND,
                        Json(ErrorMessage { message: "Item not found".to_owned() })
                    ).into_response(),
                    GeneralError::BadRequest => (
                        StatusCode::BAD_REQUEST,
                        Json(ErrorMessage { message: "Bad request".to_owned() })
                    ).into_response(),
                    GeneralError::Unauthorized => (
                        StatusCode::UNAUTHORIZED,
                        Json(ErrorMessage { message: "Unauthorized".to_owned() })
                    ).into_response(),
                    GeneralError::AlreadyExists => (
                        StatusCode::BAD_REQUEST,
                        Json(ErrorMessage { message: "Already exists".to_owned() })
                    ).into_response(),
                    GeneralError::Unhandled(message) => (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(ErrorMessage { message })
                    ).into_response(),
                }
            },

            Error::FileUploadError(error) => {
                match error {
                    FileUploadError::MultipartParseFailed => (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(ErrorMessage { message: "Failed to parse multipart data".to_owned() })
                    ).into_response(),
                    FileUploadError::FileNotCreated => (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(ErrorMessage { message: "Failed to open file handle".to_owned() })
                    ).into_response(),
                    FileUploadError::FileNotWritten => (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(ErrorMessage { message: "Failed to write to file".to_owned() })
                    ).into_response(),
                }
            }
        }
    }
}
