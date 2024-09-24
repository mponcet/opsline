use crate::fonts;
use crate::segments::SegmentGenerator;
use crate::shell::Shell;
use crate::theme::Theme;

pub struct Powerline<'a> {
    shell: Shell,
    theme: Theme,
    segments: Vec<Box<dyn SegmentGenerator + 'a>>,
}

impl<'a> Powerline<'a> {
    pub fn new(shell: Shell, theme: Theme) -> Self {
        Self {
            shell,
            theme,
            segments: vec![],
        }
    }

    pub fn add_segment(&mut self, segment: impl SegmentGenerator + 'a) {
        self.segments.push(Box::new(segment));
    }

    pub fn prompt(&self) -> String {
        let mut ps1 = String::with_capacity(256);

        let segments: Vec<_> = self
            .segments
            .iter()
            .filter_map(|s| s.output(self.shell, self.theme))
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
