use worker::*;
use serde::{Serialize, Deserialize};
//use postgres::{Client, Error, NoTls};

mod utils;

fn log_request(req: &Request) {
	console_log!(
		"{} - [{}], located at: {:?}, within: {}",
		Date::now().to_string(),
		req.path(),
		req.cf().coordinates().unwrap_or_default(),
		req.cf().region().unwrap_or("unknown region".into())
	);
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Test {
	data: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MultiLevel {
	integer: String,
	category: String,
}

#[event(fetch)]
pub async fn main(req: Request, env: Env) -> Result<Response> {
	log_request(&req);

	utils::set_panic_hook();

	let router = Router::new();

	router
		.get("/test/:number", | req, ctx| {
			if let Some(number) = ctx.param("number") {
				return Response::from_json(
					&Test { data: number.to_string() }
				)
			}

			Response::error("Bad Request", 400)
		})
		.get("/:year/:cat", | req, ctx| {
			if let (Some(year), Some(cat)) = (ctx.param("year"), ctx.param("cat")) {
				return Response::from_json(
					&MultiLevel { integer: year.to_string(), category: cat.to_string() }
				)
			}

			Response::error("Bad Request", 400)
		})
		.get("/dbtest", |_, _| {
			return Response::ok("Test")
		})
		.run(req, env)
		.await
}
/*
pub fn db_handler(&str: test) -> String {
	let mut client = Client::connect(
		"postgresql://oscar:Acad3my 1992 Aw4rds@167.71.66.195:5432/testdb",
		NoTls,
	)?;

	for row in client.query("SELECT movie_title FROM best_picture")? {
		let movie_title: &str = row.get(0);
	}

	return movie_title.to_string()
}
*/