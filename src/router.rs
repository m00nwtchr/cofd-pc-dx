use dioxus::prelude::*;

use crate::components::Navbar;
#[allow(clippy::wildcard_imports)]
use crate::views::*;

#[derive(Debug, Clone, Routable, PartialEq)]
pub enum Route {
	#[layout(Navbar)]
	#[route("/")]
	Home {},
	#[route("/character/:id")]
	Sheet { id: usize },
}

// #[server(endpoint = "static_routes")]
// #[allow(clippy::unused_async)]
// async fn static_routes() -> Result<Vec<String>, ServerFnError> {
// 	// The `Routable` trait has a `static_routes` method that returns all static routes in the enum
// 	Ok(Route::static_routes()
// 		.iter()
// 		.map(ToString::to_string)
// 		.collect())
// }
