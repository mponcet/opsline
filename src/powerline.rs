use crate::segments::SegmentGenerator;
use crate::shell::Shell;
use crate::theme::{Blink, ForegroundColor, Reset, Theme};

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

    pub fn prompt(&self) {
        let segments: Vec<_> = self
            .segments
            .iter()
            .filter_map(|s| s.output(self.shell, &self.theme))
            .flatten()
            .collect();

        for (i, segment) in segments.iter().enumerate() {
            if segment.blinking {
                print!("{}", Blink.fmt(self.shell));
            }

            if segment.name == "newline" {
                print!(r"{}{}", segment.text, Reset.fmt(self.shell));
                continue;
            } else {
                print!(
                    r"{}{}{}{}",
                    segment.fg.fmt(self.shell),
                    segment.bg.fmt(self.shell),
                    segment.text,
                    Reset.fmt(self.shell)
                );
            }

            match segments.get(i + 1) {
                Some(next_segment) if next_segment.name != "newline" => print!(
                    r"{}{}{}",
                    ForegroundColor::from(segment.bg).fmt(self.shell),
                    next_segment.bg.fmt(self.shell),
                    Reset.fmt(self.shell)
                ),
                // last triangle on the line : don't set background color
                _ => print!(
                    r"{}{} ",
                    ForegroundColor::from(segment.bg).fmt(self.shell),
                    Reset.fmt(self.shell)
                ),
            };
        }
    }
}
