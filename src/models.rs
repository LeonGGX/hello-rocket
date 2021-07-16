use super::schema::*;

use diesel::sql_types::{Integer, Nullable, Text};
use diesel::{AsChangeset, Associations, Insertable, Queryable, QueryableByName};

use rocket::serde::{Deserialize, Serialize};

#[derive(
    Debug, Clone, Deserialize, Serialize, Queryable, Identifiable, AsChangeset, Insertable, FromForm,
)]
#[serde(crate = "rocket::serde")]
#[table_name = "persons"]
pub struct Person {
    #[serde(skip_deserializing)]
    pub id: Option<i32>,
    pub full_name: String,
}

impl Person {
    pub fn compare(&self, pers: &Person) -> bool {
        if self.full_name == pers.full_name {
            true
        } else {
            false
        }
    }
}

#[derive(
    Debug,
    Clone,
    Deserialize,
    Serialize,
    Queryable,
    Identifiable,
    AsChangeset,
    Insertable,
    PartialEq,
    FromForm,
)]
#[serde(crate = "rocket::serde")]
#[table_name = "genres"]
pub struct Genre {
    #[serde(skip_deserializing)]
    pub id: Option<i32>,
    pub name: String,
}

#[derive(
    Debug, Clone, Deserialize, Serialize, Queryable, Insertable, Associations, AsChangeset, FromForm,
)]
#[serde(crate = "rocket::serde")]
#[belongs_to(Person)]
#[belongs_to(Genre)]
#[table_name = "partitions"]
pub struct Partition {
    #[serde(skip_deserializing)]
    pub id: Option<i32>,
    pub person_id: i32,
    pub title: String,
    pub genre_id: i32,
}

// une struct pour présenter les partitions avec les
// éléments des différentes tables
//
// il n'y a pas de table qui correspond
// pour diesel il faut préciser le type de champ
// et utiliser QueryableByName
//
// il faut garder les mêmes noms de champs que
// dans la table d'origine dont on reprend
// les données : par ex pour le champ persons.last_name
// il faut un champ last_name et pour le champ
// genres.name il faut un champ name ici
//
#[derive(Debug, Serialize, Queryable, QueryableByName, FromForm)]
#[serde(crate = "rocket::serde")]
pub struct ShowPartition {
    #[serde(skip_deserializing)]
    #[sql_type = "Nullable<Integer>"]
    pub id: Option<i32>,
    #[sql_type = "Text"]
    pub title: String,
    #[sql_type = "Text"]
    pub full_name: String,
    #[sql_type = "Text"]
    pub name: String,
}
