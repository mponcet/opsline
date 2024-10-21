use crate::segments::{Segment, SegmentGenerator};
use crate::theme::Theme;
use crate::theme::{BackgroundColor, ForegroundColor};
use crate::Shell;
use git2::{BranchType, Repository};

pub struct GitSegment;

impl GitSegment {
    pub fn new() -> Self {
        Self {}
    }
}

impl SegmentGenerator for GitSegment {
    fn output(&self, _shell: Shell, theme: Theme) -> Option<Vec<Segment>> {
        let repo = Repository::discover(".").ok()?;
        log::info!("repository found at {}", repo.path().to_string_lossy());

        let current_branch =
            repo.branches(Some(BranchType::Local))
                .ok()
                .and_then(|mut branches| {
                    branches
                        .find(|branch| branch.as_ref().is_ok_and(|(branch, _)| branch.is_head()))
                });

        let mut branch_name = None;
        let mut upstream = None;
        let mut local = None;
        if let Some(Ok((branch, _))) = current_branch {
            upstream = branch.upstream().ok().and_then(|b| b.get().target());
            local = branch.get().target();
            if let Ok(Some(name)) = branch.name() {
                branch_name = Some(name.to_owned());
            }
        } else if let Ok(head) = repo.head() {
            // detached state
            if let Some(oid) = head.target() {
                let mut oid = oid.to_string();
                oid.truncate(7);
                branch_name = Some(oid);
            }
        }

        let (bg, fg) = match theme {
            Theme::Default => (BackgroundColor(148), ForegroundColor(0)),
            Theme::Gruvbox => (BackgroundColor(100), ForegroundColor(237)),
        };

        let mut segments = Vec::from([Segment {
            text: format!("  {} ", branch_name?).into(),
            bg,
            fg,
            blinking: false,
        }]);

        if let (Some(local), Some(upstream)) = (local, upstream) {
            if let Ok((ahead, behind)) = repo.graph_ahead_behind(local, upstream) {
                if ahead > 0 {
                    let (bg, fg) = match theme {
                        Theme::Default => (BackgroundColor(240), ForegroundColor(250)),
                        Theme::Gruvbox => (BackgroundColor(239), ForegroundColor(248)),
                    };

                    segments.push(Segment {
                        text: format!(" {}⬆ ", ahead).into(),
                        bg,
                        fg,
                        blinking: false,
                    });
                }
                if behind > 0 {
                    let (bg, fg) = match theme {
                        Theme::Default => (BackgroundColor(240), ForegroundColor(250)),
                        Theme::Gruvbox => (BackgroundColor(239), ForegroundColor(248)),
                    };

                    segments.push(Segment {
                        text: format!(" {}⬇ ", behind).into(),
                        bg,
                        fg,
                        blinking: false,
                    });
                }
            }
        }

        let mut modified = 0;
        let mut staged = 0;
        let mut untracked = 0;
        let mut conflicted = 0;
        match repo.statuses(None) {
            Ok(statuses) => {
                for status in statuses.iter() {
                    let status = status.status();
                    if status.is_wt_modified()
                        || status.is_wt_deleted()
                        || status.is_wt_typechange()
                    {
                        modified += 1;
                    } else if status.is_index_new()
                        || status.is_index_modified()
                        || status.is_index_deleted()
                        || status.is_index_renamed()
                        || status.is_index_typechange()
                    {
                        staged += 1;
                    } else if status.is_wt_new() {
                        untracked += 1;
                    } else if status.is_conflicted() {
                        conflicted += 1;
                    }
                }
            }
            Err(_) => {
                log::error!("failed to get git repository status");
            }
        }

        if staged > 0 {
            let (bg, fg) = match theme {
                Theme::Default => (BackgroundColor(22), ForegroundColor(15)),
                Theme::Gruvbox => (BackgroundColor(106), ForegroundColor(229)),
            };

            segments.push(Segment {
                text: format!(" {}✔ ", staged).into(),
                bg,
                fg,
                blinking: false,
            });
        }

        if modified > 0 {
            let (bg, fg) = match theme {
                Theme::Default => (BackgroundColor(130), ForegroundColor(15)),
                Theme::Gruvbox => (BackgroundColor(166), ForegroundColor(229)),
            };

            segments.push(Segment {
                text: format!(" {}✎ ", modified).into(),
                bg,
                fg,
                blinking: false,
            });
        }

        if untracked > 0 {
            let (bg, fg) = match theme {
                Theme::Default => (BackgroundColor(52), ForegroundColor(15)),
                Theme::Gruvbox => (BackgroundColor(88), ForegroundColor(229)),
            };

            segments.push(Segment {
                text: format!(" {}+ ", untracked).into(),
                bg,
                fg,
                blinking: false,
            });
        }

        if conflicted > 0 {
            let (bg, fg) = match theme {
                Theme::Default => (BackgroundColor(9), ForegroundColor(15)),
                Theme::Gruvbox => (BackgroundColor(124), ForegroundColor(229)),
            };

            segments.push(Segment {
                text: format!(" {}✼ ", conflicted).into(),
                bg,
                fg,
                blinking: false,
            });
        }

        Some(segments)
    }
}
