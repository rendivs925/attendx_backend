use serde_json::Value;
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Lang {
    En,
    Id,
    De,
    Jp,
}

impl Lang {
    pub fn from_code(code: &str) -> Self {
        match code.to_ascii_lowercase().as_str() {
            "id" => Self::Id,
            "de" => Self::De,
            "jp" => Self::Jp,
            "en" => Self::En,
            _ => Self::En,
        }
    }
}

fn load_message_file(lang: Lang, namespace: &str) -> Value {
    let lang_folder = match lang {
        Lang::En => "en",
        Lang::De => "de",
        Lang::Id => "id",
        Lang::Jp => "ja",
    };

    let file_path = Path::new("locales")
        .join(lang_folder)
        .join(format!("{namespace}.json"));

    match fs::read_to_string(&file_path) {
        Ok(content) => match serde_json::from_str(&content) {
            Ok(json) => {
                println!("[DEBUG] Loaded messages from {:?}", file_path);
                json
            }
            Err(err) => {
                eprintln!("[ERROR] Failed to parse JSON from {:?}: {}", file_path, err);
                Value::Null
            }
        },
        Err(err) => {
            eprintln!("[ERROR] Failed to read file {:?}: {}", file_path, err);
            Value::Null
        }
    }
}

#[derive(Debug, Clone)]
pub enum Namespace {
    Validation,
    User,
}

impl Namespace {
    fn as_str(&self) -> &'static str {
        match self {
            Namespace::Validation => "validation",
            Namespace::User => "user",
        }
    }
}

#[derive(Debug)]
pub struct Messages {
    pub user: Value,
    pub validation: Value,
}

impl Messages {
    pub fn new(lang: Lang) -> Self {
        Self {
            user: load_message_file(lang, "user"),
            validation: load_message_file(lang, "validation"),
        }
    }

    pub fn get(&self, namespace: &Namespace, path: &str) -> Option<&Value> {
        let root = match namespace {
            Namespace::User => &self.user,
            Namespace::Validation => &self.validation,
        };

        let mut current = root;
        for key in path.split('.') {
            match current.get(key) {
                Some(next) => {
                    current = next;
                }
                None => {
                    eprintln!(
                        "[DEBUG] Key '{}' not found in path '{}.{}'",
                        key,
                        namespace.as_str(),
                        path
                    );
                    return None;
                }
            }
        }

        println!(
            "[DEBUG] Found message for '{}.{}'",
            namespace.as_str(),
            path
        );
        Some(current)
    }

    pub fn get_str(&self, namespace: Namespace, path: &str, fallback: &str) -> String {
        let result = self
            .get(&namespace, path)
            .and_then(Value::as_str)
            .unwrap_or(fallback)
            .to_string();

        log::debug!(
            "🔍 Accessed message [{}::{}]: {}",
            namespace.as_str(),
            path,
            result
        );
        result
    }
}

pub fn get_lang(req: &actix_web::HttpRequest) -> Lang {
    req.headers()
        .get("Accept-Language")
        .and_then(|value| value.to_str().ok())
        .and_then(|header| {
            header
                .split(',')
                .next()
                .and_then(|tag| tag.split('-').next())
        })
        .map(Lang::from_code)
        .unwrap_or(Lang::De)
}
