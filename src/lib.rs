#[macro_use]

extern crate diesel; 
#[macro_use]

extern crate serde_derive; 
use actix_web::{middleware, App, HttpServer}; 
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager}; 
type Pool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

mod errors; 
mod models; 
mod routes; 
mod schema; 

pub struct Blog {
    port: u16, 
}

impl Blog { 
    pub fn new(port: u16) -> Self { 
        Blog { port }
    }
// We represent DataBase URL as a string which is used to construct a pool of database connections
    pub fn run(&self, database_url: String) -> std::io::Result<()> { 

        // Connection Manager is specified to be SqliteConnection which came from DIESEL
        let manager = ConnectionManager::<SqliteConnection>::new(database_url); 

        // Here we construct a connection pool and store it in the pool variable
        let pool = r2d2::Pool::builder() 
            .build(manager)
            .expect("Failed to create pool");

            println!("Starting HTTP Server: 127.0.0.1:{}", self.port);
            
            //We clone our pool so that each worker will have access to the same shared pool 
            HttpServer::new(move || { 
                App::new()
                    .data(pool.clone())
                    .wrap(middleware::Logger::default())

                    // Route modules exposes a submodule called users, adn that submodule needs to
                    // publicly expose a function called configure 
                    .configure(routes::users::configure)  
                    .configure(routes::posts::configure)
                    .configure(routes::comments::configure)

            }) 
            .bind(("127.0.0.1", self.port))? 
            .run()

    }
}
