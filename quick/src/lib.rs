use worker::*;

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


#[event(fetch)]
pub async fn main(req: Request, env: Env) -> Result<Response> {
	log_request(&req);

	utils::set_panic_hook();


}

