use crate::diesel;
use diesel::prelude::*;

use actix_web::{web, HttpResponse, HttpRequest};

use super::utils::return_state;

use crate::database::establish_connection;
use crate::json_serialization::to_do_item::ToDoItem;
use crate::schema::to_do;
use crate::auth::jwt::JwtToken;
// use crate::models::item::item::Item;

pub async fn edit(to_do_item: web::Json<ToDoItem>, req: HttpRequest) -> HttpResponse {
    let title_ref: String = to_do_item.title.clone();

    let token: JwtToken = JwtToken::decode_from_request(req).unwrap();

    let connection = establish_connection();

    // asta le da update la toate, fiindca e defapt o lista aici
    let results = to_do::table.filter(to_do::columns::title
        .eq(title_ref))
        .filter(to_do::columns::user_id.eq(&token.user_id));

    // asta da update doar la primul rezultat huehue.
    // let item = to_do::table
    //     .filter(to_do::columns::title.eq(title_ref))
    //     .first::<Item>(&connection)        
    //     .unwrap();

    let db_call = diesel::update(results).set(to_do::columns::status.eq("done")).execute(&connection);
    println!("The db call: {:?}", db_call);

    return HttpResponse::Ok().json(return_state(&token.user_id));
}
