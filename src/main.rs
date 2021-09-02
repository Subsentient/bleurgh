#![forbid(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_macros)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate lazy_static;
mod layouts;
mod common;
mod config;

use rocket::response::content::Html;

#[get("/", format = "html")]
fn derp() -> Html<String>
{
	Html(layouts::default_header("Lick my vinegary nutsack") + "Uuuuuurp" + &layouts::default_footer())
}

#[launch]
fn rocket() -> _
{
	rocket::build().mount("/", routes![derp])
}
