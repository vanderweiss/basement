use rocket::{fs::relative, fs::FileServer, *};

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", FileServer::from(relative!("static")))
}
