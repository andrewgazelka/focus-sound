use std::path::PathBuf;

#[derive(serde::Serialize, serde::Deserialize, Default)]
pub struct State {
    pub volumes: [f32; 4],
}

fn state_path() -> Option<PathBuf> {
    dirs::home_dir().map(|h| h.join(".sound.json"))
}

impl State {
    pub fn load() -> Self {
        state_path()
            .and_then(|p| std::fs::read_to_string(p).ok())
            .and_then(|s| serde_json::from_str(&s).ok())
            .unwrap_or_default()
    }

    pub fn save(&self) {
        if let Some(path) = state_path() {
            let _ = std::fs::write(path, serde_json::to_string(self).unwrap_or_default());
        }
    }
}
