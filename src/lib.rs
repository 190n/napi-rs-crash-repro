#![deny(clippy::all)]

use napi::{
  bindgen_prelude::{FromNapiValue, Promise, Result},
  sys::{napi_env, napi_value},
  JsUnknown,
};

#[macro_use]
extern crate napi_derive;

pub struct NumberWrapper(pub u32);

impl FromNapiValue for NumberWrapper {
  unsafe fn from_napi_value(env: napi_env, napi_val: napi_value) -> napi::Result<Self> {
    Ok(Self(
      JsUnknown::from_napi_value(env, napi_val)?
        .coerce_to_number()?
        .get_uint32()?,
    ))
  }
}

#[napi]
pub async fn get(x: Promise<NumberWrapper>) -> Result<u32> {
  Ok(x.await?.0)
}
