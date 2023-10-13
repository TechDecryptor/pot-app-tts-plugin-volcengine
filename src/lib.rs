use base64::{engine::general_purpose, Engine as _};
use serde_json::{json, Value};
use std::collections::HashMap;
use std::error::Error;

#[no_mangle]
pub fn tts(
    text: &str,
    lang: &str,
    needs: HashMap<String, String>,
) -> Result<Value, Box<dyn Error>> {
    let client = reqwest::blocking::ClientBuilder::new().build()?;

    let url = "https://translate.volcengine.com/crx/tts/v1/";

    let speaker = match needs.get(&format!("{lang}-speaker")) {
        Some(speaker) => speaker,
        None => match lang {
            "zh_cn" => "zh_male_xiaoming".to_string(),
            "zh_tw" => "zh_male_xiaoming".to_string(),
            "en" => "en_male_adam".to_string(),
            "ja" => "jp_male_satoshi".to_string(),
            _ => return Err("Language not supported".into()),
        },
    };
    let res:Value = client
        .post(url)
        .header("authority", "translate.volcengine.com")
        .header("origin", "chrome-extension://klgfhbdadaspgppeadghjjemk")
        .header("accept", "application/json, text/plain, */*")
        .header("sec-fetch-dest", "empty")
        .header("sec-fetch-mode", "cors")
        .header("sec-fetch-site", "none")
        .header("user-agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/106.0.0.0 Safari/537.36")
        .json(&json!({
            "text": text,
            "speaker":speaker,
        }))
        .send()?
        .json()?;

    fn parse_res(json: Value) -> Option<String> {
        let data = json
            .as_object()?
            .get("audio")?
            .as_object()?
            .get("data")?
            .as_str()?
            .to_string();
        Some(data)
    }
    let base64 = match parse_res(res) {
        Some(v) => v,
        None => return Err("Parse Response Error".into()),
    };

    let data = general_purpose::STANDARD.decode(base64)?;
    let result = data.to_vec();
    Ok(json!(result))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn try_request() {
        let mut needs = HashMap::new();
        needs.insert("zh_cn-speaker".to_string(), "zh_male_rap".to_string());
        let result = tts("你好", "zh_cn", needs).unwrap();
        println!("{result}");
    }
}
