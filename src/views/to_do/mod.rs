mod create;
mod edit;
mod get;
mod delete;
mod utils;

use super::path::Path;
use actix_web::web;

pub fn item_factory(app: &mut web::ServiceConfig) {
    app.route(path("/create/{title}").as_ref(), web::post().to(create::create));
    app.route(path("/get").as_ref(), web::get().to(get::get));
    app.route(path("/edit").as_ref(), web::put().to(edit::edit));
    app.route(path("/delete").as_ref(), web::delete().to(delete::delete));
}

fn path(following_path: &str) -> String {
    let base_path: Path = Path {
        prefix: String::from("/item"),
        backend: true
    };

    base_path.define(String::from(following_path))
}
