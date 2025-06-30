use std::{borrow::Cow, path::Path, sync::Mutex};

use elasticlunr::{Index, IndexBuilder};
use reflexo_typst::{error::prelude::*, path::unix_slash};
use serde::Serialize;
use typst::ecow::EcoString;

use crate::meta::Search;
use crate::utils::{collapse_whitespace, write_file};

const MAX_WORD_LENGTH_TO_INDEX: usize = 80;

/// Tokenizes in the same way as elasticlunr-rs (for English), but also drops
/// long tokens.
fn tokenize(text: &str) -> Vec<String> {
    text.split(|c: char| c.is_whitespace() || c == '-')
        .filter(|s| !s.is_empty())
        .map(|s| s.trim().to_lowercase())
        .filter(|s| s.len() <= MAX_WORD_LENGTH_TO_INDEX)
        .collect()
}

pub struct SearchRenderer {
    index: Index,
    doc_urls: Vec<String>,
    pub config: Search,
}

impl SearchRenderer {
    pub fn new() -> Self {
        let index = IndexBuilder::new()
            .add_field_with_tokenizer("title", Box::new(&tokenize))
            .add_field_with_tokenizer("body", Box::new(&tokenize))
            .add_field_with_tokenizer("breadcrumbs", Box::new(&tokenize))
            .build();

        SearchRenderer {
            index,
            doc_urls: vec![],
            config: Search::default(),
        }
    }

    /// Uses the given arguments to construct a search document, then inserts it
    /// to the given index.
    fn add_doc(&mut self, anchor_base: &str, section_id: &Option<String>, items: &[&str]) {
        let url = if let Some(ref id) = *section_id {
            Cow::Owned(format!("{anchor_base}#{id}"))
        } else {
            Cow::Borrowed(anchor_base)
        };
        let doc_ref = self.doc_urls.len().to_string();
        self.doc_urls.push(url.into());

        let items = items.iter().map(|&x| collapse_whitespace(x.trim()));
        self.index.add_doc(&doc_ref, items);
    }

    pub fn render_search_index(&mut self, dest_dir: &Path) -> Result<()> {
        let index = write_to_json(&self.index, &self.config, &self.doc_urls)?;
        if index.len() > 10_000_000 {
            log::warn!("searchindex.json is very large ({} bytes)", index.len());
        }

        write_file(dest_dir.join("searchindex.json"), index.as_bytes())?;
        write_file(
            dest_dir.join("searchindex.js"),
            format!("Object.assign(window.search, {index});").as_bytes(),
        )?;

        Ok(())
    }

    pub fn build(&mut self, items: &[SearchItem]) -> Result<()> {
        for item in items {
            let title = item.title.as_str();
            let desc = item.desc.as_str();
            let dest = item.anchor_base.as_str();

            // , &breadcrumbs.join(" » ")
            // todo: currently, breadcrumbs is title it self
            self.add_doc(dest, &None, &[title, desc, title]);
        }

        Ok(())
    }
}

pub struct SearchItem {
    anchor_base: String,
    title: EcoString,
    desc: EcoString,
}

pub struct SearchCtx<'a> {
    pub config: &'a Search,
    pub items: Mutex<Vec<SearchItem>>,
}

impl SearchCtx<'_> {
    pub fn index_search(&self, dest: &Path, title: EcoString, desc: EcoString) {
        let anchor_base = unix_slash(dest);

        self.items.lock().unwrap().push(SearchItem {
            anchor_base,
            title,
            desc,
        });
    }
}

fn write_to_json(index: &Index, search_config: &Search, doc_urls: &Vec<String>) -> Result<String> {
    use elasticlunr::config::{SearchBool, SearchOptions, SearchOptionsField};
    use std::collections::BTreeMap;

    #[derive(Serialize)]
    struct ResultsOptions {
        limit_results: u32,
        teaser_word_count: u32,
    }

    #[derive(Serialize)]
    struct SearchindexJson<'a> {
        /// The options used for displaying search results
        results_options: ResultsOptions,
        /// The searchoptions for elasticlunr.js
        search_options: SearchOptions,
        /// Used to lookup a document's URL from an integer document ref.
        doc_urls: &'a Vec<String>,
        /// The index for elasticlunr.js
        index: &'a elasticlunr::Index,
    }

    let mut fields = BTreeMap::new();
    let mut opt = SearchOptionsField::default();
    let mut insert_boost = |key: &str, boost| {
        opt.boost = Some(boost);
        fields.insert(key.into(), opt);
    };
    insert_boost("title", search_config.boost_title);
    insert_boost("body", search_config.boost_paragraph);
    insert_boost("breadcrumbs", search_config.boost_hierarchy);

    let search_options = SearchOptions {
        bool: if search_config.use_boolean_and {
            SearchBool::And
        } else {
            SearchBool::Or
        },
        expand: search_config.expand,
        fields,
    };

    let results_options = ResultsOptions {
        limit_results: search_config.limit_results,
        teaser_word_count: search_config.teaser_word_count,
    };

    let json_contents = SearchindexJson {
        results_options,
        search_options,
        doc_urls,
        index,
    };

    // By converting to serde_json::Value as an intermediary, we use a
    // BTreeMap internally and can force a stable ordering of map keys.
    let json_contents =
        serde_json::to_value(&json_contents).context("Failed to serialize search index to JSON")?;
    let json_contents = serde_json::to_string(&json_contents)
        .context("Failed to serialize search index to JSON string")?;

    Ok(json_contents)
}
