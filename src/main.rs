use anyhow::anyhow;
use gotham::{
    helpers::http::response::create_permanent_redirect,
    hyper::{Body, Response},
    middleware::state::StateMiddleware,
    pipeline,
    router::{
        builder::{self, DefineSingleRoute, DrawRoutes},
        Router,
    },
    state::{FromState, State},
};
use gotham_derive::StateData;

#[derive(Clone, StateData)]
/// `StateData` to hold the Slack Redirect URL
struct RedirectUrl {
    inner: String,
}

impl RedirectUrl {
    pub fn new(url: impl Into<String>) -> Self {
        Self { inner: url.into() }
    }

    pub fn value(&self) -> &str {
        &self.inner
    }
}

/// Instantiate Redirect URL from env var and redirect all traffic to the REDIRECT_URL
fn router() -> anyhow::Result<Router> {
    let redirect_url = RedirectUrl::new(
        std::env::var("REDIRECT_URL")
            .map_err(|_| anyhow!("REDIRECT_URL environment variable not found."))?,
    );
    let middleware = StateMiddleware::new(redirect_url);
    let pipeline = pipeline::single_middleware(middleware);
    let (chain, pipelines) = pipeline::single::single_pipeline(pipeline);

    Ok(builder::build_router(chain, pipelines, |route| {
        route.get("/").to(slack_invite);
    }))
}

/// Permanent Redirect from Redirect URL from State Middleware
fn slack_invite(state: State) -> (State, Response<Body>) {
    let url = {
        let state_data = RedirectUrl::borrow_from(&state);
        state_data.value().to_string()
    };
    let resp = create_permanent_redirect(&state, url);

    (state, resp)
}

fn main() -> anyhow::Result<()> {
    let port = std::env::var("PORT").unwrap_or("3000".to_string());
    let addr = format!("0.0.0.0:{}", port);
    let router = router()?;
    gotham::start(addr, router);

    Ok(())
}
