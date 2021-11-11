//This file encapsulates the different types of errors that can happen so we can explicitly handle those 
//scenariors and can avoid generic 500 errors as much as possible 

use actix_web::error::BlockingError; 
use actix_web::web::HttpResponse; 
use diesel::result::DatabaseErrorKind::UniqueViolation; 
use diesel::result::Error::{DatabaseError, NotFound}; 
use std::fmt; 

#[derive(Debug)]
pub enum AppError { 
    RecordAlreadyExists, 
    RecordNotFound,
    // Diesel Errors that we don't specifically handle 
    DatabaseError(diesel::result::Error), 
    // Related to a actix_web error having to do with an async operation which we will explain later 
    OperationCanceled, 
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self { 
            AppError::RecordAlreadyExists => write!(f, "This record violates a unique constraint"), 
            AppError::RecordNotFound => write!(f, "This record does not exist"), 
            AppError::DatabaseError(e) => write!(f, "Database error: {:?}", e),
            AppError::OperationCanceled => write!(f, "The running operation was canceled"),  
        } 
    }
}

// This will give us an instance of AppError and then we'll match it on cases we care about
impl From<diesel::result::Error> for AppError {

    fn from(e: diesel::result::Error) -> Self {

        match e {

            // Return a Unique Violation if there's already an existing username 
            DatabaseError(UniqueViolation, _) => AppError::RecordAlreadyExists,
            // If not found in our Database, we return a Not Found Error 
            NotFound => AppError::RecordNotFound, 
            // We catch all other case of errors here and call it a DatabaseError
            _ => AppError::DatabaseError(e), 
        }
    }
}

// This is specific to Actix Web 
impl From<BlockingError<AppError>> for AppError {
    fn from(e: BlockingError<AppError>) -> Self {
        match e {
            BlockingError::Error(inner) => inner, 
            BlockingError::Canceled => AppError::OperationCanceled,
        }
    }
}

// Errors as responses, here, we are creating a struct to represent a JSON error response 
#[derive(Debug, Serialize)]
struct ErrorResponse { 
    err: String, 
}

impl actix_web::ResponseError for AppError { 
    fn error_response(&self) -> HttpResponse { 
        let err = format!("{}", self); 
        let mut builder = match self { 
            AppError::RecordAlreadyExists => HttpResponse::BadRequest(), 
            AppError::RecordNotFound => HttpResponse::NotFound(),
            _ => HttpResponse::InternalServerError(),  
        }; 

        builder.json(ErrorResponse { err })
    }

    fn render_response(&self) -> HttpResponse { 
        self.error_response()
    }
}



