use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct File {
    pub name: String,
    pub path: String,
    pub covered_lines: u64,
    pub executable_lines: u64,
}

impl File {
    pub fn coverage(&self) -> f64 {
        if self.executable_lines > 0 {
            self.covered_lines as f64 / self.executable_lines as f64
        } else {
            0.0
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct Target {
    pub files: Vec<File>,
}

#[derive(Deserialize, Debug)]
pub struct Xccov {
    pub targets: Vec<Target>,
}
