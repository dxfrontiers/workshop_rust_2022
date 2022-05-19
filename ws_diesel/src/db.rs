use actix_web::{ web};
use diesel::{prelude::*, r2d2::PooledConnection};

use crate::model::{Person, NewPerson};
use diesel::r2d2::{self, ConnectionManager};

type DbError = Box<dyn std::error::Error +'static>;
pub type DbPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;


pub async fn insert_person(pool: &DbPool, new_person: NewPerson) -> Result<Person, DbError> {
    use crate::schema::people::dsl::*;
    let conn = get_conn(pool).await?;
    web::block(move || {
        diesel::insert_into(people)
            .values(&new_person)
            .execute(&conn)?;
        let new_id : i32 = diesel::select(last_insert_rowid).get_result::<i32>(&conn)?;
        people.filter(id.eq(new_id)).first::<Person>(&conn)
    }).await?
    .map_err(|e| e.into())
}



pub async fn get_person_by_id(pool: &DbPool, selected_id: i32) -> Result<Option<Person>, DbError> {
    use crate::schema::people::dsl::*;
    let conn = get_conn(pool).await?;
    web::block(move ||      
        people.filter(id.eq(selected_id))        
            .first::<Person>(&conn)
            .optional()
    )
    .await?
    .map_err(|e| e.into())
}

pub async fn get_person_all(pool: &DbPool) -> Result<Vec<Person>, DbError> {
    use crate::schema::people::dsl::*;
    let conn = get_conn(pool).await?;
    web::block(move || people.load::<Person>(&conn))
        .await?
        .map_err(|e| e.into())
}

async fn get_conn(pool: &DbPool) -> Result<PooledConnection<ConnectionManager<SqliteConnection>>,DbError>{
    let pool = pool.clone();
    web::block(move || pool.get())
        .await?
        .map_err(|e| e.into())
}

// credit goes to https://github.com/diesel-rs/diesel/issues/771
no_arg_sql_function!(
    last_insert_rowid,
    diesel::sql_types::Integer,
    "Represents the SQL last_insert_row() function"
);

