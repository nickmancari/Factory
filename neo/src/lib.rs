use worker::*;
use serde::{Serialize, Deserialize};

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
		.get("/test/:number", |mut req, ctx| {
			if let Some(number) = ctx.param("number") {
				return Response::from_json(
					&Test { data: number.to_string() }
				)
			}

			Response::error("Bad Request", 400)
		})
		.get("/:year/:cat", |mut req, ctx| {
			if let (Some(year), Some(cat)) = (ctx.param("year"), ctx.param("cat")) {
				return Response::from_json(
					&MultiLevel { integer: year.to_string(), category: cat.to_string() }
				)
			}

			Response::error("Bad Request", 400)
		})
		.run(req, env)
		.await
}

