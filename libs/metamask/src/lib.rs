pub mod ethereum;
pub mod transaction;

use ethereum::EthereumProvider;
use js_sys::Array;
pub use transaction::Transaction;
use wasm_bindgen::JsValue;
use web_sys::Window;

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

    /// Retuns addresses of accessible accounts for currently connected metamask.
    ///
    /// # Example
    ///
    /// ```rust
    /// use wasm_bindgen::UnwrapThrowExt;
    ///
    /// let window = web_sys::window().expect_throw("window is undefined");
    /// let metamask =
    ///     Metamask::from_window(&window).expect_throw("failed to create metamask client");
    /// let accounts = metamask
    ///     .accounts()
    ///     .await
    ///     .expect_throw("failed to get accounts");
    /// ```
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

    /// TODO: refactor, copy of EthereumProvider::new
    pub fn is_connected(window: &Window) -> bool {
        const ETHEREUM_PROVIDER: &'static str = "ethereum";

        let eth =
            js_sys::Reflect::get(&window, &ETHEREUM_PROVIDER.into()).unwrap_or(JsValue::UNDEFINED);

        !eth.is_undefined()
    }

    /// Send a transaction request to metamask, and return a hash of the transaction.
    ///
    /// ```rust
    /// use wasm_bindgen::UnwrapThrowExt;
    ///
    /// let window = web_sys::window().expect_throw("window is undefined");
    /// let metamask = Metamask::from_window(&window)
    ///                         .expect_throw("failed to create metamask client");
    ///
    /// let tx = Transaction {
    ///     to: "0xA1fFe6f9C12Fc0d42740A7a6d1538daA005EfBC8".to_owned(),
    ///     from: "0xA1fFe6f9C12Fc0d42740A7a6d1538daA005EfBC8".to_owned(),
    ///     data: Vec::default(),
    ///     chain_id: 5,
    /// };
    ///
    /// let tx_hash = metamask.send_transaction(&tx)
    ///                       .await
    ///                       .expect_throw("failed to send tx");
    /// ```
    pub async fn send_transaction(&self, tx: &Transaction) -> Result<String, Error> {
        const ETH_SEND_TX: &str = "eth_sendTransaction";

        let ctx = JsValue::default();

        let tx_hash = self
            .eth
            .request(
                &ctx,
                ETH_SEND_TX.to_owned(),
                vec![serde_wasm_bindgen::to_value(&tx).unwrap()],
            )
            .await?;

        Ok(tx_hash.as_string().unwrap_or_default())
    }
}
