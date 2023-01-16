mod components;

use log::Level;
use metamask::Metamask;
use monaco::api::TextModel;
use std::ops::Deref;
use wasm_bindgen::UnwrapThrowExt;
use wasm_bindgen_futures::spawn_local;
use web_sys::window;
use yew::prelude::*;

use crate::components::{editor::MarkdownEditor, metamask::MetamaskButton};

#[function_component]
fn App() -> Html {
    let current = use_state(|| "".to_owned());

    let onclick = {
        let current = current.clone(); // copy inside callback

        Callback::from(move |_event: MouseEvent| {
            let current = current.clone(); // copy inside async block

            spawn_local(async move {
                let window = window().expect_throw("window is undefined");

                let metamask =
                    Metamask::from_window(&window).expect_throw("failed to create metamask client");

                let accounts = metamask
                    .accounts()
                    .await
                    .expect_throw("failed to get accounts");

                // FIXME:
                current.set(
                    accounts
                        .first()
                        .expect_throw("no accounts found")
                        .to_string(),
                );
            });
        })
    };

    let text =
        use_state_eq(|| TextModel::create("# Hello, world!", Some("markdown"), None).unwrap());

    let create_task_cb = Callback::from(|_event: MouseEvent| {
        spawn_local(async move {
            let window = window().expect_throw("window is undefined");

            let metamask =
                Metamask::from_window(&window).expect_throw("failed to create metamask client");

            let tx = metamask::Transaction {
                to: "0xA1fFe6f9C12Fc0d42740A7a6d1538daA005EfBC8".to_owned(),
                from: "0xd35c0a2d081493467196A01769B63616F8D8805f".to_owned(),
                data: "0x".to_owned(),
                chain_id: 5,
            };

            let tx_hash = metamask
                .send_transaction(&tx)
                .await
                .expect_throw("failed to send tx");

            log::info!("{}", tx_hash);
        });
    });

    html! {
        <div>
            <MetamaskButton
                user_address={current.deref().clone()}
                {onclick}
            />
            <MarkdownEditor text_model={text.deref().clone()} />
            <button onclick={create_task_cb}>
                {"Create task"}
            </button>
        </div>
    }
}

fn main() {
    console_log::init_with_level(Level::Debug).unwrap();
    yew::Renderer::<App>::new().render();
}
