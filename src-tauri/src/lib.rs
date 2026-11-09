use serde::{Deserialize, Serialize};
use std::path::PathBuf;

const OPENROUTER_MODEL: &str = "google/gemma-4-31b-it:free";
const OPENROUTER_API_KEY: &str = "OPENROUTER_API_KEY";

#[derive(Debug, Serialize)]
struct ChatRequest {
    model: &'static str,
    messages: Vec<Message>,
    temperature: f32,
    max_tokens: u16,
}

#[derive(Debug, Serialize)]
struct Message {
    role: &'static str,
    content: String,
}

#[derive(Debug, Deserialize)]
struct ChatResponse {
    choices: Vec<Choice>,
}

#[derive(Debug, Deserialize)]
struct Choice {
    message: ResponseMessage,
}

#[derive(Debug, Deserialize)]
struct ResponseMessage {
    content: String,
}

#[derive(Debug, Serialize)]
pub struct KannadaLesson {
    kannada: String,
    transliteration: String,
    english: String,
    explanation: String,
    practice: String,
}

#[derive(Debug, Deserialize)]
struct ModelLesson {
    kannada: String,
    transliteration: String,
    english: String,
    explanation: String,
    practice: String,
}

#[tauri::command]
async fn teach_kannada(topic: String) -> Result<KannadaLesson, String> {
    let api_key = openrouter_api_key()?;

    let prompt = format!(
        "Teach me one beginner-friendly Kannada item about: {topic}.
Return only valid JSON with these exact keys:
{{
  \"kannada\": \"...\",
  \"transliteration\": \"...\",
  \"english\": \"...\",
  \"explanation\": \"...\",
  \"practice\": \"...\"
}}
Keep it short, practical, and suitable for one small daily popup lesson."
    );

    let request = ChatRequest {
        model: OPENROUTER_MODEL,
        temperature: 0.7,
        max_tokens: 350,
        messages: vec![
            Message {
                role: "system",
                content: "You are a careful Kannada teacher. Reply with JSON only.".to_string(),
            },
            Message {
                role: "user",
                content: prompt,
            },
        ],
    };

    let client = reqwest::Client::new();
    let response = client
        .post("https://openrouter.ai/api/v1/chat/completions")
        .bearer_auth(api_key)
        .header("Content-Type", "application/json")
        .header("HTTP-Referer", "http://localhost")
        .header("X-OpenRouter-Title", "Kalthuko")
        .json(&request)
        .send()
        .await
        .map_err(|error| format!("Could not reach OpenRouter: {error}"))?;

    if !response.status().is_success() {
        let status = response.status();
        let body = response
            .text()
            .await
            .unwrap_or_else(|_| "No error body returned.".to_string());
        return Err(format!("OpenRouter returned {status}: {body}"));
    }

    let chat = response
        .json::<ChatResponse>()
        .await
        .map_err(|error| format!("Could not read OpenRouter response: {error}"))?;

    let content = chat
        .choices
        .first()
        .map(|choice| choice.message.content.trim())
        .ok_or_else(|| "OpenRouter returned no lesson.".to_string())?;

    let lesson = serde_json::from_str::<ModelLesson>(content).map_err(|error| {
        format!("The model did not return clean JSON: {error}\n\nRaw response:\n{content}")
    })?;

    Ok(KannadaLesson {
        kannada: lesson.kannada,
        transliteration: lesson.transliteration,
        english: lesson.english,
        explanation: lesson.explanation,
        practice: lesson.practice,
    })
}

fn openrouter_api_key() -> Result<String, String> {
    if let Ok(api_key) = std::env::var(OPENROUTER_API_KEY) {
        return Ok(api_key);
    }

    for env_path in candidate_env_paths() {
        if !env_path.exists() {
            continue;
        }

        let env_file = dotenvy::from_path_iter(&env_path)
            .map_err(|error| format!("Could not read {}: {error}", env_path.display()))?;

        for item in env_file {
            let (key, value) =
                item.map_err(|error| format!("Could not parse {}: {error}", env_path.display()))?;
            if key == OPENROUTER_API_KEY {
                return Ok(value);
            }
        }
    }

    Err("Set OPENROUTER_API_KEY in your shell or in the .env file one folder above Kalthuko."
        .to_string())
}

fn candidate_env_paths() -> Vec<PathBuf> {
    let mut paths = Vec::new();

    if let Ok(current_dir) = std::env::current_dir() {
        paths.push(current_dir.join("..").join(".env"));
        paths.push(current_dir.join(".env"));

        if let Some(parent) = current_dir.parent() {
            paths.push(parent.join(".env"));
        }
    }

    if let Ok(exe_path) = std::env::current_exe() {
        if let Some(exe_dir) = exe_path.parent() {
            paths.push(exe_dir.join("..").join(".env"));
        }
    }

    paths
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![teach_kannada])
        .run(tauri::generate_context!())
        .expect("error while running Kalthuko");
}
