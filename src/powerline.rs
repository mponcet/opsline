use crate::segments::{Segment, SegmentSection};
use crate::shell::Shell;
use crate::theme::{Blink, Reset, Theme};
use std::time::Instant;
use tracing::debug;

pub struct Powerline<'a> {
    shell: Shell,
    theme: Theme,
    segments: Vec<Box<dyn Segment + 'a>>,
}

impl<'a> Powerline<'a> {
    pub fn new(shell: Shell, theme: Theme) -> Self {
        Self {
            shell,
            theme,
            segments: vec![],
        }
    }

    pub fn add_segment(&mut self, segment: impl Segment + 'a) {
        self.segments.push(Box::new(segment));
    }

    pub fn prompt(&self) {
        let sections: Vec<_> = self
            .segments
            .iter()
            .filter_map(|s| {
                let start = Instant::now();
                let mut sections = s.output(self.shell, &self.theme);
                let duration = start.elapsed();
                debug!(segment = s.name(), duration = ?duration, "segment completed");

                if let Some(ref mut sections) = sections {
                    sections.push(SegmentSection::Seperator);
                }
                sections
            })
            .flatten()
            .collect();

        for (i, section) in sections.iter().enumerate() {
            match section {
                SegmentSection::Section {
                    text,
                    bg,
                    fg,
                    blinking,
                } => {
                    if *blinking {
                        print!("{}", Blink.fmt(self.shell));
                    }
                    print!(
                        r"{}{}{}{}",
                        bg.fmt(self.shell),
                        fg.fmt(self.shell),
                        text,
                        Reset.fmt(self.shell)
                    );
                }
                SegmentSection::Seperator => {
                    if i == sections.len() - 1 {
                        print!(r"{}", Reset.fmt(self.shell),);
                    } else {
                        print!(r"{}", Reset.fmt(self.shell),);
                    }
                }
            }
        }
    }
}
