use crate::Shell;
use crate::segments::{Segment, SegmentGenerator};
use crate::theme::Theme;
use git2::{BranchType, Repository};

pub struct GitSegment;

impl GitSegment {
    pub fn new() -> Self {
        Self {}
    }
}

impl SegmentGenerator for GitSegment {
    fn output(&self, _shell: Shell, theme: &Theme) -> Option<Vec<Segment>> {
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

        let mut segments = Vec::from([Segment {
            name: "git",
            text: format!("  {} ", branch_name?).into(),
            bg: theme.git_branch_bg,
            fg: theme.git_branch_fg,
            blinking: false,
        }]);

        if let (Some(local), Some(upstream)) = (local, upstream)
            && let Ok((ahead, behind)) = repo.graph_ahead_behind(local, upstream)
        {
            if ahead > 0 {
                segments.push(Segment {
                    name: "git",
                    text: format!("{}⬆ ", ahead).into(),
                    bg: theme.git_ahead_bg,
                    fg: theme.git_ahead_fg,
                    blinking: false,
                });
            }
            if behind > 0 {
                segments.push(Segment {
                    name: "git",
                    text: format!("{}⬇ ", behind).into(),
                    bg: theme.git_behind_bg,
                    fg: theme.git_behind_fg,
                    blinking: false,
                });
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
            segments.push(Segment {
                name: "git",
                text: format!("{}✔ ", staged).into(),
                bg: theme.git_staged_bg,
                fg: theme.git_staged_fg,
                blinking: false,
            });
        }

        if modified > 0 {
            segments.push(Segment {
                name: "git",
                text: format!("{}✎ ", modified).into(),
                bg: theme.git_modified_bg,
                fg: theme.git_modified_fg,
                blinking: false,
            });
        }

        if untracked > 0 {
            segments.push(Segment {
                name: "git",
                text: format!("{}+ ", untracked).into(),
                bg: theme.git_untracked_bg,
                fg: theme.git_untracked_fg,
                blinking: false,
            });
        }

        if conflicted > 0 {
            segments.push(Segment {
                name: "git",
                text: format!("{}✼ ", conflicted).into(),
                bg: theme.git_conflicted_bg,
                fg: theme.git_conflicted_fg,
                blinking: false,
            });
        }

        Some(segments)
    }
}
