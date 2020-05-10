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
        let (total, good, bad) = self
            .checks
            .values()
            .fold((0, 0, 0), |(total, good, bad), check| {
                (total + 1, good + if check.state.ok { 1 } else { 0}, bad + if !check.state.ok { 1 } else { 0})
            });
        self.check_summary.total = total;
        self.check_summary.good = good;
        self.check_summary.bad = bad;

        let s = serde_yaml::to_string(&self).unwrap();
        info!("{}", s);
    }
}
