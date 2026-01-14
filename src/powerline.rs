use crate::segments::SegmentGenerator;
use crate::shell::Shell;
use crate::theme::{Blink, Reset, Theme};

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

            print!(r"{}{}", segment.fg.fmt(self.shell), segment.text);

            if segment.name == "newline" {
                print!("{}", Reset.fmt(self.shell));
                continue;
            }

            match segments.get(i + 1) {
                Some(next_segment) if next_segment.name != "newline" => {
                    if next_segment.name != segment.name {
                        print!(
                            r"{}{}",
                            Reset.fmt(self.shell),
                            next_segment.fg.fmt(self.shell)
                        );
                    } else {
                        print!(
                            r"{}{}",
                            Reset.fmt(self.shell),
                            next_segment.fg.fmt(self.shell)
                        );
                    }
                }
                // last segment on the line
                _ => print!(r"{}{} ", Reset.fmt(self.shell), Reset.fmt(self.shell)),
            };
        }
    }
}
