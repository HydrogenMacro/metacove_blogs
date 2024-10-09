use leptos::*;
use metacove_blogs::App;

fn main() {
	// set up logging
	console_log::init_with_level(log::Level::Debug).unwrap();
	console_error_panic_hook::set_once();

	mount_to_body(|| {
		view! {
			<App />
		}
	})
}
