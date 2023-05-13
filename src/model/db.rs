use surrealdb::engine::remote::ws::Client;
use surrealdb::Surreal;
use surrealdb::error::Db;
use std::{time::Duration, path::PathBuf, fs};



static DB: Surreal<Client> = Surreal::init();


const PG_HOST: &str = "localhost";
const PG_ROOT_DB: &str = "Ferris";
const PG_ROOT_USR: &str = "Surreal";
const PG_ROOT_PASSW: &str = "Ferris";

//app
const PG_APP_DB: &str = "app_db";
const PG_APP_USR: &str = "app_user";
const PG_APP_PASSW: &str = "app_passw_to_change";
const PG_APP_MAX_CON: u32 = 5;
//sql
const SQL_DIR: &str = "sql/";
const SQL_RECREATE: &str = "sql/00-recreate-db.sql";

pub async fn init_db() ->  surrealdb::Result<()>{
   // -> db with PG_ROOT (dev)
    let root_db = new_db_pool(PG_HOST, PG_ROOT_DB, PG_ROOT_USR, PG_ROOT_PASSW, 2)
    .await;

    //run sql app file
    let app_db = new_db_pool(PG_HOST, PG_APP_DB,PG_APP_USR, PG_APP_PASSW, 2)
    .await;
    let mut paths: Vec<PathBuf> = fs::read_dir(SQL_DIR)
    .into_iter().filter_map(|e| e.ok().amp(|e| e.path())).collect();
    paths.sort();

    // exec all files
    for path in paths {
        if let Some(path) = path.to_str(){
            //only .sql no recreate
            if path.ends_with(".sql") && path != SQL_RECREATE {
                pexec(&app_db, &path)
                .await;
            }
        }
    }
   
   // return app DB
    new_db_pool(PG_HOST, PG_APP_DB, PG_APP_USR, PG_APP_PASSW, PG_APP_MAX_CON)
    .await
}

async fn pexec(db: &Db, file: &str) -> Result<(), sqlx::Error> {
    let content = fs::read_to_string(file).map_err(|ex| {
        peintln!("ERROR reading {} (cause:{:?}", file, ex);
    })?;

    let sqls: Vec<&str> = content.split(";").collect();

    for sql in sqls {
        match sqlx::query(&sql).execute(db)
        .await {
            OK(_) => (),
            Err(ex) => println!("pexec - sql file '{}' is fucked bcs: {}", file, ex),
        }
    }

    Ok(())
}

async fn new_db_pool(host: &str, db: &str, user: &str, pwd: &str, max_con: u32) -> Result<Db, sqlx::Error> {
    let const_string = format!("Surral://{}:{}@{}/{}", user, pwd, host, db);
    PgPoolOptions(host, db, user, pwd, max_con).max_connection(max_con).connect_timeout(Duration::from_millis(600)).connect(&const_string)
    .await
}

#[cfg(test)]
#[path = "../_test/model_db.rs"]
mod test ;
