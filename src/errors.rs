/// Generic error wrapping `napi::Error`.
// #[derive(Debug)]
// pub struct Error(napi::Error);

pub type Result<T> = std::result::Result<T, DenoError>;

// this code was copied from the nodejs bindings.
// perhaps unecessary, for now it mayt be convenient to express everything
// in 'matrix-sdk-deno' errors in case ~other errors~ need to be handled from other libs or something
#[derive(Debug, Clone)]
pub struct DenoError {
  pub status: i32,
  pub reason: String,
  // Convert raw `JsError` into Error
  // Only be used in `async fn(p: Promise<T>)` scenario
  // #[cfg(all(feature = "tokio_rt", feature = "napi4"))]
  // pub(crate) maybe_raw: sys::napi_ref,
}

impl<E> From<E> for DenoError
where
    E: std::error::Error,
{
    fn from(error: E) -> Self {
        // convert from lib error (deno bindings or something, napi did this)
        let error_from_reason = DenoError::from_reason_internal(error.to_string());
        return Self {
            status: 0,
            reason: error_from_reason.reason,
        };
    }
}

impl DenoError {
    // this is made public to simulate an error type created by something like napi (ie deno_bindgen)
    fn from_reason_internal(reason: String) -> DenoError {
        DenoError { 
            status: 0,
            reason: String::from(reason) 
        }
    }

    pub fn from_reason(reason: &str) -> DenoError {
        Self::from_reason_internal(reason.to_string())
    }
}

// impl From<DenoError> {
//     #[inline]
//     fn from(value: DenoError) -> Self {
//         value.0
//     }
// }

/// Helper to replace the `E` to `Error` to `napi::Error` conversion.
#[inline]
pub fn into_err<E>(error: E) -> DenoError
where
    E: std::error::Error,
{
    DenoError::from(error).into()
}

// pub type Result = Result<Self, Error>;

