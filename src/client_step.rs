use leptos::logging::error;
use leptos::logging::log;
use leptos::*;

async fn scan_client() -> Vec<jojo_common::network::Ssid> {
    // TODO: implement a retry logic
    match tauri_sys::tauri::invoke::<_, jojo_common::otp::ScanResponse>("scan_client", &()).await {
        Ok(scan_result) => {
            log!("[scan_network]: {:?}", scan_result);
            scan_result.found_ssid().clone()
        }
        Err(err) => {
            // TODO: think what to do in error
            error!("[scan_network]: something went wrong {:?}", err);
            vec![jojo_common::network::Ssid::try_from("Network 1".to_string()).unwrap()]
        }
    }
}

// TODO: implement a retry logic
async fn connect_client(ssid: String, password: String) -> anyhow::Result<()> {
    let credentials = jojo_common::network::NetworkCredentials::new(
        ssid.try_into().unwrap(),
        password.try_into().unwrap(),
    );

    match tauri_sys::tauri::invoke::<jojo_common::network::NetworkCredentials, ()>(
        "connect_client",
        &credentials,
    )
    .await
    {
        Ok(_) => Ok(()),
        Err(err) => {
            // TODO: think what to do in error
            error!("[connect_client]: something went wrong {:?}", err);
            Err(anyhow::Error::new(err))
        }
    }
}

#[component]
pub fn ClientStep<F>(on_success: F) -> impl IntoView
where
    F: Fn() + 'static + Clone,
{
    // TODO: think about using primitive types like String and numbers in the front
    // TODO: think about impl IntoProperty for Ssid struct
    let (ssid, set_ssid) = create_signal("".to_string());
    let (password, set_password) = create_signal("".to_string());
    let (error, set_error) = create_signal::<Option<String>>(None);
    let (disabled, set_disabled) = create_signal(false);

    // TODO: replace credential with NetworkCredential
    let connect_action = create_action(move |credential: &(String, String)| {
        let (ssid, password) = credential.clone();
        log!("Action: connect to client, {:?}:{:?}", ssid, password);

        let on_success = on_success.clone();

        async move {
            match connect_client(ssid, password).await {
                Ok(_) => on_success(),
                Err(_) => todo!(),
            }
        }
    });

    let ssid_resources = create_resource(|| (), |_| async move { scan_client().await });

    let dispatch_action = move || {
        log!(
            "Dispatching connect action with values {:?}:{:?}",
            ssid.get(),
            password.get()
        );
        connect_action.dispatch((ssid.get(), password.get()));
    };

    let button_is_disabled = Signal::derive(move || {
        disabled.get() || password.get().is_empty() || ssid.get().is_empty()
    });

    view! {
        <Suspense fallback=move || view! { <p>"Loading..."</p>}>
            <form class="form-control w-full max-w-xs" on:submit=|ev| ev.prevent_default()>
                {move || {
                    error
                        .get()
                        .map(|err| {
                            view! { <p style="color:red;">{err}</p> }
                        })
                }}
                <div class="py-5">
                    <label class="label">
                        <span class="label-text">"Network Name"</span>
                    </label>
                    <select
                        prop:disabled=move || disabled.get()
                        required
                        class="select select-bordered w-full max-w-xs"
                        on:input=move |ev| {
                            set_ssid.update(|v| *v = event_target_value(&ev));
                        }>
                        <option disabled selected>"Pick your network"</option>
                        {move || {
                            ssid_resources.get().map(|networks| {
                                networks.into_iter().map(|ssid| {
                                    view! { <option>{ssid.to_string()}</option>}
                                }).collect_view()
                            })
                        }}
                    </select>
                </div>
                <div class="py-5">
                    <label class="label">
                        <span class="label-text">"Password"</span>
                    </label>
                    <input
                        type="password"
                        required
                        placeholder="Password"
                        class="input input-bordered w-full max-w-xs"
                        prop:disabled=move || disabled.get()
                        on:keyup=move |ev: ev::KeyboardEvent| {
                            match &*ev.key() {
                                "Enter" => {
                                    dispatch_action();
                                }
                                _ => {
                                    let val = event_target_value(&ev);
                                    set_password.update(|p| *p = val);
                                }
                            }
                        }
                        on:change=move |ev| {
                            let val = event_target_value(&ev);
                            set_password.update(|p| *p = val);
                        }
                    />
                </div>
                <div class="py-10">
                    <button
                        prop:disabled=move || button_is_disabled.get()
                        on:click=move |_| dispatch_action()
                        class="btn btn-accent w-full rounded-md px-5 py-2 text-sm font-semibold  shadow-sm  focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2"
                    >
                    "Connect"
                    </button>
                </div>
            </form>
        </Suspense>
    }
}
