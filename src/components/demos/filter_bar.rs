use crate::data::demos::{DemoEntry, DemoStatus};

#[derive(Clone, PartialEq)]
pub struct FilterState {
    pub search: String,
    pub language: LanguageFilter,
    pub status: StatusFilter,
}

#[derive(Clone, PartialEq, Debug)]
pub enum LanguageFilter {
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
            language: LanguageFilter::All,
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
        if let LanguageFilter::Specific(ref lang) = self.language
            && entry.group_id != lang.as_str()
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

pub fn filter_languages() -> Vec<(&'static str, String)> {
    crate::data::demos::filter_languages()
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
