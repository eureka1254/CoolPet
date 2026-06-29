use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct TtsRequest {
    app: AppInfo,
    text: String,
    voice_type: String,
    format: String,
    sample_rate: i32,
}

#[derive(Serialize)]
struct AppInfo {
    appid: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TtsResult {
    pub success: bool,
    pub http_status: u16,
    pub content_type: String,
    pub response_headers: String,
    pub body_length: usize,
    pub body_preview: String,
    pub error_message: String,
    pub audio_saved: bool,
}

pub async fn synthesize(api_key: &str, app_id: &str, voice_type: &str, text: &str) -> (TtsResult, Option<Vec<u8>>) {
    let mut r = TtsResult {
        success: false, http_status: 0, content_type: String::new(),
        response_headers: String::new(), body_length: 0, body_preview: String::new(),
        error_message: String::new(), audio_saved: false,
    };

    let url = "https://openspeech.bytedance.com/api/v1/tts";

    // Hard assert: only V1 HTTP endpoint allowed
    assert_eq!(url, "https://openspeech.bytedance.com/api/v1/tts",
        "ERROR: NOT HTTP V1 TTS ENDPOINT");

    let body = TtsRequest {
        app: AppInfo { appid: app_id.to_string() },
        text: text.to_string(),
        voice_type: voice_type.to_string(),
        format: "mp3".to_string(),
        sample_rate: 24000,
    };

    println!("================ TTS REQUEST TRACE ================");
    println!("HTTP MODE = ONLY");
    println!("FINAL URL = {}", url);
    println!("FINAL METHOD = POST");
    println!("FINAL BODY = {}", serde_json::to_string_pretty(&body).unwrap_or_default());
    println!("==================================================");

    let client = match reqwest::Client::builder().build() {
        Ok(c) => c,
        Err(e) => { r.error_message = format!("Client: {}", e); return (r, None); }
    };

    let resp = match client
        .post(url)
        .header("X-Api-App-Key", api_key)
        .header("X-Api-Resource-Id", "TTS-SeedTTS2.0.822524252738")
        .header("Content-Type", "application/json")
        .json(&body)
        .send()
        .await
    {
        Ok(v) => v,
        Err(e) => { r.error_message = format!("Request: {}", e); return (r, None); }
    };

    r.http_status = resp.status().as_u16();
    r.content_type = resp.headers().get("content-type").and_then(|v| v.to_str().ok()).unwrap_or_default().to_string();
    r.response_headers = resp.headers().iter().map(|(k,v)| format!("{}: {}",k,v.to_str().unwrap_or("<bin>"))).collect::<Vec<_>>().join("\n");

    let bytes = match resp.bytes().await {
        Ok(b) => b,
        Err(e) => { r.error_message = format!("Read: {}", e); return (r, None); }
    };
    r.body_length = bytes.len();

    if bytes.is_empty() { r.error_message = "Empty".to_string(); return (r, None); }

    if r.content_type.contains("audio/mpeg") || r.content_type.contains("audio/mp3")
        || (bytes.len() >= 2 && bytes[0] == 0xFF && (bytes[1] & 0xE0) == 0xE0)
    {
        r.success = true;
        r.body_preview = format!("audio/mpeg {} bytes", bytes.len());
        return (r, Some(bytes.to_vec()));
    }

    if let Ok(json) = serde_json::from_slice::<serde_json::Value>(&bytes) {
        r.body_preview = format!("{:#}", json);
        let code = json.get("code").and_then(|v| v.as_i64()).unwrap_or(-1);
        let msg = json.get("message").and_then(|v| v.as_str()).unwrap_or("");
        r.error_message = format!("code={} {}", code, msg);
    } else {
        r.body_preview = String::from_utf8_lossy(&bytes[..bytes.len().min(2000)]).to_string();
        r.error_message = format!("content-type={}", r.content_type);
    }

    (r, None)
}
