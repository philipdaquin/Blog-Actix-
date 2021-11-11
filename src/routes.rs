use crate::errors::AppError; 
use actix_web::HttpResponse; 

pub(super) mod posts; 
pub(super) mod users; 
pub(super) mod comments;    
//  We want the ROUTES to be able to refer to the USERS module but we don't want MODELS module to be able to refer to the users module 

fn convert<T, E>(res: Result<T, E>) -> Result<HttpResponse, AppError> where T: serde::Serialize, AppError: From<E>, {
//  **This function takes some generic result and returns another result with fixed types which will end up being nice to return from our handler functions
//  The successful variant (OK) is turned into HTTP Response with the data Serialized as JSON     
//  The Error variant is turned into AppError type => This will return a JSON error message
//  Then we put trait bounds  to specify the only input arguments that we're going to serialised to JSON 

    res.map(|d| HttpResponse::Ok().json(d))
    // We call map which only operates only on the success variant and builds a response

        .map_err(Into::into)    
}


