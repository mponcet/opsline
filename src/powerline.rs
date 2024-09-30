use crate::fonts;
use crate::segments::SegmentGenerator;
use crate::shell::Shell;
use crate::theme::{BackgroundColor, Blink, ForegroundColor, Reset, Theme};

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
            let segment_ps1 = format!(
                r"{}{}{}{}{}",
                output.fg.fmt(self.shell),
                output.bg.fmt(self.shell),
                if output.blinking {
                    Box::new(Blink.fmt(self.shell)) as Box<dyn std::fmt::Display>
                } else {
                    Box::new("".to_string())
                },
                output.text,
                Reset.fmt(self.shell)
            );

            let segment_triangle = match segments.get(i + 1).map(|o| o.bg) {
                Some(next_bg) => format!(
                    r"{}{}{}{}",
                    unsafe { std::mem::transmute::<BackgroundColor, ForegroundColor>(output.bg) }
                        .fmt(self.shell),
                    next_bg.fmt(self.shell),
                    fonts::NerdFonts::LEFT_HARD_DIVIDER,
                    Reset.fmt(self.shell)
                ),
                // last triangle: don't set background color
                None => format!(
                    r"{}{}{} ",
                    unsafe { std::mem::transmute::<BackgroundColor, ForegroundColor>(output.bg) }
                        .fmt(self.shell),
                    fonts::NerdFonts::LEFT_HARD_DIVIDER,
                    Reset.fmt(self.shell)
                ),
            };

            ps1.push_str(&segment_ps1);
            ps1.push_str(&segment_triangle);
        }

        ps1
    }
}
