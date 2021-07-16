table! {
    persons (id) {
        id -> Nullable<Integer>,
        full_name -> Varchar,
    }
}

table! {
    genres (id) {
        id -> Nullable<Integer>,
        name -> Varchar,
    }
}

table! {
    partitions (id) {
        id -> Nullable<Integer>,
        person_id -> Integer,
        title -> Varchar,
        genre_id -> Integer,
    }
}

allow_tables_to_appear_in_same_query!(partitions, genres, persons);
joinable!(partitions -> genres(genre_id));
joinable!(partitions -> persons(person_id));
