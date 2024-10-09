use leptos::*;
use leptos_meta::*;
use leptos_router::*;

mod components;
mod routes;

use routes::{home::Home, not_found::NotFound};
use components::error_handler::ErrorHandler;

#[component]
pub fn App() -> impl IntoView {
	// Provides context that manages stylesheets, titles, meta tags, etc.
	provide_meta_context();

	view! {
		<Html lang="en" dir="ltr" attr:data-theme="dark"/>

		// sets the document title
		<Title text="Welcome to Lept"/>

		// injects metadata in the <head> of the page
		<Meta charset="UTF-8"/>
		<Meta name="viewport" content="width=device-width, initial-scale=1.0"/>
		<ErrorHandler></ErrorHandler>
		<Router>
			<Routes>
				<Route path="/" view=Home/>
				<Route path="/*" view=NotFound/>
			</Routes>
		</Router>
	}
}
