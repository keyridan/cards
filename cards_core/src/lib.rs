#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_codegen;
extern crate r2d2;
extern crate r2d2_diesel;
extern crate dotenv;

pub mod models;
pub mod connection;
pub mod repository;
pub mod service;

pub use diesel::prelude::*;