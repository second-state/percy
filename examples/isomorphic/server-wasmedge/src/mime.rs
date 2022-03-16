pub struct MimeType {
    pub r#type: String,
}

impl MimeType {
    pub fn new(r#type: &str) -> Self {
        MimeType {
            r#type: r#type.to_string(),
        }
    }

    pub fn from_ext(ext: &str) -> Self {
        match ext {
            "html" => MimeType::new("text/html"),
            "css" => MimeType::new("text/css"),
            "map" => MimeType::new("application/json"),
            "js" => MimeType::new("application/javascript"),
            "json" => MimeType::new("application/json"),
            "svg" => MimeType::new("image/svg+xml"),
            "wasm" => MimeType::new("application/wasm"),
            _ => MimeType::new("text/plain"),
        }
    }

    pub fn get(self) -> String {
        self.r#type
    }
}
