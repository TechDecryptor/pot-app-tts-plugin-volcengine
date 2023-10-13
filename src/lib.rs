use serde_json::{json, Value};
use std::collections::HashMap;
use std::error::Error;
use urlencoding::encode;

#[no_mangle]
pub fn tts(
    text: &str,
    lang: &str,
    needs: HashMap<String, String>,
) -> Result<Value, Box<dyn Error>> {
    let client = reqwest::blocking::ClientBuilder::new().build()?;
    let mut url = match needs.get("requestPath") {
        Some(url) => url.to_string(),
        None => "lingva.pot-app.com".to_string(),
    };

    if !url.starts_with("http") {
        url = format!("https://{}", url);
    }

    let plain_text = text.replace("/", "%2F");
    let encode_text = encode(plain_text.as_str());

    let res = client
        .get(format!("{url}/api/v1/audio/{lang}/{encode_text}"))
        .send()?
        .bytes()?;

    let result = res.to_vec();

    Ok(json!(result))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn try_request() {
        let mut needs = HashMap::new();
        needs.insert("requestPath".to_string(), "lingva.pot-app.com".to_string());
        let result = tts("你好", "zh", needs).unwrap();
        println!("{result}");
    }
}
