use rocket::{get, post, routes, http::{CookieJar, Cookie}};
use rocket_cors::CorsOptions;

#[get("/")]
fn index(jar: &CookieJar) -> &'static str {
	jar.add(Cookie::build("hello", "world").http_only(true).finish());
	"Hello, world!"
}

#[post("/set_cookies?<key>&<value>")]
fn set_cookies(jar: &CookieJar, key: String, value: String) {
	jar.add(Cookie::build(key, value)
		.http_only(true).finish())
}

fn main() {
	let cors = CorsOptions {
		allow_credentials: true,
		..Default::default()
	}.to_cors().unwrap();
	
	let rt = tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap();
	rt.block_on(async {
		rocket::build()
			.attach(cors).mount("/", routes![
			index,
			set_cookies
		]).launch().await
	}).unwrap();
}
