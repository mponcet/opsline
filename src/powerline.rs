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
            .filter_map(|s| s.output(self.shell, self.theme))
            .flatten()
            .collect();

        for (i, output) in segments.iter().enumerate() {
            if output.blinking {
                print!("{}", Blink.fmt(self.shell));
            }

            print!(
                r"{}{}{}{}",
                output.fg.fmt(self.shell),
                output.bg.fmt(self.shell),
                output.text,
                Reset.fmt(self.shell)
            );

            match segments.get(i + 1).map(|o| o.bg) {
                Some(next_bg) => print!(
                    r"{}{}{}",
                    ForegroundColor::from(output.bg).fmt(self.shell),
                    next_bg.fmt(self.shell),
                    Reset.fmt(self.shell)
                ),
                // last triangle: don't set background color
                None => print!(
                    r"{}{} ",
                    ForegroundColor::from(output.bg).fmt(self.shell),
                    Reset.fmt(self.shell)
                ),
            };
        }
    }
}
