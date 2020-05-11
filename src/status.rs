use crate::common::*;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::Debug;

pub struct Status {
    inner: RefCell<InnerStatus>,
}

impl Status {
    pub fn new() -> Self {
        Status {
            inner: RefCell::new(InnerStatus::new()),
        }
    }

    pub fn handle<T: Debug>(
        &self,
        holder: &crate::Holder<T>,
        result: Result<(), Box<dyn std::error::Error>>,
    ) {
        let mut inner = self.inner.borrow_mut();
        if inner.process_result(holder, result) {
            inner.on_change();
        }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct InnerStatus {
    last: String,
    check_summary: CheckSummary,
    checks: HashMap<String, CheckStatus>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct CheckSummary {
    total: usize,
    good: usize,
    bad: usize,
}

impl Default for CheckSummary {
    fn default() -> Self {
        CheckSummary {
            total: 0,
            good: 0,
            bad: 0,
        }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct CheckStatus {
    config: CheckConfig,
    state: StatusState,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct StatusState {
    ok: bool,
    message: String,
}

impl InnerStatus {
    pub fn new() -> Self {
        InnerStatus {
            last: format!(""),
            check_summary: Default::default(),
            checks: HashMap::new(),
        }
    }

    fn process_result<T: Debug>(
        &mut self,
        holder: &crate::Holder<T>,
        result: Result<(), Box<dyn std::error::Error>>,
    ) -> bool {
        trace!("handle {:?} {:?}", result, holder);

        let new_state = match result {
            Ok(m) => StatusState {
                ok: true,
                message: format!("ok"),
            },
            Err(e) => StatusState {
                ok: false,
                message: format!("{}", e),
            },
        };

        let existing = self.checks.get_mut(&holder.name);

        if let Some(existing) = existing {
            if existing.state != new_state {
                info!("state change {} {:?}", holder.name, new_state);
                existing.state = new_state;
                return true;
            } else {
                return false;
            }
        } else {
            info!("new {} {:?}", holder.name, new_state);
            self.checks.insert(
                holder.name.clone(),
                CheckStatus {
                    config: holder.check_config.clone(),
                    state: new_state,
                },
            );
            return true;
        }
    }

    fn on_change(&mut self) {
        let (total, good, bad) =
            self.checks
                .values()
                .fold((0, 0, 0), |(total, good, bad), check| {
                    (
                        total + 1,
                        good + if check.state.ok { 1 } else { 0 },
                        bad + if !check.state.ok { 1 } else { 0 },
                    )
                });
        self.check_summary.total = total;
        self.check_summary.good = good;
        self.check_summary.bad = bad;

        // let s = serde_yaml::to_string(&self).unwrap();
        // info!("{}", s);

        let mut l = String::new();
        if self.check_summary.bad == 0 {
            l.push_str(
                format!(
                    "All checks ok [{}/{}]",
                    self.check_summary.good, self.check_summary.total
                )
                .as_str(),
            );
        } else {
            let failures = self
                .checks
                .iter()
                .filter(|i| !i.1.state.ok)
                .map(|i| i.0.as_str())
                .collect::<Vec<_>>()
                .join(", ");

            l.push_str(
                format!(
                    "Failures: {} [{}/{}]",
                    failures, self.check_summary.good, self.check_summary.total
                )
                .as_str(),
            );
        }

        if self.last == l {
            return;
        }

        {
            use std::env;
            use std::fs::File;
            use std::io::Write;

            let path = env::home_dir().unwrap().join(".rustymon.status");
            let mut file = File::create(path).unwrap();
            file.write_all(l.as_bytes()).unwrap();
        }
        self.last = l;

        {
            use std::process::Command;
            Command::new("sh")
                .arg("-c")
                .arg("tmux refresh-client -S")
                .output()
                .expect("failed to execute process");
        }
    }
}
