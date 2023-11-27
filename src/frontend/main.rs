use rocket::{get, routes};

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[shuttle_runtime::main]
async fn main() -> shuttle_rocket::ShuttleRocket {
    let rocket = rocket::build().mount("/", routes![index]);

    Ok(rocket.into())
}

// TODO: https://github.com/jbradberry/ultimate_tictactoe
// https://github.com/search?q=ultimate%20tic%20tac%20toe&type=repositories
// TODO: https://github.com/joews/rubik-js
