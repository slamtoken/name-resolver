mod constants;
mod derivation;
mod utils;

pub use derivation::resolve_domain;

use worker::*;

fn log_request(req: &Request) {
    console_log!(
        "{} - [{}], located at: {:?}, within: {}",
        Date::now().to_string(),
        req.path(),
        req.cf().coordinates().unwrap_or_default(),
        req.cf().region().unwrap_or_else(|| "unknown region".into())
    );
}

#[event(fetch)]
pub async fn main(req: Request, env: Env, _ctx: worker::Context) -> Result<Response> {
    log_request(&req);

    // Get more helpful error messages written to the console in the case of a panic.
    utils::set_panic_hook();

    let router = Router::new();

    router
        .get("/", |_, _| Response::ok(constants::HOME_MSG))
        .get_async("/:sns_name", |_, ctx| async move {
            let error_response = Response::redirect(Url::parse(constants::ERROR_URL).unwrap());
            match ctx.param("sns_name") {
                Some(sns_name) => {
                    if let Ok(response) = resolve_domain(sns_name).await {
                        Ok(response)
                    } else {
                        error_response
                    }
                }
                None => error_response,
            }
        })
        .run(req, env)
        .await
}