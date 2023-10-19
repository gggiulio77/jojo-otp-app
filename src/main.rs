mod app;
mod modal_otp;

use leptos::*;

use crate::app::App;

fn main() {
    mount_to_body(|| {
        view! {
            <App/>
        }
    })
}
