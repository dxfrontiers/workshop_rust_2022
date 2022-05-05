use actix_web::{error, web, Error};
use r2d2::PooledConnection;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::{OptionalExtension};


use crate::model::{Person, NewPerson};

pub type Pool = r2d2::Pool<r2d2_sqlite::SqliteConnectionManager>;
pub type Connection = r2d2::PooledConnection<r2d2_sqlite::SqliteConnectionManager>;


async fn get_conn(pool: &Pool) -> Result<PooledConnection<SqliteConnectionManager>,Error>{
    let pool = pool.clone();
    web::block(move || pool.get())
        .await?
        .map_err(error::ErrorInternalServerError)
}


/*
 */
pub async fn get_person_by_id(pool: &Pool, id: u32) -> Result<Option<Person>, Error> {
    let conn = get_conn(&pool).await?;
     web::block( move ||get_person_by_id_query(conn, id))
        .await?
        .map_err(error::ErrorInternalServerError)
}

pub async fn get_person_by_age(pool: &Pool, id: u32) -> Result<Vec<Person>, Error> {
    let conn = get_conn(&pool).await?;
     web::block( move ||get_persons_by_age_query(conn, id))
        .await?
        .map_err(error::ErrorInternalServerError)
}

/*
 */
pub async fn insert_person(pool: &Pool, person: NewPerson) -> Result<Person, Error> {    
    let conn = get_conn(&pool).await?;
    web::block(move || insert_person_query(conn, person))
    .await?
    .map_err(error::ErrorInternalServerError)
}


/*
    Get one person by their ID, if they exist
 */
fn get_person_by_id_query(conn: Connection, id: u32) -> Result<Option<Person>, rusqlite::Error> {
    conn.prepare(
        "
        SELECT id, first_name, last_name, age
            FROM people
            WHERE id == :id
        ",
    )?
    .query_row(&[(":id", &id)], |row| {
        Ok(Person {
            id: row.get(0)?,
            first_name: row.get(1)?,
            last_name: row.get(2)?,
            age: row.get(3)?,
        })
    }).optional()
}



/*
    Insert one person and return them
    Since sqlite does not support returning, do an extra query
 */
fn insert_person_query(conn: Connection, person: NewPerson) -> Result<Person, rusqlite::Error> {
    match conn.execute(
        "
        INSERT INTO people (first_name, last_name, age) 
        VALUES(:fn, :ln, :age)
        ",&[
            (":fn", &person.first_name),
            (":ln", &person.last_name),
            (":age", &person.age.to_string())
        ]
    ){
        Ok(updated) => {
            println!("{} rows were updated", updated);
            let id = conn.last_insert_rowid();
            get_person_by_id_query(conn,id as u32)?
            .ok_or(rusqlite::Error::QueryReturnedNoRows)
        },
        Err(err) => {
            println!("update failed: {}", err);
            Err(err)
        },
    }
}

/*
    TODO
 */
fn get_persons_by_age_query(conn: Connection, age: u32) -> Result<Vec<Person>, rusqlite::Error> {
    conn.prepare(
        "
        SELECT id, first_name, last_name
            FROM people
            WHERE age == :age
        ",
    )?
    .query_map(&[(":age", &age)], |row| {
        Ok(Person {
            id: row.get(0)?,
            first_name: row.get(1)?,
            last_name: row.get(2)?,
            age,
        })
    }).and_then(Iterator::collect)
}