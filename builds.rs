use futures::{StreamExt, stream::FuturesUnordered};
use reqwest::Client;
use serde_json::{Map, Value, json};
use std::{
    collections::BTreeMap,
    fs,
    path::{Path, PathBuf},
};
use tokio;

const TARGET_LANGS: [&str; 3] = ["de", "id", "ja"];
const SOURCE_DIR: &str = "locales/en";
const OUTPUT_DIR: &str = "locales";

async fn fetch_translation(
    client: &Client,
    text: &str,
    target_lang: &str,
) -> Result<String, reqwest::Error> {
    let url = "http://localhost:5000/translate";
    let payload = json!({
        "q": text,
        "source": "en",
        "target": target_lang,
        "format": "text"
    });

    let res = client
        .post(url)
        .header("Content-Type", "application/json")
        .json(&payload)
        .send()
        .await?;

    let body: serde_json::Value = res.json().await?;
    Ok(body["translatedText"].as_str().unwrap_or(text).to_string())
}

fn flatten_json(value: &Value, prefix: String, map: &mut BTreeMap<String, String>) {
    match value {
        Value::Object(obj) => {
            for (k, v) in obj {
                let new_prefix = if prefix.is_empty() {
                    k.to_string()
                } else {
                    format!("{}.{}", prefix, k)
                };
                flatten_json(v, new_prefix, map);
            }
        }
        Value::String(s) => {
            map.insert(prefix, s.clone());
        }
        _ => {}
    }
}

fn unflatten_json(flat: &BTreeMap<String, String>) -> Value {
    let mut root = Map::new();

    for (key, val) in flat {
        let parts: Vec<&str> = key.split('.').collect();
        let mut current = &mut root;

        for i in 0..parts.len() {
            if i == parts.len() - 1 {
                current.insert(parts[i].to_string(), Value::String(val.clone()));
            } else {
                current = current
                    .entry(parts[i])
                    .or_insert_with(|| Value::Object(Map::new()))
                    .as_object_mut()
                    .unwrap();
            }
        }
    }

    Value::Object(root)
}

async fn translate_file(
    client: &Client,
    file_path: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    let file_content = fs::read_to_string(file_path)?;
    let json: Value = serde_json::from_str(&file_content)?;
    let mut flat_map = BTreeMap::new();
    flatten_json(&json, "".to_string(), &mut flat_map);

    let mut translations: BTreeMap<&str, BTreeMap<String, String>> = TARGET_LANGS
        .iter()
        .map(|&lang| (lang, BTreeMap::new()))
        .collect();

    let mut futures = FuturesUnordered::new();

    for (key, val) in &flat_map {
        for &lang in &TARGET_LANGS {
            let client = client.clone();
            let val = val.clone();
            let key = key.clone();
            futures.push(async move {
                let translated = fetch_translation(&client, &val, lang).await.unwrap_or(val);
                (lang, key, translated)
            });
        }
    }

    while let Some((lang, key, val)) = futures.next().await {
        translations.get_mut(lang).unwrap().insert(key, val);
    }

    for &lang in &TARGET_LANGS {
        let flat = &translations[lang];
        let reconstructed = unflatten_json(flat);
        let relative = file_path.strip_prefix(SOURCE_DIR)?;
        let out_path = Path::new(OUTPUT_DIR).join(lang).join(relative);
        if let Some(parent) = out_path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(out_path, serde_json::to_string_pretty(&reconstructed)?)?;
    }

    Ok(())
}

fn find_json_files(dir: &str) -> Vec<PathBuf> {
    let mut files = vec![];
    for entry in walkdir::WalkDir::new(dir) {
        let entry = entry.unwrap();
        if entry.path().is_file()
            && entry.path().extension().and_then(|s| s.to_str()) == Some("json")
        {
            files.push(entry.into_path());
        }
    }
    files
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let files = find_json_files(SOURCE_DIR);

    for file in files {
        println!("Translating {:?}", file);
        translate_file(&client, &file).await?;
    }

    println!("All translations saved to locales/[de,id,ja]/");
    Ok(())
}
