use crate::errors::AppError; 
use crate::routes::convert; 
use crate::{models, Pool}; 
use actix_web::{web, HttpResponse};
use futures::Future; 

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/users").route(web::post().to_async(create_user)))
        .service(web::resource("/users/find{name}").route(web::get().to_async(find_user)))
        .service(web::resource("/users/{id}").route(web::get().to_async(get_user))); 
}
//  THe signature of this function is epcified by Actix Web. The only parameter is a mutable reference to a service configuration object
//  TThe object oassed to us here allows for that exact same type of configuration 
//  We use to_async to specify the handlers here because our handlers return futures rather 'to' that we used before with synchronous handlers 

//  Create a user
#[derive(Debug, Serialize, Deserialize)] 
//  As we derive Deserialise we will be able to accept this type as a JSON post body
struct UserInput {
    username: String, 
}

fn create_user(item: web::Json<UserInput>, pool: web::Data<Pool>, ) -> impl Future<Item = HttpResponse, Error = AppError> { 
//  We need the input data as the JSON body of the request and we need a handle to our database pool which 
//  we put inside our application state back in our application factor

// Future<Item, Error> is an object that represents a computation which can be queried for a result or an error 
// This is a standard approach to writing async code where your return immediately some value that represents a computation rather than doing the computation before returning 
// the Future resolves to a results or an error when the computation completes

// The 'impl Future' means that we are going to return some type that implements the Future Trait, but we are not telling you exactly what that type is
// This will give us some flexibility anbd is neccessary for some types which are hard to write

//  The item is the type that the futureresolves to in a successful case
//  The Error is self explanotory 
//  We want to return an HttpResonse int he successful and AppError in the other case 
    
    web::block(move || { 

        let conn = &pool.get().unwrap(); 
        let username = item.into_inner().username; 
        models::create_user(conn, username.as_str())
    })


    .then(convert)
    //  The convert function is used to turn the result of the call to models::create_user into the response we desire 
}

//  Find a User 
fn find_user(name: web::Path<String>, pool: web::Data<Pool>, ) -> impl Future<Item = HttpResponse, Error = AppError> { 
    web::block(move || {
        let conn = &pool.get().unwrap(); 
        let name = name.into_inner(); 
        let key = models::UserKey::Username(name.as_str()); 
        models::find_user(conn, key)
    })
    .then(convert)
}
//  This will be a GET request so we expect the username to be a strng in the path
//  We have to create a UserKey::Username and then call our models::find_user function to do the work 

fn get_user(user_id: web::Path<i32>, pool: web::Data<Pool>, ) -> impl Future<Item = HttpResponse, Error = AppError> {
//  We expect an i32 in the path instead of a string and we create the other variant of the UserKey enum 

    web::block(move || { 
        let conn = &pool.get().unwrap(); 
        let id = user_id.into_inner(); 
        let key = models::UserKey::ID(id); 
        models::find_user(conn, key)
    })
    .then(convert)

}