use std::result;

use crate::model::db::init_db;
use super::{Todo, TodoMac};
use crate::model::todo::{TodoPatch, TodoStatus};
use crate::secutity::utx_from_token;



#[tokio::test]
async fn model_todo_create() -> Result<(), Box<dyn std::error::Error>> {
    let db = init_db()
    .await?;
    let utx = utx_from_token("123")
    .await?;
    let data_fx = TodoPatch {
        title: Some("test - model_todo_create 1".to_string()),
        ..Default::default()
    };
    let todo_created = TodoMac::create(&db, &utx, data_fx.clone())
    .await?;

    assert!(todo_created.id >= 1000, "Id should be >= 1000");
    assert_eq!(data_fx.title.unwrap(), todo_created.title);
    assert_eq!(TodoStatus::Open, todo_created.status);
    Ok(())
}

#[tokio::test]
async fn model_todo_get_good() ->Result<(), Box<dyn std::error::Error>> {
    let db = init_db()
    .await?;
    let utx = utx_from_token("123")
    .await?;

    let todo = TodoMac::get(&db, &utx, 100)
    .await?;

    assert_eq!(100, todo.id);
    assert_eq!("todo 100", todo.title);
    assert_eq!(TodoStatus::Close, todo.status);

    Ok(())
}
#[tokio::test]
async fn model_todo_get_less_good_id() ->Result<(), Box<dyn std::error::Error>> {
    let db = init_db()
    .await?;
    let utx = utx_from_token("123")
    .await?;

    let result = TodoMac::get(&db, &utx, 999)
    .await;

    match result {
        Ok(_) => assert!(false, "Ye whom shall not suceed"),
        Err(model::Error::EntityNotFound(typ, id)) => {
            assert_eq!("todo", typ);
            assert_eq!(999.to_string(), id);
        }
        other_error => assert!(false, "Wang Error {:?}", other_error), //jako pan Wnag 
    }
    Ok(())
}
#[tokio::test]
async fn model_todo_update() ->Result<(), Box<dyn std::error::Error>> {
    let db = init_db()
    .await?;
    let utx = utx_from_token("123")
    .await?;
    let data_fx = TodoPatch{title: Some("test_todo_update_good 1".to_string()),
    ..Default::default()
};
    let todo_fx = TodoMac::create(&db, data_fx.clone())
    .await?;
    let update_data_fx = TodoPatch {
        title: Some("test_todo_update_good 2".to_string()),
    ..Default::default()
    };

    let todo_update = TodoMac::update(&db, ,utx)
    .await?;

    assert_eq!(3, todos.len());
    assert_eq!(todo_fx.id, todo_update.id);
    assert_eq!(update_data_fx.title.unwrap(), todo_update.title);

    
    
    Ok(())
}

#[tokio::test]
async fn model_todo_list() -> Result<(), Box<dyn std::error::Error>> {
    let db = init_db()
    .await?;
    let utx = utx_from_token("123")
    .await?;

    let todos = TodoMac::list(&db, &utx)
    .await?;

    //cehck
    assert_eq!(2, todos.len());
    
    assert_eq!(101, todos[0].id);
    assert_eq!(123, todos[0].cid);
    assert_eq!("todo 101", todos[0].title);

    assert_eq!(100, todos[0].id);
    assert_eq!(123, todos[0].cid);
    assert_eq!("todo 101", todos[0].title);

    Ok(())
}