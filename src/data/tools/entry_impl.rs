use super::types::ToolEntry;

impl ToolEntry {
    pub fn repo_url(&self) -> String {
        format!("https://github.com/sw-embed/{}", self.repo)
    }

    pub fn demo_url(&self) -> Option<String> {
        if !self.has_web_ui {
            return None;
        }
        if let Some(override_url) = self.live_url_override {
            Some(override_url.to_string())
        } else {
            Some(format!("https://sw-embed.github.io/{}/", self.repo))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::data::all_groups;

    #[test]
    fn repo_urls_valid() {
        for g in all_groups() {
            for t in g.items {
                let url = t.repo_url();
                assert!(
                    url.starts_with("https://github.com/sw-embed/"),
                    "bad repo url for {}: {}",
                    t.name,
                    url
                );
            }
        }
    }

    #[test]
    fn demo_urls_only_when_has_web_ui() {
        for g in all_groups() {
            for t in g.items {
                if t.has_web_ui {
                    assert!(
                        t.demo_url().is_some(),
                        "has_web_ui but no demo_url for {}",
                        t.name
                    );
                } else {
                    assert!(
                        t.demo_url().is_none(),
                        "no web_ui but has demo_url for {}",
                        t.name
                    );
                }
            }
        }
    }
}
