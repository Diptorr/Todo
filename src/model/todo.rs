use std::result;

use sqlb::HasFields;
use super::db::Db;
use crate::{model, secutity::UserContx};




#[derive(sqlx::FromRow, Debug, Clone)]
pub struct Todo {
    pub id: i64,
    pub cid: i64, //create id
    pub title: String,
    pub status: TodoStatus
}  


#[derive(sqlb::Fields, Default, Debug, Clone)]
pub struct TodoPatch {
    pub title: Option<String>,
    pub status: Option<TodoStatus>
}

#[derive(sqlx::Type, Debug, Clone, PartialEq, Eq)]
#[sqlx(type_name = "todo_status_enum")]
#[sqlx(rename_all = "lowercase")]
pub enum TodoStatus {
    Open,
    Close,
}
sqlb::bindable!(TodoStatus);

pub struct TodoMac;
impl TodoMac {
    const TABLE: &'static str = "todo";
    const COLUMS: &'static [&'static str] = &["id", "cid", "title", "status"];
}
impl TodoMac {


    pub async fn create(db: &Db, utx: &UserContx, data: TodoPatch) -> Result<Todo, model::Error> {

        let mut fields = data.fields();
        fields.push(("cid", 123).into());
        let sb = sqlb::insert()
            .table(Self::TABLE).data(fields).returning(Self::COLUMS);

        let todo = sb.fetch_one(db)
        .await?;

        Ok(todo)
    }

        
pub async fn get(db: &Db, _utx: &UserContx, id: i64) -> Result<Todo, model::Error> {
    let sb = sqlb::select()
    .table(Self::TABLE).columns(Self::COLUMS).and_where_eq("id", id);

    let result = sb.fetch_one(db)
    .await;
    

    handle_fetch_one_result(result, Self::TABLE, Self::COLUMS)
}

    pub async fn update(db: &Db, utx: UserContx, id: i64, data: TodoPatch) -> Result<Todo, model::Error> {
        let sb = sqlb::update().table(Self::TABLE).and_where_eq("id", id)
        .returning(Self::COLUMS);

        let todo = sb.fetch_one(db)
        .await?;

        Ok(todo)
    }

    pub async fn list(db: &Db, _utx: &UserContx) -> Result<Vec<Todo>, model::Error> {

        let sb = sqlb::select()
        .table(Self::TABLE).columns(Self::COLUMS).order_by("!id");

        let todos = sb.fetch_all(db)
        .await?;

    Ok(todos)
    }
}

fn handle_fetch_one_result(result: Result<Todo, sqlx::Error>, typ: &'static str, id: i64) -> Result<Todo, model::Error> {
    result.map_err(|sqlx_error| 
        match sqlx_error{
            sqlx::Error::RowNotFound => model::Error::EntityNotFound(Self::TABLE, id.to_string()),
            other => model::Error::SqlxError(other)
    })
}

#[cfg(test)]
#[path = "../_test/model_todo.rs"]
mod test;