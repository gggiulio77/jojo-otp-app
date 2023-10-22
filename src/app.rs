use crate::client_step;
use crate::otp_step;
use gloo_timers::callback::Timeout;
use leptos::logging::log;
use leptos::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <div class="hero min-h-screen bg-base-200">
            <Router>
                <Routes>
                    <Route path="/" view= move || {
                        view! { <client_step::ClientStep on_success=move || {
                            log!("Successfully connected");
                            let timeout = Timeout::new(500, move || {
                                let navigate = use_navigate();
                                navigate("/otp_client", Default::default());
                            });
                            timeout.forget();
                        }/> }
                    }/>
                    <Route path="/otp_client" view= move || {
                        view! { <otp_step::OtpStep on_success=move || {
                            log!("Successfully saved");
                            let navigate = use_navigate();
                            navigate("/finish", Default::default());
                        }/> }
                    }/>
                    <Route path="/finish" view= move || {
                        view! { <a>"Now you can restart your device and close this window"</a> }
                    }/>
                </Routes>
            </Router>
        </div>
    }
}
