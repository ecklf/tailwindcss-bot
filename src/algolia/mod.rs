use std::env;

use reqwest::{self, header::HeaderMap, Error};
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AlgoliaResponse {
    pub results: Vec<AlgoliaSearchResult>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AlgoliaSearchResult {
    pub hits: Vec<Hit>,
    #[serde(rename = "nbHits")]
    pub nb_hits: i64,
    pub page: i64,
    #[serde(rename = "nbPages")]
    pub nb_pages: i64,
    #[serde(rename = "hitsPerPage")]
    pub hits_per_page: i64,
    #[serde(rename = "exhaustiveNbHits")]
    pub exhaustive_nb_hits: bool,
    pub query: String,
    pub params: String,
    pub index: String,
    #[serde(rename = "processingTimeMS")]
    pub processing_time_ms: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Hit {
    pub content: ::serde_json::Value,
    pub hierarchy: Hierarchy,
    #[serde(rename = "type")]
    pub type_field: String,
    pub url: String,
    #[serde(rename = "objectID")]
    pub object_id: String,
    #[serde(rename = "_snippetResult")]
    pub snippet_result: SnippetResult,
    #[serde(rename = "_highlightResult")]
    pub highlight_result: HighlightResult,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Hierarchy {
    pub lvl0: String,
    pub lvl1: String,
    pub lvl2: Option<String>,
    pub lvl3: ::serde_json::Value,
    pub lvl4: ::serde_json::Value,
    pub lvl5: ::serde_json::Value,
    pub lvl6: ::serde_json::Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SnippetResult {
    pub hierarchy: Hierarchy2,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Hierarchy2 {
    pub lvl1: Lvl1,
    pub lvl2: Option<Lvl2>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Lvl1 {
    pub value: String,
    #[serde(rename = "matchLevel")]
    pub match_level: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Lvl2 {
    pub value: String,
    #[serde(rename = "matchLevel")]
    pub match_level: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HighlightResult {
    pub hierarchy: Hierarchy3,
    pub hierarchy_camel: Vec<HierarchyCamel>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Hierarchy3 {
    pub lvl0: Lvl0,
    pub lvl1: Lvl12,
    pub lvl2: Option<Lvl22>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Lvl0 {
    pub value: String,
    #[serde(rename = "matchLevel")]
    pub match_level: String,
    #[serde(rename = "matchedWords")]
    pub matched_words: Vec<::serde_json::Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Lvl12 {
    pub value: String,
    #[serde(rename = "matchLevel")]
    pub match_level: String,
    #[serde(rename = "fullyHighlighted")]
    pub fully_highlighted: Option<bool>,
    #[serde(rename = "matchedWords")]
    pub matched_words: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Lvl22 {
    pub value: String,
    #[serde(rename = "matchLevel")]
    pub match_level: String,
    #[serde(rename = "fullyHighlighted")]
    pub fully_highlighted: Option<bool>,
    #[serde(rename = "matchedWords")]
    pub matched_words: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HierarchyCamel {
    pub lvl0: Lvl02,
    pub lvl1: Lvl13,
    pub lvl2: Option<Lvl23>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Lvl02 {
    pub value: String,
    #[serde(rename = "matchLevel")]
    pub match_level: String,
    #[serde(rename = "matchedWords")]
    pub matched_words: Vec<::serde_json::Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Lvl13 {
    pub value: String,
    #[serde(rename = "matchLevel")]
    pub match_level: String,
    #[serde(rename = "fullyHighlighted")]
    pub fully_highlighted: Option<bool>,
    #[serde(rename = "matchedWords")]
    pub matched_words: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Lvl23 {
    pub value: String,
    #[serde(rename = "matchLevel")]
    pub match_level: String,
    #[serde(rename = "fullyHighlighted")]
    pub fully_highlighted: Option<bool>,
    #[serde(rename = "matchedWords")]
    pub matched_words: Vec<String>,
}

#[derive(Debug)]
pub struct DocResult {
    pub label: String,
    pub description: Option<String>,
    pub url: String,
}

pub async fn search_tailwind_docs(query: &str) -> Result<Vec<DocResult>, Error> {
    let client = reqwest::Client::new();
    let mut headers = HeaderMap::new();

    const HITS_PER_PAGE: u16 = 5;
    let api_key =
        env::var("ALGOLIA_API_KEY").expect("ALGOLIA_API_KEY is undefined. Check your .env");
    let app_id = env::var("ALGOLIA_APP_ID").expect("ALGOLIA_APP_ID is undefined. Check your .env");
    let tailwind_version =
        env::var("TAILWIND_VERSION").expect("TAILWIND_VERSION is undefined. Check your .env");

    headers.insert("x-algolia-api-key", api_key.parse().unwrap());
    headers.insert("x-algolia-application-id", app_id.parse().unwrap());

    let res: Result<AlgoliaResponse, Error> = client
        .post(format!("https://{}-dsn.algolia.net/1/indexes/*/queries", app_id))
        .headers(headers)
        .body(format!("{{\"requests\":[{{\"indexName\":\"tailwindcss\",\"query\":\"{}\",\"params\":\"hitsPerPage={}&highlightPreTag=%2A%2A&highlightPostTag=%2A%2A&attributesToRetrieve=%5B%22hierarchy.lvl0%22%2C%22hierarchy.lvl1%22%2C%22hierarchy.lvl2%22%2C%22hierarchy.lvl3%22%2C%22hierarchy.lvl4%22%2C%22hierarchy.lvl5%22%2C%22hierarchy.lvl6%22%2C%22content%22%2C%22type%22%2C%22url%22%5D&attributesToSnippet=%5B%22hierarchy.lvl1%3A10%22%2C%22hierarchy.lvl2%3A10%22%2C%22hierarchy.lvl3%3A10%22%2C%22hierarchy.lvl4%3A10%22%2C%22hierarchy.lvl5%3A10%22%2C%22hierarchy.lvl6%3A10%22%2C%22content%3A10%22%5D&snippetEllipsisText=%E2%80%A6&facetFilters=version%3Av{}&distinct=1\"}}]}}", query, HITS_PER_PAGE, tailwind_version))
        .send()
        .await?.json().await;
    match res {
        Ok(res) => {
            let doc_results =
                res.results
                    .iter()
                    .fold(Vec::<Vec<DocResult>>::new(), |mut result_acc, result| {
                        let hits =
                            result
                                .hits
                                .iter()
                                .fold(Vec::<DocResult>::new(), |mut hit_acc, hit| {
                                    let h = hit.clone();
                                    hit_acc.push(DocResult {
                                        label: h.hierarchy.lvl1,
                                        description: h.hierarchy.lvl2,
                                        url: h.url.replace("#content-wrapper", ""),
                                    });
                                    hit_acc
                                });
                        result_acc.push(hits);
                        result_acc
                    });

            let flattened_doc_results = doc_results
                .into_iter()
                .flatten()
                .collect::<Vec<DocResult>>();

            Ok(flattened_doc_results)
        }
        Err(why) => Err(why),
    }
}
