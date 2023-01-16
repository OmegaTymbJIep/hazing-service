use web_sys::MouseEvent;
use yew::{function_component, html, Callback, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct ButtonProps {
    pub user_address: String,
    pub onclick: Callback<MouseEvent, ()>,
}

#[function_component(MetamaskButton)]
pub fn metamask_button(
    ButtonProps {
        user_address,
        onclick,
    }: &ButtonProps,
) -> Html {
    html! {
        <button {onclick}>
            {
                if user_address.is_empty() {
                    "Connect Metamask!".to_owned()
                } else {
                    user_address.clone()
                }
            }
        </button>
    }
}
