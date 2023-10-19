use futures::StreamExt;
use leptos::logging::log;
use leptos::*;

#[component]
pub fn App() -> impl IntoView {
    // TODO: pass devices found to Devices component
    let listen_action = create_action(|_: &()| async {
        let mut events = tauri_sys::event::listen::<String>("event-name")
            .await
            .unwrap();

        while let Some(value) = events.next().await {
            log!("event payload: {:?}", value);
        }
    });

    // let emit_action = create_action(cx, |_: &()| async {
    //     log!("Emitting event from frontend");
    //     tauri_sys::event::emit("event-name", &"event from frontend")
    //         .await
    //         .unwrap()
    // });

    listen_action.dispatch(());

    view! {
        <div class="navbar bg-neutral h-[64px]">
            <div class="navbar-start">
                <label for="devices-drawer" tabindex="0" class="btn drawer-button btn-ghost btn-circle">
                    <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h16M4 18h7" /></svg>
                </label>
            </div>
            // <div class="navbar-center">
            //     <a class="btn btn-ghost normal-case text-xl">coquito</a>
            // </div>
            <div class="navbar-end">
                <button class="btn btn-square btn-ghost">
                    <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" class="inline-block w-5 h-5 stroke-current"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 12h.01M12 12h.01M19 12h.01M6 12a1 1 0 11-2 0 1 1 0 012 0zm7 0a1 1 0 11-2 0 1 1 0 012 0zm7 0a1 1 0 11-2 0 1 1 0 012 0z"></path></svg>
                </button>
            </div>
        </div>
        <main class="drawer">
            <input id="devices-drawer" type="checkbox" class="drawer-toggle" />
        </main>
    }
}
