#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> String {
   return format!("Hello, world!");
}

#[get("/posts")]
fn posts() -> String {
   return format!("hello from posts");
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index,posts])
}