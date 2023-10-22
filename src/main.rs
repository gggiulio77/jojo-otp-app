mod app;
mod client_step;
mod otp_step;

use leptos::*;

use crate::app::App;

fn main() {
    mount_to_body(|| {
        view! {
            <App/>
        }
    })
}
