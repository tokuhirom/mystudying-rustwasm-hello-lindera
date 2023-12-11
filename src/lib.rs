extern crate serde_json;

use lindera_analyzer::analyzer::Analyzer;
use serde_json::Value;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn analyze(text: &str, configuration: &str) -> Result<JsValue, JsValue> {
  let args = match serde_json::from_slice::<Value>(configuration.as_ref()) {
    Ok(args) => { args }
    Err(e) => {
      return Err(serde_wasm_bindgen::to_value(
        &format!("JSON deserialization error: {:?}", e))?)
    }
  };

  let analyzer = Analyzer::from_value(&args).unwrap();

  println!("text: {}", text);

  // tokenize the text
  let tokens = match analyzer.analyze(&text) {
    Ok(o) => { o}
    Err(e) => {
      return Err(serde_wasm_bindgen::to_value(&format!("Lindera error: {:?}", e))?)
    }
  };

  // JSに対応した戻り値を返すので
  Ok(serde_wasm_bindgen::to_value(&tokens)?)
}
