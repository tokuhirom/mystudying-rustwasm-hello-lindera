extern crate serde_json;

use lindera_analyzer::analyzer::Analyzer;
use serde_json::json;
use wasm_bindgen::prelude::*;


#[wasm_bindgen]
pub fn analyze(text: &str) -> Result<JsValue, JsValue> {
    let analyzer = Analyzer::from_value(&json!({
  "character_filters": [
    {
      "kind": "unicode_normalize",
      "args": {
        "kind": "nfkc"
      }
    },
    {
      "kind": "japanese_iteration_mark",
      "args": {
        "normalize_kanji": true,
        "normalize_kana": true
      }
    }
  ],
  "tokenizer": {
    "dictionary": {
      "kind": "ipadic"
    },
    "mode": "normal"
  },
  "token_filters": [
    {
      "kind": "japanese_katakana_stem",
      "args": {
        "min": 3
      }
    }
  ]
        })).unwrap();

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
