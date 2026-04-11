use crate::db::HistoryDb;

pub enum NavigateResult {
    Entry(String),
    Original(String),
    AtBottom,
}

pub struct HistoryNavigator {
    entries: Vec<String>,
    index: Option<usize>,
    saved_input: String,
    stale: bool,
}

impl Default for HistoryNavigator {
    fn default() -> Self {
        Self {
            entries: Vec::new(),
            index: None,
            saved_input: String::new(),
            stale: true,
        }
    }
}

impl HistoryNavigator {
    pub fn navigate_up(&mut self, db: &HistoryDb, current_input: &str) -> Option<String> {
        if self.stale {
            self.entries = match db.search("", 500) {
                Ok(records) => records.into_iter().map(|r| r.command).collect(),
                Err(_) => return None,
            };
            self.stale = false;
        }

        if self.entries.is_empty() {
            return None;
        }

        match self.index {
            None => {
                self.saved_input = current_input.to_string();
                self.index = Some(0);
            }
            Some(i) if i + 1 < self.entries.len() => {
                self.index = Some(i + 1);
            }
            _ => return None,
        }

        self.entries.get(self.index.unwrap()).cloned()
    }

    pub fn navigate_down(&mut self) -> NavigateResult {
        match self.index {
            None => NavigateResult::AtBottom,
            Some(0) => {
                self.index = None;
                NavigateResult::Original(self.saved_input.clone())
            }
            Some(i) => {
                self.index = Some(i - 1);
                match self.entries.get(i - 1) {
                    Some(cmd) => NavigateResult::Entry(cmd.clone()),
                    None => NavigateResult::AtBottom,
                }
            }
        }
    }

    pub fn invalidate(&mut self) {
        self.stale = true;
        self.index = None;
        self.saved_input.clear();
    }

    pub fn reset(&mut self) {
        self.index = None;
        self.saved_input.clear();
    }
}
