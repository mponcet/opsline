use config::{Config, ConfigError, File};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct KubeConfig;

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct Configuration {
    pub shell: Option<String>,
    pub segments: Option<Vec<String>>,
    pub theme: Option<String>,
    pub cwd: Option<String>,
    pub kube: Option<KubeConfig>,
}

impl Configuration {
    pub fn try_from_file(path: &str) -> Result<Configuration, ConfigError> {
        let builder = Config::builder()
            .add_source(File::with_name(path))
            .build()?;

        builder.try_deserialize()
    }
}
