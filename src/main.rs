use gotham::{
    helpers::http::response::create_permanent_redirect,
    hyper::{Body, Response},
    state::State,
};

/// Redirect to slack invite
fn slack_invite(state: State) -> (State, Response<Body>) {
    let resp = create_permanent_redirect(
        &state,
        "https://join.slack.com/t/buildpacks/shared_invite/zt-odkxxq0z-EqYmUbATpDLcX1c8nZCglQ",
    );

    (state, resp)
}

fn main() {
    let port = std::env::var("PORT").unwrap_or("3000".to_string());
    let addr = format!("0.0.0.0:{}", port);
    gotham::start(addr, || Ok(slack_invite))
}
