
use deno_bindgen::deno_bindgen;

#[deno_bindgen]
#[derive(Debug, Clone)]
pub struct UserId {
    pub(crate) inner: ruma::OwnedUserId,
}