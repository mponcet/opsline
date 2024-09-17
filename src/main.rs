use clap::{command, Arg, ArgAction};
use segments::{
    cwd::CwdSegment, kube::KubeSegment, root::RootSegment, Segment, SegmentGenerator, Segments,
};
use theme::Theme;

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
            .flat_map(|s| match s {
                Segments::One(o) => {
                    Box::new(std::iter::once(o)) as Box<dyn Iterator<Item = Segment>>
                }
                Segments::Many(m) => Box::new(m.into_iter()) as Box<dyn Iterator<Item = Segment>>,
            })
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
            Arg::new("shell")
                .long("shell")
                .required(true)
                .value_parser(["bare", "bash", "zsh"]),
        )
        .arg(
            Arg::new("theme")
                .long("theme")
                .required(false)
                .default_value("default")
                .value_parser(["default"]),
        )
        .arg(
            Arg::new("segment-cwd")
                .long("segment-cwd")
                .required(false)
                .value_parser(["dironly", "full"]),
        )
        .arg(
            Arg::new("segment-root")
                .long("segment-root")
                .action(ArgAction::Count),
        )
        .arg(
            Arg::new("segment-kube")
                .long("segment-kube")
                .required(false)
                .action(ArgAction::Count),
        )
        .get_matches();

    let shell = match matches.get_one("shell").map(String::as_str) {
        Some("bare") => Shell::Bare,
        Some("bash") => Shell::Bash,
        Some("zsh") => todo!(),
        _ => unreachable!(),
    };

    let theme = match matches.get_one("theme").map(String::as_str) {
        Some("default") => Theme::Default,
        _ => unreachable!(),
    };

    let mut powerline: Powerline = Powerline::new(shell, theme);

    if let Some(mode) = matches.get_one::<String>("segment-cwd") {
        let dironly = match mode.as_str() {
            "dironly" => true,
            "full" => false,
            _ => unreachable!(),
        };
        powerline.add_segment(CwdSegment::new(dironly));
    }

    if matches.get_count("segment-kube") > 0 {
        powerline.add_segment(KubeSegment::new());
    }

    if matches.get_count("segment-root") > 0 {
        powerline.add_segment(RootSegment::new());
    }

    let ps1 = powerline.prompt();
    println!("{}", ps1);
}
