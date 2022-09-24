use rocket::serde::{json::Json, Deserialize};
use rocket::http::Status;
use rocket::response::{content, status};

#[macro_use] extern crate rocket;

#[get("/get/hello_world")]
fn index() -> &'static str {
    "Hello, world!"
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
struct Request {
    string: String,
}

#[post("/post/hello_world", format = "json", data="<request>")]
fn new(request: Json<Request>) ->  status::Custom<content::RawJson<&'static str>> {    
     if request.string == *"Hello world!" {
        return status::Custom(Status::Accepted, content::RawJson("{ \"success\": true }"));
     }
     status::Custom(Status::BadRequest, content::RawJson("{ \"success\": false }"))
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, new])
}