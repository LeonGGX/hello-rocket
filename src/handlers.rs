use std::collections::HashMap;

use rocket::form::Form;
use rocket::response::{Flash, Redirect};
use rocket::serde::Serialize;
use rocket::Request;

use rocket_dyn_templates::Template;

use crate::db::{get_list_genres, get_list_persons};
use crate::models::{Genre, Person, ShowPartition, Partition};
use crate::{db, DBPool};

// Context : pour affichage général
//
#[derive(Debug, Serialize)]
struct Context {
    flash: Option<(String, String)>,
    title: String,
    partitions: Vec<ShowPartition>,
    persons: Vec<Person>,
    genres: Vec<Genre>,
}

impl Context {
    pub async fn err<M: std::fmt::Display>(conn: &DBPool, msg: M) -> Context {
        Context {
            flash: Some(("error".into(), msg.to_string())),
            persons: vec![],
            genres: vec![],
            title: "Erreur !".to_string(),
            partitions: vec![],
        }
    }

    pub async fn raw_pers(conn: &DBPool, flash: Option<(String, String)>) -> Context {
        match db::get_list_persons(conn).await {
            Ok(persons) => Context {
                flash,
                persons,
                genres: vec![],
                title: "Liste des Personnes".to_string(),
                partitions: vec![],
            },
            Err(e) => {
                error_!("DB get_list_persons error: {}", e);
                Context {
                    flash: Some(("error".into(), "Fail to access database.".into())),
                    persons: vec![],
                    genres: vec![],
                    title: "".to_string(),
                    partitions: vec![],
                }
            }
        }
    }

    pub async fn raw_genres(conn: &DBPool, flash: Option<(String, String)>) -> Context {
        match db::get_list_genres(conn).await {
            Ok(genres) => Context {
                flash,
                persons: vec![],
                genres,
                title: "Liste des Genres".to_string(),
                partitions: vec![],
            },
            Err(e) => {
                error_!("DB get_list_genres error: {}", e);
                Context {
                    flash: Some(("error".into(), "Fail to access database.".into())),
                    persons: vec![],
                    genres: vec![],
                    title: "".to_string(),
                    partitions: vec![],
                }
            }
        }
    }

    pub async fn raw_partitions(conn: &DBPool, flash: Option<(String, String)>) -> Context {
        let persons = db::get_list_persons(conn).await.unwrap();
        let genres = db::get_list_genres(conn).await.unwrap();

        match db::get_list_show_partitions(conn).await {
            Ok(show_part) => Context {
                flash,
                persons,
                genres,
                title: "Liste des Partitions".to_string(),
                partitions: show_part,
            },
            Err(e) => {
                error_!("DB get_list_show_partitions error: {}", e);
                Context {
                    flash: Some(("error".into(), "Fail to access database.".into())),
                    persons,
                    genres,
                    title: "".to_string(),
                    partitions: vec![],
                }
            }
        }
    }
}

//********************************************
// GET all pages

#[get("/")]
pub fn start() -> Template {
    #[derive(serde::Serialize)]
    struct StartContext {
        title: String,
    }
    let context = StartContext {
        title: "Start".to_string(),
    };
    Template::render("start", &context)
}

#[get("/genres")]
pub async fn all_genres(conn: DBPool) -> Template {
    let flash = Some(("pas d'erreur".into(), "liste des genres trouvée".into()));
    Template::render("genres", Context::raw_genres(&conn, flash).await)
}

#[get("/persons")]
pub async fn all_persons(conn: DBPool) -> Template {
    let flash = Some(("pas d'erreur".into(), "liste des personnes trouvée".into()));
    Template::render("persons", Context::raw_pers(&conn, flash).await)
}

#[get("/partitions")]
pub async fn all_partitions(conn: DBPool) -> Template {
    let flash = Some(("pas d'erreur".into(), "liste des partitions trouvée".into()));
    Template::render("partitions", Context::raw_partitions(&conn, flash).await)
}

// ********************************************************************************************
// Handles DELETE operations
//

#[delete("/persons/<id>")]
pub async fn delete_person(id: i32, conn: DBPool) -> Result<Flash<Redirect>, Template> {
    match db::delete_one_person(&conn, id).await {
        Ok(_) => Ok(Flash::success(
            Redirect::to("/persons"),
            "Person successfully deleted",
        )),
        Err(e) => {
            error_!("DB deletion({}) error: {}", id, e);
            Err(Template::render(
                "persons",
                Context::err(&conn, "Failed to delete person.").await,
            ))
        }
    }
}

#[delete("/genres/<id>")]
pub async fn delete_genre(id: i32, conn: DBPool) -> Result<Flash<Redirect>, Template> {
   match db::delete_one_genre(&conn, id).await{
       Ok(_) => Ok(Flash::success(
           Redirect::to("/genres"),
           "Genre successfully deleted",
       )),
       Err(e) => {
           error_!("DB deletion({}) error: {}", id, e);
           Err(Template::render(
               "genres",
               Context::err(&conn, "Failed to delete genre.").await,
           ))
       }
   }
}

#[delete("/partitions/<id>")]
pub async fn delete_partition(id: i32, conn: DBPool) -> Result<Flash<Redirect>, Template> {
    match db::delete_one_partition(&conn, id).await {
        Ok(_) => Ok(Flash::success(
            Redirect::to("/partitions"),
            "Partition successfully deleted",
        )),
        Err(e) => {
            error_!("DB deletion({}) error: {}", id, e);
            Err(Template::render(
                "partitions",
                Context::err(&conn, "Failed to delete partition.").await,
            ))
        }
    }
}

// ********************************************************************************************
// Handles ADD operations
//

#[post("/persons/add", data = "<person_form>")]
pub async fn new_person(person_form: Form<Person>, conn: DBPool) -> Flash<Redirect> {
    let person = person_form.into_inner();
    /*
    let comp = db::get_person_by_name(&conn, person.full_name.clone()).await.unwrap();

    if comp.is_empty() {
        db::create_person(&conn, person).await.unwrap();
        Flash::success(Redirect::to("/persons"), "Person successfully added.")
    } else {
        // comparer toutes les instances pour voir si une est la même
        let mut res: usize = 0;
        for pers in comp {
            if person.compare(&pers) {
                res += 1;
            }
        }
        if res != 0 {
            Flash::error(Redirect::to("/persons"), "Person already exists")
        } else {
            db::create_person(&conn, person).await.unwrap();
            Flash::success(Redirect::to("/persons"), "Person successfully added.")
        }
    }

     */
    db::create_person(&conn, person).await.unwrap();
    Flash::success(Redirect::to("/persons"), "Person successfully added.")
}

#[post("/genres/add", data = "<genre_form>")]
pub async fn new_genre(genre_form: Form<Genre>, conn: DBPool) -> Flash<Redirect> {
    let genre = genre_form.into_inner();
    db::create_genre(&conn, genre).await.unwrap();
    Flash::success(Redirect::to("/genres"), "Genre successfully added.")

    /*
    let comp = db::get_person_by_name(&conn, person.last_name.clone()).await.unwrap();

    if comp.is_empty() {
        db::create_person(&conn, person).await.unwrap();
        Flash::success(Redirect::to("/persons"), "Person successfully added.")
    } else {
        // comparer toutes les instances pour voir si une est la même
        let mut res: usize = 0;
        for pers in comp {
            if person.compare(&pers) {
                res += 1;
            }
        }
        if res != 0 {
            Flash::error(Redirect::to("/persons"), "Person already exists")
        } else {
            db::create_person(&conn, person).await.unwrap();
            Flash::success(Redirect::to("/persons"), "Person successfully added.")
        }
    }
    */
}

#[post("/partitions/add", data = "<partition_form>")]
pub async fn new_partition(partition_form: Form<ShowPartition>, conn: DBPool) -> Flash<Redirect> {
    let data = partition_form.into_inner();
    println!("ShowPartition from partitions/add : {:?}", data);

    db::create_partition(&conn, data).await.unwrap();
    Flash::success(Redirect::to("/partitions"), "Partition successfully added.")
}

// ********************************************************************************************
// Handles UPDATE operations
//

#[put("/persons/<id>", data = "<person_form>")]
pub async fn update_person(id: i32, person_form: Form<Person>, conn: DBPool) -> Flash<Redirect> {
    let person = person_form.into_inner();
    println!("{:?}", person);

    db::update_person(id, person, &conn).await.unwrap();
    Flash::success(Redirect::to("/persons"), "Person successfully modified.")
}

#[put("/genres/<id>", data = "<genre_form>")]
pub async fn update_genre(id: i32, genre_form: Form<Genre>, conn: DBPool) -> Flash<Redirect> {
    let genre = genre_form.into_inner();
    println!("{:?}", genre);

    db::update_genre(id, genre, &conn).await.unwrap();
    Flash::success(Redirect::to("/genres"), "Genre successfully modified.")
}

#[put("/partitions/<id>", data = "<show_partition_form>")]
pub async fn update_partition(
    id: i32,
    show_partition_form: Form<ShowPartition>,
    conn: DBPool,
) -> Flash<Redirect> {
    let show_partition = show_partition_form.into_inner();

    let partition_id = id;
    let musician = db::get_person_by_name(&conn, show_partition.full_name).await.unwrap();
    let musician_id = musician.id.unwrap();
    let genre = db::get_genre_by_name(&conn, show_partition.name).await.unwrap();
    let genre_id = genre.id.unwrap();

    let partition = Partition {
        id: Some(partition_id),
        person_id: musician_id,
        title: show_partition.title,
        genre_id,
    };
    println!("{:?}", partition);
    db::update_partition(id, partition, &conn).await.unwrap();
    Flash::success(Redirect::to("/partitions"), "Partition successfully modified.")
}

//*************************************************************************************************
// Handles RESEARCH operations
//

#[post("/persons/find", data = "<name>")]
pub async fn get_person_by_name(name: Form<String>, conn: DBPool) -> Template {

    let n = name.into_inner();
    match db::get_person_by_name(&conn, n).await {
        Ok(p) => {
            println!("{:?}", p);
            let mut vec_pers: Vec<Person> = Vec::new();
            vec_pers.push(p);
            let flash = Some(("pas d'erreur".into(), "Personne trouvée".into()));

            let context = Context {
                flash,
                persons: vec_pers,
                genres: vec![],
                title: "Personne trouvée".to_string(),
                partitions: vec![],
            };
            Template::render("persons", &context)
        }
        Err(e) => {
            let flash = Some(("Erreur".into(), e.to_string()));
            let context = Context {
                flash,
                persons: vec![],
                genres: vec![],
                title: "Erreur !".to_string(),
                partitions: vec![],
            };
            Template::render("persons", &context)
        }
    }
}

#[post("/genres/find", data = "<name>")]
pub async fn get_genre_by_type(name: Form<String>, conn: DBPool) -> Template {

    let name =name.into_inner();
    match db::get_genre_by_name(&conn, name).await {
        Ok(g) => {
            println!("{:?}", g);
            let mut vec_genres: Vec<Genre> = Vec::new();
            vec_genres.push(g);
            let flash = Some(("pas d'erreur".into(), "Genre trouvé".into()));

            let context = Context {
                flash,
                persons: vec![],
                genres: vec_genres,
                title: "Genre trouvé".to_string(),
                partitions: vec![],
            };
            Template::render("genres", &context)
        }
        Err(e) => {
            let flash = Some(("Erreur".into(), e.to_string()));
            let context = Context {
                flash,
                persons: vec![],
                genres: vec![],
                title: "Erreur !".to_string(),
                partitions: vec![],
            };
            Template::render("persons", &context)
        }
    }
}

#[post("/partitions/find/title", data = "<title>")]
pub async fn get_partition_by_title(title: Form<String>, conn: DBPool) -> Template {

    let title = title.into_inner();
    let persons = get_list_persons(&conn).await.unwrap();
    let genres = get_list_genres(&conn).await.unwrap();

    match db::get_partition_by_title(&conn, title).await {
        Ok(p) => {
            let mut vec_p: Vec<ShowPartition> = Vec::new();
            vec_p.push(p);
            if vec_p.len() != 0 {
                let flash = Some(("pas d'erreur".into(), "Partition trouvée".into()));
                let context = Context {
                    flash,
                    persons,
                    genres,
                    title: "Partition trouvée".to_string(),
                    partitions: vec_p,
                };
                Template::render("partitions", &context)
            } else {
                let flash = Some(("pas d'erreur".into(), "Aucune partition trouvée".into()));
                let context = Context {
                    flash,
                    persons: vec![],
                    genres: vec![],
                    title: "Aucune Partition trouvée".to_string(),
                    partitions: vec![],
                };
                Template::render("partitions", &context)
            }
        }
        Err(e) => {
            let flash = Some(("Erreur".into(), e.to_string()));
            let context = Context {
                flash,
                persons: vec![],
                genres: vec![],
                title: "Erreur !".to_string(),
                partitions: vec![],
            };
            Template::render("persons", &context)
        }
    }
}

#[post("/partitions/find/author", data = "<author>")]
pub async fn get_partition_by_author(author: Form<String>, conn: DBPool) -> Template {

    let persons = get_list_persons(&conn).await.unwrap();
    let genres = get_list_genres(&conn).await.unwrap();
    let author = author.into_inner();
    let partitions = db::get_partition_by_author(&conn, author).await.unwrap();
    let flash = Some(("pas d'erreur".into(), "partition trouvée".into()));
    let context = Context {
        flash,
        persons,
        genres,
        title: "Aucune Partition trouvée".to_string(),
        partitions,
    };
    Template::render("partitions", &context)
}

#[post("/partitions/find/genre", data = "<genre>")]
pub async fn get_partition_by_genre(genre: Form<String>, conn: DBPool) -> Template {

    let persons = get_list_persons(&conn).await.unwrap();
    let genres = get_list_genres(&conn).await.unwrap();
    let genre = genre.into_inner();
    let partitions = db::get_partition_by_genre(&conn, genre).await.unwrap();
    let flash = Some(("pas d'erreur".into(), "partition trouvée".into()));
    let context = Context {
        flash,
        persons,
        genres,
        title: "Aucune Partition trouvée".to_string(),
        partitions,
    };
    Template::render("partitions", &context)
}

//*************************************************************************************************
// various
//
#[catch(404)]
pub fn not_found(req: &Request<'_>) -> Template {
    let mut map = HashMap::new();
    map.insert("path", req.uri().path().raw());
    Template::render("error/404", &map)
}

#[get("/about")]
pub fn about() -> Template {
    let mut map = HashMap::new();
    map.insert("title", "A propos de ...");
    Template::render("about", &map)
}
