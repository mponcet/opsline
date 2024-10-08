use config::{Config, ConfigError, File};
use serde::Deserialize;

#[derive(Debug, Default, Deserialize)]
#[allow(unused)]
pub struct CwdConfiguration {
    pub dironly: bool,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct KubeContextAlias {
    pub context: String,
    pub alias: String,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct KubeConfiguration {
    pub critical_contexts: Option<Vec<String>>,
    pub context_aliases: Option<Vec<KubeContextAlias>>,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct ContainersConfiguration {
    pub url: String,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct Configuration {
    pub shell: String,
    pub segments: Option<Vec<String>>,
    pub theme: Option<String>,
    pub cwd: Option<CwdConfiguration>,
    pub kube: Option<KubeConfiguration>,
    pub containers: Option<ContainersConfiguration>,
}

impl Configuration {
    pub fn try_from_file(path: &str) -> Result<Configuration, ConfigError> {
        let builder = Config::builder()
            .add_source(File::with_name(path))
            .build()?;

        builder.try_deserialize()
    }
}
