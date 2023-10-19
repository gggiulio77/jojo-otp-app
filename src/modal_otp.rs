use leptos::logging::{error, log};
use leptos::{
    ev::SubmitEvent,
    html::{Form, Input, Select},
    *,
};

const SELECT_DEFAULT: &'static str = "Pick your network";

async fn scan_network() -> Vec<jojo_common::network::Ssid> {
    // TODO: implement a retry logic
    match tauri_sys::tauri::invoke::<_, jojo_common::otp::ScanResponse>("scan", &()).await {
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

async fn send_network_credentials(
    credentials: jojo_common::network::NetworkCredentials,
) -> Result<(), String> {
    match tauri_sys::tauri::invoke::<jojo_common::network::NetworkCredentials, ()>(
        "save_credentials",
        &credentials,
    )
    .await
    {
        Ok(_) => {
            log!("[send_network_credentials]: Ok");
            Ok(())
        }
        Err(err) => {
            error!("[send_network_credentials]: something went wrong {:?}", err);
            Err("[send_network_credentials]: something went wrong".to_string())
        }
    }
}

#[component]
pub fn ModalOtp() -> impl IntoView {
    let (_networks, _set_networks) = create_signal(Vec::<String>::new());

    // TODO: replace all this shit with controlled inputs
    // TODO: search if is possible to list and select a wifi to connected

    let ssid_element: NodeRef<Select> = create_node_ref();
    let password_element: NodeRef<Input> = create_node_ref();
    let form_element: NodeRef<Form> = create_node_ref();

    let async_scan = create_resource(|| (), |_| async move { scan_network().await });

    let save_credentials = create_action(
        move |credentials: &jojo_common::network::NetworkCredentials| {
            let input = credentials.clone();

            async move { send_network_credentials(input).await }
        },
    );

    let (ssid, set_ssid) = create_signal(jojo_common::network::Ssid::default());
    let (password, set_password) = create_signal(jojo_common::network::Password::default());

    let on_submit = move |ev: SubmitEvent| {
        ev.prevent_default();

        let credentials = jojo_common::network::NetworkCredentials::new(
            ssid_element.get().unwrap().value().try_into().unwrap(),
            password_element.get().unwrap().value().try_into().unwrap(),
        );

        log!("Sending to backend, {:?}", credentials);

        save_credentials.dispatch(credentials);
    };

    view! {
        <div class="modal-box">
            <form method="dialog">
                <button class="btn btn-sm btn-circle btn-ghost absolute right-2 top-2">"✕"</button>
            </form>
            <Suspense fallback=move || view! { <p>"Loading..."</p>}>
                <form node_ref=form_element class="form-control w-full max-w-xs" on:submit=on_submit>
                    <div>
                        <label class="label">
                            <span class="label-text">"Network Name"</span>
                        </label>

                        <select disabled=move || {save_credentials.pending().get()} class="select select-bordered w-full max-w-xs" node_ref=ssid_element on:input=move |ev| {
                            set_ssid.set(event_target_value(&ev).try_into().unwrap());
                        }>
                            <option disabled selected>{SELECT_DEFAULT}</option>
                            {move || {
                                async_scan.get().map(|networks| {
                                    networks.into_iter().map(|ssid| {
                                        view! { <option>{ssid.to_string()}</option>}
                                    }).collect_view()
                                })
                            }}
                        </select>
                    </div>
                    <div>
                        <label class="label">
                            <span class="label-text">"Password"</span>
                        </label>
                        <input node_ref=password_element disabled=move || {save_credentials.pending().get()} type="text" placeholder="Type here" class="input input-bordered w-full max-w-xs" on:input=move |ev| {
                            set_password.set(event_target_value(&ev).try_into().unwrap());
                        } />
                    </div>

                    <div class="mt-6 flex items-center justify-end gap-x-6">
                        <button type="button" disabled=move || {save_credentials.pending().get()} on:click=move |_ev| {
                            set_ssid.set(jojo_common::network::Ssid::default());
                            set_password.set(jojo_common::network::Password::default());
                            form_element.get().unwrap().reset() } class="btn btn-outline">Cancel</button>
                        <button type="submit"
                        // disabled=move || { (ssid.get().is_default() || password.get().is_empty()) || save_credentials.pending().get() }
                        on:click=move |_ev| {  }
                        class="btn btn-accent rounded-md px-5 py-2 text-sm font-semibold  shadow-sm  focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2">
                            Save
                        </button>
                    </div>
                </form>
            </Suspense>
        </div>

    }
}
