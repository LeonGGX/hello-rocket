use self::diesel::prelude::*;
use rocket_sync_db_pools::diesel;

use crate::models::{Genre, Partition, Person, ShowPartition};

use crate::schema::genres::columns::name;
use crate::schema::persons::columns::full_name;
use crate::schema::partitions::columns::title;
use crate::schema::{genres, partitions, persons};

use crate::DBPool;


// ***********************************************************************************************
// LISTS

pub async fn get_list_raw_partitions(conn: &DBPool) -> QueryResult<Vec<Partition>> {
    conn.run(|c| {
        partitions::table
            .order(partitions::title)
            .load::<Partition>(c)
    })
    .await
}

pub async fn get_list_show_partitions(conn: &DBPool) -> QueryResult<Vec<ShowPartition>> {
    /*
    let rep: QueryResult<Vec<ShowPartition>>  = conn.run(|c| {
        sql_query("
    SELECT partitions.id, partitions.title, persons.last_name, genres.name
    FROM partitions
    INNER JOIN persons
    ON partitions.person_id = persons.id
    INNER JOIN genres
    ON partitions.genre_id = genres.id"
        ).load::<ShowPartition>(c)
    }).await;
    rep

     */

    let data = conn
        .run(|c| {
            partitions::table
                .inner_join(persons::table)
                .inner_join(genres::table)
                .select((
                    partitions::id,
                    partitions::title,
                    persons::full_name,
                    genres::name,
                ))
                .order(partitions::title)
                .load(c)
                .expect("error")
        })
        .await;
    Ok(data)
}

pub async fn get_list_genres(conn: &DBPool) -> QueryResult<Vec<Genre>> {
    conn.run(|c| genres::table.order(name.asc()).load::<Genre>(c))
        .await
}

pub async fn get_list_persons(conn: &DBPool) -> QueryResult<Vec<Person>> {
    conn.run(|c| persons::table.order(full_name.asc()).load::<Person>(c))
        .await
}

// ************************************************************************************************
// Get vector with all occurences with same name or title

pub async fn get_person_by_name(conn: &DBPool, person_full_name: String) -> QueryResult<Person> {
    conn.run(move |c|
        persons::table
            .filter(full_name.eq(person_full_name))
            .first(c)
    ).await
}

pub async fn get_genre_by_name(conn: &DBPool, genre_name: String) -> QueryResult<Genre> {
    conn.run(move |c|
            genres::table
                .filter(name.eq(genre_name))
                .first(c)
            ).await
}

pub async fn get_raw_partition_by_title(conn: &DBPool, partition_title: String) -> QueryResult<Partition> {
    conn.run(move |c|
        partitions::table
            .filter(title.eq(partition_title))
            .first(c)
    ).await
}

pub async fn get_partition_by_title(
    conn: &DBPool,
    partition_title: String,
) -> QueryResult<ShowPartition> {

    let raw_partition = get_raw_partition_by_title(conn, partition_title.clone()).await;
    match raw_partition {
        Ok(..) => {
           let data = conn.run(|c| {
                partitions::table
                    .inner_join(persons::table)
                    .inner_join(genres::table)
                    .select((
                        partitions::id,
                        partitions::title,
                        persons::full_name,
                        genres::name,
                    ))
                    .filter(partitions::title.eq(partition_title))
                    .first(c)
                    .expect("error")
            }).await;
            Ok(data)
        }
        Err(e) => Err(e)
    }
    /*
    let data = conn.run(|c| {
        partitions::table
            .inner_join(persons::table)
            .inner_join(genres::table)
            .select((
                partitions::id,
                partitions::title,
                persons::full_name,
                genres::name,
            ))
            .filter(partitions::title.eq(partition_title))
            .first(c)
            .expect("error")
    }).await;
    Ok(data)
     */
}

pub async fn get_partition_by_author(
    conn: &DBPool,
    partition_author: String,
) -> QueryResult<Vec<ShowPartition>> {
    let pers = get_person_by_name(conn, partition_author).await.unwrap();

    let data = conn
        .run(move |c| {
            partitions::table
                .inner_join(persons::table)
                .inner_join(genres::table)
                .select((
                    partitions::id,
                    partitions::title,
                    persons::full_name,
                    genres::name,
                ))
                .filter(partitions::person_id.eq(pers.id.unwrap()))
                .load(c)
                .expect("error in finding partition by author")
        })
        .await;
    Ok(data)
}

pub async fn get_partition_by_genre(
    conn: &DBPool,
    partition_genre: String,
) -> QueryResult<Vec<ShowPartition>> {
    let genre = get_genre_by_name(conn, partition_genre).await.unwrap();

    let data = conn
        .run(move |c| {
            partitions::table
                .inner_join(persons::table)
                .inner_join(genres::table)
                .select((
                    partitions::id,
                    partitions::title,
                    persons::full_name,
                    genres::name,
                ))
                .filter(partitions::genre_id.eq(genre.id.unwrap()))
                .load(c)
                .expect("error in finding partition by genre")
        })
        .await;
    Ok(data)
}

//*************************************************************************************************
// DELETE

pub async fn delete_one_person(conn: &DBPool, person_id: i32) -> QueryResult<usize> {
    conn.run(move |c| diesel::delete(persons::table.find(person_id)).execute(c))
        .await
}

pub async fn delete_one_genre(conn: &DBPool, genre_id: i32) -> QueryResult<usize> {
    conn.run(move |c| diesel::delete(genres::table.find(genre_id)).execute(c))
        .await
}

pub async fn delete_one_partition(conn: &DBPool, partition_id: i32) -> QueryResult<usize> {
    conn.run(move |c| diesel::delete(partitions::table.find(partition_id)).execute(c))
        .await
}

//*************************************************************************************************
// CREATE

pub async fn create_person(conn: &DBPool, person: Person) -> QueryResult<Person> {
    conn.run(move |c| {
        diesel::insert_into(persons::table)
            .values(&person)
            .get_result(c)
    })
    .await
}

pub async fn create_genre(conn: &DBPool, genre: Genre) -> QueryResult<Genre> {
    conn.run(move |c| {
        diesel::insert_into(genres::table)
            .values(&genre)
            .get_result(c)
    })
    .await
}

pub async fn create_partition(
    conn: &DBPool,
    show_partition: ShowPartition,
) -> QueryResult<Partition> {
    let nom = show_partition.full_name.trim();

    let pers = get_person_by_name(conn, nom.to_string()).await?;
    println!("{:?}", pers);
    let g = get_genre_by_name(conn, show_partition.name).await?;
    println!("{:?}", g);
    let person_id = pers.id.unwrap();
    let genre_id = g.id.unwrap();

    let partition = Partition {
        id: None,
        person_id,
        title: show_partition.title,
        genre_id,
    };

    conn.run(move |c| {
        diesel::insert_into(partitions::table)
            .values(&partition)
            .get_result(c)
    })
    .await
}

//******************************************************************************************
// UPDATE

pub async fn update_person(pers_id: i32, person: Person, conn: &DBPool) -> QueryResult<Person> {
    conn.run(move |c| {
        diesel::update(persons::table.find(pers_id))
            .set(&person)
            .get_result(c)
    })
    .await
}

pub async fn update_genre(genre_id: i32, genre: Genre, conn: &DBPool) -> QueryResult<Genre> {
    conn.run(move |c| {
        diesel::update(genres::table.find(genre_id))
            .set(&genre)
            .get_result(c)
    })
    .await
}

pub async fn update_partition(
    part_id: i32,
    partition: Partition,
    conn: &DBPool,
) -> QueryResult<Partition> {
    conn.run(move |c| {
        diesel::update(partitions::table.find(part_id))
            .set(&partition)
            .get_result(c)
    })
    .await
}
