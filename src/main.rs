
// with this line you can import everything from rocket
// #[macro_use] extern crate rocket;
use rocket::get;
use rocket::launch;
use rocket::routes;
use rocket::serde::Serialize;
use rocket::serde::json::Json;
use rocket_db_pools::Database;
use rocket_db_pools::sqlx::PgPool;
use rocket_db_pools::Connection;
use sqlx::Row;
use sqlx::postgres::PgRow;



#[derive(Database)]
#[database("company")]
struct Db(PgPool);



#[derive(sqlx::FromRow)]
#[derive(Serialize)]
struct CompanyUser {
    pub name: String,
    pub last_name: String,
    pub age: i32
}

#[get("/")]
fn index () -> &'static str {
    "hello world!"
}


#[get("/user/<id>")]
async fn get_user(mut db: Connection<Db>, id: i64) -> Json<CompanyUser> {
    match sqlx::query("SELECT * FROM company_users WHERE user_id = $1").bind(id)
        .fetch_one(&mut **db).await    
        .and_then(| row: PgRow | Ok(CompanyUser {
            name: row.try_get("name").unwrap_or_default(),
            last_name: row.try_get("last_name").unwrap_or_default(),
            age: row.try_get("age").unwrap_or_default()
        })) 
        .ok() {
            Some(value) => Json(value),
            None => panic!("Error al buscar al usuario")
        }
}


#[get("/users")]
async fn user_list (mut db: Connection<Db>) -> Json<Vec<CompanyUser>> {
    let mut user_list: Vec<CompanyUser> = Vec::new();
    match sqlx::query("SELECT * FROM company_users")
        .fetch_all(&mut **db).await
        .and_then(|rows: Vec<PgRow>| {
            for row in &rows {
                user_list.push(CompanyUser {
                    name: row.try_get("name").unwrap_or_default(),
                    last_name: row.try_get("last_name").unwrap_or_default(),
                    age: row.try_get("age").unwrap_or_default()
                });
            }
            Ok(user_list)
        })
        .ok() {
            Some(value) => Json(value),
            None => panic!("Error al buscar al usuario")
        }
}


#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, user_list, get_user]).attach(Db::init())
}
