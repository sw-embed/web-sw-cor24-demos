use crate::data::demos::{DemoEntry, DemoStatus};

#[derive(Clone, PartialEq)]
pub struct FilterState {
    pub search: String,
    pub category: CategoryFilter,
    pub status: StatusFilter,
}

#[derive(Clone, PartialEq, Debug)]
pub enum CategoryFilter {
    All,
    Specific(String),
}

#[derive(Clone, PartialEq, Debug)]
pub enum StatusFilter {
    All,
    Specific(DemoStatus),
}

impl Default for FilterState {
    fn default() -> Self {
        Self {
            search: String::new(),
            category: CategoryFilter::All,
            status: StatusFilter::All,
        }
    }
}

impl FilterState {
    pub fn matches(&self, entry: &DemoEntry) -> bool {
        if !self.search.is_empty() {
            let q = self.search.to_lowercase();
            let name_match = entry.name.to_lowercase().contains(&q);
            let desc_match = entry.description.to_lowercase().contains(&q);
            let tag_match = entry.tags.iter().any(|t| t.to_lowercase().contains(&q));
            if !name_match && !desc_match && !tag_match {
                return false;
            }
        }
        if let CategoryFilter::Specific(ref cat) = self.category
            && !entry.tags.iter().any(|t| tag_to_filter(t) == *cat)
        {
            return false;
        }
        if let StatusFilter::Specific(ref s) = self.status
            && &entry.status != s
        {
            return false;
        }
        true
    }
}

fn tag_to_filter(tag: &str) -> String {
    match tag {
        "IDE" | "Assembler" => "Compiler".to_string(),
        "Compiler" | "C" | "Rust" | "Pascal" | "PL/SW" | "Fortran" => "Compiler".to_string(),
        "Interpreter" | "APL" | "Forth" | "Lisp" | "BASIC" | "Scripting" => {
            "Interpreter".to_string()
        }
        "Debugger" => "Debugger".to_string(),
        "VM" | "Emulator" => "VM".to_string(),
        "System" | "Monitor" | "Docs" => "System".to_string(),
        _ => tag.to_string(),
    }
}

pub fn filter_categories() -> Vec<(&'static str, String)> {
    vec![
        ("All", "all".to_string()),
        ("Compiler", "compiler".to_string()),
        ("Interpreter", "interpreter".to_string()),
        ("Debugger", "debugger".to_string()),
        ("VM / Emulator", "vm".to_string()),
        ("System", "system".to_string()),
    ]
}

pub fn filter_statuses() -> Vec<(&'static str, StatusFilter)> {
    vec![
        ("All", StatusFilter::All),
        ("Active", StatusFilter::Specific(DemoStatus::Active)),
        ("WIP", StatusFilter::Specific(DemoStatus::Wip)),
        ("Testing", StatusFilter::Specific(DemoStatus::Testing)),
        ("Design", StatusFilter::Specific(DemoStatus::Design)),
        ("Long-term", StatusFilter::Specific(DemoStatus::LongTerm)),
    ]
}
