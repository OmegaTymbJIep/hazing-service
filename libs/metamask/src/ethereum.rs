use js_sys::{Array, Function};
use wasm_bindgen::{JsCast, JsValue, UnwrapThrowExt};
use wasm_bindgen_futures::JsFuture;

pub struct EthereumProvider {
    raw: JsValue,
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("no ethereum provider")]
    NoEthereumProvider,
}

#[derive(thiserror::Error, Debug)]
pub enum RequestError {
    #[error("no request method")]
    NoRequestMethod,
    #[error("failed to convert to value to function")]
    ConvertionError,
    #[error("failed to call request method")]
    CallError,
    #[error("failed to await request")]
    FutureError,
}

impl EthereumProvider {
    pub fn from_window(window: &web_sys::Window) -> Result<Self, Error> {
        const ETHEREUM_PROVIDER: &'static str = "ethereum";

        let eth = js_sys::Reflect::get(&window, &ETHEREUM_PROVIDER.into())
            .map_err(|_| Error::NoEthereumProvider)?;

        Ok(Self { raw: eth })
    }

    pub async fn request(
        &self,
        ctx: &JsValue,
        method: String,
        params: Vec<JsValue>,
    ) -> Result<JsValue, RequestError> {
        const METAMASK_REQUEST_METHOD: &str = "request";

        let request_method = js_sys::Reflect::get(&self.raw, &METAMASK_REQUEST_METHOD.into())
            .map_err(|_| RequestError::NoRequestMethod)?;

        let request_method = request_method
            .dyn_ref::<js_sys::Function>()
            .ok_or(RequestError::ConvertionError)?;

        let params = Self::request_params_to_js_value(method, params);

        let promise =
            Function::call1(request_method, &ctx, &params).map_err(|_| RequestError::CallError)?;
        let promise = js_sys::Promise::resolve(&promise);

        let accounts = JsFuture::from(promise)
            .await
            .map_err(|_| RequestError::FutureError)?;

        Ok(accounts)
    }

    // TODO: handle errors
    fn request_params_to_js_value(method: String, params: Vec<JsValue>) -> JsValue {
        let params_js_array = Array::new();
        for param in params {
            params_js_array.push(&param);
        }

        let params = JsValue::default();
        js_sys::Reflect::set(&params, &"method".into(), &method.into())
            .expect_throw("failed to add method property");

        js_sys::Reflect::set(&params, &"params".into(), &params_js_array)
            .expect_throw("failed to add method property");

        params
    }
}
