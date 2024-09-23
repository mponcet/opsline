use clap::{command, Arg};
use segments::{
    cwd::CwdSegment, git::GitSegment, kube::KubeSegment, root::RootSegment, SegmentGenerator,
};
use theme::Theme;

mod configuration;
mod fonts;
mod segments;
mod theme;

struct Powerline {
    shell: Shell,
    theme: Theme,
    segments: Vec<Box<dyn SegmentGenerator + 'static>>,
}

#[derive(Clone, Copy)]
enum Shell {
    Bash,
    Zsh,
    Bare,
}

impl TryFrom<&str> for Shell {
    type Error = Box<dyn std::error::Error>;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "auto" => todo!(),
            "bash" => Ok(Shell::Bash),
            "bare" => Ok(Shell::Bare),
            "zsh" => Ok(Shell::Zsh),
            _ => Err("unknown shell".into()),
        }
    }
}

impl Powerline {
    fn new(shell: Shell, theme: Theme) -> Self {
        Self {
            shell,
            theme,
            segments: vec![],
        }
    }

    fn add_segment(&mut self, segment: impl SegmentGenerator + 'static) {
        self.segments.push(Box::new(segment));
    }

    fn prompt(&self) -> String {
        let mut ps1 = String::with_capacity(256);

        let segments: Vec<_> = self
            .segments
            .iter()
            .filter_map(|s| s.output(self.shell, &self.theme))
            .flatten()
            .collect();

        for (i, output) in segments.iter().enumerate() {
            // 38;5 => foreground color
            // 48;5 => background color
            let segment_ps1 = format!(
                r"\[\e[38;5;{}m\]\[\e[48;5;{}m\]{}{}\[\e[0m\]",
                output.fg,
                output.bg,
                if output.blinking { r"\[\e[5m\]" } else { "" },
                output.text,
            );

            let segment_triangle = match segments.get(i + 1).map(|o| o.bg) {
                Some(next_bg) => format!(
                    r"\[\e[38;5;{}m\]\[\e[48;5;{}m\]{}\[\e[0m\]",
                    output.bg,
                    next_bg,
                    fonts::NerdFonts::LEFT_HARD_DIVIDER,
                ),
                // last triangle: don't set background color
                None => format!(
                    r"\[\e[38;5;{}m\]{}\[\e[0m\] ",
                    output.bg,
                    fonts::NerdFonts::LEFT_HARD_DIVIDER
                ),
            };

            ps1.push_str(&segment_ps1);
            ps1.push_str(&segment_triangle);
        }

        ps1
    }
}

fn main() {
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
                "cwd" => {
                    let dironly = match config.cwd.as_deref().unwrap_or("full") {
                        "dironly" => true,
                        "full" => false,
                        _ => panic!("cwd must be \"full\" or \"dironly\""),
                    };
                    powerline.add_segment(CwdSegment::new(dironly));
                }

                "root" => {
                    powerline.add_segment(RootSegment::new());
                }
                "kube" => {
                    powerline.add_segment(KubeSegment::new());
                }
                "git" => {
                    powerline.add_segment(GitSegment::new());
                }
                s => panic!("unknown segment name: {}", s),
            }
        }
    }

    let ps1 = powerline.prompt();
    println!("{}", ps1);
}
