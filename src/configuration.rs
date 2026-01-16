use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct CwdConfiguration {
    pub dironly: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct KubeContextAlias {
    pub context: String,
    pub alias: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct KubeConfiguration {
    pub critical_contexts: Option<Vec<String>>,
    pub context_aliases: Option<Vec<KubeContextAlias>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ContainersConfiguration {
    pub url: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TerraformConfiguration {
    pub critical_workspaces: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Configuration {
    pub segments: Vec<String>,
    pub theme: String,
    pub cwd: CwdConfiguration,
    pub kube: Option<KubeConfiguration>,
    pub containers: Option<ContainersConfiguration>,
    pub terraform: Option<TerraformConfiguration>,
}

impl Default for Configuration {
    fn default() -> Self {
        Self {
            segments: vec!["cwd".into(), "root".into()],
            theme: "default".into(),
            cwd: CwdConfiguration::default(),
            kube: None,
            containers: None,
            terraform: None,
        }
    }
}
