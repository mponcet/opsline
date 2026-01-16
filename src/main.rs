use clap::Parser;

use powerline::Powerline;
use segments::{
    ContainersSegment, CwdSegment, GitSegment, KubeSegment, NewLineSegment, ReadonlySegment,
    RootSegment, SegmentGenerator, SshSegment, TerraformSegment,
};
use shell::Shell;
use theme::Theme;

mod configuration;
mod powerline;
mod segments;
mod shell;
mod theme;
mod utils;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(long, value_parser = ["bash", "zsh"])]
    shell: String,
    #[arg(long, value_parser = ["default", "gruvbox"], default_value = "default")]
    theme: String,
    #[arg(long, default_value = "cwd,root")]
    segments: String,
    #[arg(long, default_value_t = false)]
    cwd_dironly: bool,
    #[arg(long)]
    kube_critical_contexts: Option<String>,
    #[arg(long)]
    kube_context_aliases: Option<String>,
    #[arg(long)]
    containers_url: Option<String>,
    #[arg(long)]
    terraform_critical_workspaces: Option<String>,
}

fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().filter_or("OPSLINE_LOG", "none"))
        .init();

    let args = Args::parse();

    let shell = Shell::try_from(args.shell.as_str()).expect("failed to set shell");
    let theme = Theme::try_from(args.theme.as_str()).expect("failed to set theme");

    let segments: Vec<String> = args
        .segments
        .split(',')
        .map(|s| s.trim().to_string())
        .collect();

    let config = configuration::Configuration {
        segments,
        theme: args.theme.to_string(),
        cwd: configuration::CwdConfiguration {
            dironly: args.cwd_dironly,
        },
        kube: if args.kube_critical_contexts.is_some() || args.kube_context_aliases.is_some() {
            let critical_contexts = args
                .kube_critical_contexts
                .map(|s| s.split(',').map(|s| s.trim().to_string()).collect());

            let context_aliases = args.kube_context_aliases.map(|s| {
                s.split(',')
                    .map(|pair| {
                        if let Some((lhs, rhs)) = pair.split_once(':') {
                            configuration::KubeContextAlias {
                                context: lhs.trim().to_string(),
                                alias: rhs.trim().to_string(),
                            }
                        } else {
                            panic!("Invalid context alias format: {}", pair);
                        }
                    })
                    .collect()
            });

            Some(configuration::KubeConfiguration {
                critical_contexts,
                context_aliases,
            })
        } else {
            None
        },
        containers: args
            .containers_url
            .map(|url| configuration::ContainersConfiguration { url }),
        terraform: args.terraform_critical_workspaces.map(|s| {
            configuration::TerraformConfiguration {
                critical_workspaces: s.split(',').map(|s| s.trim().to_string()).collect(),
            }
        }),
    };

    let mut powerline: Powerline = Powerline::new(shell, theme);

    for segment in config.segments {
        match segment.as_str() {
            "cwd" => powerline.add_segment(CwdSegment::new(&config.cwd)),
            "root" => powerline.add_segment(RootSegment::new()),
            "kube" => powerline.add_segment(KubeSegment::new(config.kube.as_ref())),
            "git" => powerline.add_segment(GitSegment::new()),
            "newline" => powerline.add_segment(NewLineSegment::new()),
            "ssh" => powerline.add_segment(SshSegment::new()),
            "readonly" => powerline.add_segment(ReadonlySegment::new()),
            "containers" => {
                powerline.add_segment(ContainersSegment::new(config.containers.as_ref()))
            }
            "terraform" => powerline.add_segment(TerraformSegment::new(config.terraform.as_ref())),
            s => panic!("unknown segment name: {}", s),
        }
    }

    powerline.prompt();
}
