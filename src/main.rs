#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;
extern crate rocket_sync_db_pools;

use rocket_dyn_templates::Template;
use rocket_sync_db_pools::database;

use rocket::fs::{relative, FileServer};

mod db;
mod handlers;
mod models;
mod schema;

use crate::handlers::*;

#[database("persons")]
pub struct DBPool(diesel::PgConnection);

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", FileServer::from(relative!("/static")))
        .mount(
            "/",
            routes![
                start,
                all_persons,
                new_person,
                delete_person,
                update_person,
                get_person_by_name,
                all_genres,
                new_genre,
                get_genre_by_type,
                all_partitions,
                new_partition,
                get_partition_by_title,
                get_partition_by_author,
                get_partition_by_genre,
                delete_partition,
                about
            ],
        )
        .attach(DBPool::fairing())
        .attach(Template::fairing())
        .register("/", catchers![not_found])
}
