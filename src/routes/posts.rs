//  Adding routes for posts 
//  First, We declare and export the soon to be written posts module within our routes module;

use crate::errors::AppError; 
use crate::routes::convert; 
use crate::{models, Pool}; 
use actix_web::{web, HttpResponse}; 
use diesel::prelude::*;
use futures::Future; 

//  Creating a Posts
//  We only need to get the title and body of the Post as the rest of the information will be inferred from the URL or will take on the default value
#[derive(Debug, Serialize, Deserialize)]
struct PostInput { 
    title: String,
    body: String, 
}

//  Exporting a Configure Function 
//  This will modify the input configuration to add the relevant routes for posts: 
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/users/{id}/posts")
            .route(web::post().to_async(add_post))
            .route(web::get().to_async(user_posts)), 
            //  POST and a GET request route to our 'add_posts' and 'user_posts' handlers
    ) 
    .service(web::resource("/posts").route(web::get().to_async(all_posts)))
    .service(web::resource("/posts/{id}/publish").route(web::post().to_async(publish_post))); 
}


//  The User_Id will the be the Author. We take that path as input as well as the post as JSON and the database pool
//  We need to convert the ID we take as input into a User before we can use 
fn add_post(user_id: web::Path<i32>, post: web::Json<PostInput>, pool: web::Data<Pool>,) -> impl Future<Item = HttpResponse, Error = AppError> { 
    web::block(move || { 
        let conn: &SqliteConnection = &pool.get().unwrap();
        let key = models::UserKey::ID(user_id.into_inner()); 
        
        models::find_user(conn, key).and_then(|user| { 
        //  We use the 'and_then' method on Result to continue on to creating a post only in the case where we actually found a user
        //  This way we handle all of the different errors without having a mess of conditionals  
            let post = post.into_inner(); 
            let title = post.title; 
            let body = post.body; 
            models::create_post(conn, &user, title.as_str(), body.as_str())
        })
    })
    .then(convert)
}

//  Publishing a Post 
//  We just need a post_id in the path and can then rely on the model code 

fn publish_post(post_id: web::Path<i32>, pool: web::Data<Pool>,) -> impl Future<Item = HttpResponse, Error = AppError> {
    web::block(move || { 
        let conn: &SqliteConnection = &pool.get().unwrap();
        models::publish_post(conn, post_id.into_inner())
    }) 
    .then(convert)

}

//  Fetching Posts 
//  We can fetch posts either given a user_id or just fetch them all. 
fn user_posts(user_id: web::Path<i32>, pool: web::Data<Pool>) -> impl Future<Item = HttpResponse, Error = AppError> { 
    web::block( move || { 
        let conn: &SqliteConnection = &pool.get().unwrap(); 
        models::user_posts(conn, user_id.into_inner())
    })
    .then(convert)
}

fn all_posts(pool: web::Data<Pool>) -> impl Future<Item = HttpResponse, Error = AppError> {
    web::block(move || { 
        let conn: &SqliteConnection = &pool.get().unwrap(); 
        models::all_posts(conn)
    }) 
    .then(convert)

}