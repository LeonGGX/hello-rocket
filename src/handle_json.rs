use std::collections::HashMap;

use rocket::form::Form;
use rocket::Request;
use rocket::response::{Flash, Redirect};
use rocket_contrib::templates::Template;

use crate::{DBPool, db};
use crate::models::Person;
use rocket::http::Status;
use rocket_contrib::databases::diesel::result::Error;
use rocket::response::content::Json;


#[get("/")]
pub fn start() -> Template {

    #[derive(serde::Serialize)]
    struct StartContext {
        title: String,
    }

    let context = StartContext { title: "Start".to_string() };
    Template::render("start", &context)
}

#[get("/persons")]
pub async fn all_persons(conn: DBPool) -> Json<Vec<Person>> {
    conn.run(move |c| db::get_list_persons(c)
        .map(|persons| Json(persons))).await
}

#[get("/persons/<id>")]
pub async fn get_person_id(conn: DBPool, id: i32) -> Json<Person> {
    conn.run(move |c|{
        db::get_one_person_id(c, id)
            .map(|person|Json(person))
    }).await
}

#[delete("/persons/<id>")]
pub async fn delete_person(id: i32, conn: DBPool)-> Flash<Redirect> {

    conn.run(move |c| {
        match db::get_one_person_id(c, id) {
            Ok(_)=> db::delete_one_person(c, id),
            Err(_) => Err(Error::NotFound)
        }
    }).await;
    Flash::success(Redirect::to("/persons"), "Person succesfully deleted")
}

#[get("/persons/add")]
pub async fn add_person() -> Template {

    #[derive(serde::Serialize)]
    struct OnePersonContext {
        title: String,
        person: Person,
    }

    let p= Person {
        id: None,
        first_name: "".to_string(),
        last_name: "".to_string()
    };
    Template::render("add_person", &OnePersonContext{
        title : "Ajouter une Personne".to_string(),
        person: p,
    })
}

#[post("/persons/add", data = "<person_form>")]
pub async fn new_person(person_form: Form<Person>, conn: DBPool) -> Flash<Redirect> {
    if let res = conn.run(move |c| db::create_person(c, person_form.into_inner())).await {
        Flash::success(Redirect::to("/persons"), "Person succesfully added")
    } else {
        Flash::error(Redirect::to("/persons/add"), "Person not inserted")
    }
}

#[catch(404)]
pub fn not_found(req: &Request<'_>) -> Template {
    let mut map = HashMap::new();
    map.insert("path", req.uri().path());
    Template::render("error/404", &map)
}

#[get("/about")]
pub fn about() -> Template {
    let mut map = HashMap::new();
    map.insert("title", "About");
    Template::render("about", &map)
}