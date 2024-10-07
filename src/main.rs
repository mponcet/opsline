use clap::{command, Arg};

use powerline::Powerline;
use segments::{
    containers::ContainersSegment, cwd::CwdSegment, git::GitSegment, kube::KubeSegment,
    readonly::ReadonlySegment, root::RootSegment, ssh::SshSegment, SegmentGenerator,
};
use shell::Shell;
use theme::Theme;

mod configuration;
mod fonts;
mod powerline;
mod segments;
mod shell;
mod theme;

fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("none")).init();

    let matches = command!()
        .arg(
            Arg::new("config")
                .long("config")
                .value_name("path")
                .required(false)
                .default_value("config.yaml"),
        )
        .get_matches();

    let config_path = matches.get_one::<String>("config").unwrap();
    let config = configuration::Configuration::try_from_file(config_path)
        .expect("couldn't get configuration");

    let shell =
        Shell::try_from(config.shell.as_deref().unwrap_or("auto")).expect("failed to set shell");
    let theme =
        Theme::try_from(config.theme.as_deref().unwrap_or("default")).expect("failed to set theme");

    let mut powerline: Powerline = Powerline::new(shell, theme);

    if let Some(segments) = config.segments {
        for segment in segments {
            match segment.as_str() {
                "cwd" => powerline.add_segment(CwdSegment::new(config.cwd.as_ref())),
                "root" => powerline.add_segment(RootSegment::new()),
                "kube" => powerline.add_segment(KubeSegment::new(config.kube.as_ref())),
                "git" => powerline.add_segment(GitSegment::new()),
                "ssh" => powerline.add_segment(SshSegment::new()),
                "readonly" => powerline.add_segment(ReadonlySegment::new()),
                "containers" => {
                    powerline.add_segment(ContainersSegment::new(config.containers.as_ref()))
                }
                s => panic!("unknown segment name: {}", s),
            }
        }
    }

    powerline.prompt();
}
