pub mod ethereum;
pub mod transaction;

use ethereum::EthereumProvider;
use js_sys::Array;
use wasm_bindgen::JsValue;

#[derive(Clone)]
pub struct Metamask {
    eth: EthereumProvider,
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("failed to init etherum provider: {0}")]
    EthreumProviderError(#[from] ethereum::Error),
    #[error("failed to request: {0}")]
    RequestError(#[from] ethereum::RequestError),
}

impl Metamask {
    pub fn from_window(window: &web_sys::Window) -> Result<Self, Error> {
        let eth = EthereumProvider::from_window(window)?;

        Ok(Metamask { eth })
    }

    pub async fn accounts(&self) -> Result<Vec<String>, Error> {
        const ETH_REQUEST_ACCOUNTS: &str = "eth_requestAccounts";

        let ctx = JsValue::default();

        let accounts = self
            .eth
            .request(&ctx, ETH_REQUEST_ACCOUNTS.to_owned(), Vec::default())
            .await?;

        let accounts = Array::from(&accounts);

        let accounts = accounts
            .iter()
            .map(|value| value.as_string().unwrap_or_default())
            .collect();

        Ok(accounts)
    }

    pub fn selected_account(&self) -> Option<String> {
        // TODO:
        self.eth.selected_account().unwrap_or_default().as_string()
    }
}
