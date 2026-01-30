// phoenix-web/src/tools/web_search.rs
//
// Web Search Tool for Sola's Autonomous Capabilities
//
// This module provides real-time web search functionality using
// the Tavily API (or Serper as fallback). It enables Sola to
// research topics when local knowledge is insufficient.
//
// Part of Level 5 Autonomy: "Unlimited Discovery"

use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::env;
use tracing::{info, warn, error};

/// Search result from web search
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    pub title: String,
    pub url: String,
    pub content: String,
    pub score: f64,
}

/// Web search response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebSearchResponse {
    pub query: String,
    pub results: Vec<SearchResult>,
    pub answer: Option<String>,
    pub source: String,
}

/// Tavily API response structure
#[derive(Debug, Deserialize)]
struct TavilyResponse {
    query: String,
    #[serde(default)]
    results: Vec<TavilyResult>,
    #[serde(default)]
    answer: Option<String>,
}

#[derive(Debug, Deserialize)]
struct TavilyResult {
    title: String,
    url: String,
    content: String,
    #[serde(default)]
    score: f64,
}

/// Serper API response structure (fallback)
#[derive(Debug, Deserialize)]
struct SerperResponse {
    #[serde(default)]
    organic: Vec<SerperResult>,
    #[serde(default)]
    answer_box: Option<SerperAnswerBox>,
}

#[derive(Debug, Deserialize)]
struct SerperResult {
    title: String,
    link: String,
    snippet: String,
}

#[derive(Debug, Deserialize)]
struct SerperAnswerBox {
    #[serde(default)]
    answer: Option<String>,
    #[serde(default)]
    snippet: Option<String>,
}

/// Web Search Tool for autonomous research
pub struct WebSearchTool {
    client: Client,
    tavily_api_key: Option<String>,
    serper_api_key: Option<String>,
}

impl WebSearchTool {
    /// Create a new WebSearchTool
    pub fn new() -> Self {
        Self {
            client: Client::new(),
            tavily_api_key: env::var("TAVILY_API_KEY").ok(),
            serper_api_key: env::var("SERPER_API_KEY").ok(),
        }
    }

    /// Check if web search is available
    pub fn is_available(&self) -> bool {
        self.tavily_api_key.is_some() || self.serper_api_key.is_some()
    }

    /// Perform a web search
    pub async fn search(&self, query: &str) -> Result<WebSearchResponse, String> {
        // Try Tavily first (preferred)
        if let Some(api_key) = &self.tavily_api_key {
            match self.search_tavily(query, api_key).await {
                Ok(response) => return Ok(response),
                Err(e) => {
                    warn!("Tavily search failed, trying fallback: {}", e);
                }
            }
        }

        // Fallback to Serper
        if let Some(api_key) = &self.serper_api_key {
            match self.search_serper(query, api_key).await {
                Ok(response) => return Ok(response),
                Err(e) => {
                    error!("Serper search also failed: {}", e);
                    return Err(format!("All search providers failed: {}", e));
                }
            }
        }

        Err("No search API keys configured. Set TAVILY_API_KEY or SERPER_API_KEY in .env".to_string())
    }

    /// Search using Tavily API
    async fn search_tavily(&self, query: &str, api_key: &str) -> Result<WebSearchResponse, String> {
        let url = "https://api.tavily.com/search";
        
        let body = serde_json::json!({
            "api_key": api_key,
            "query": query,
            "search_depth": "advanced",
            "include_answer": true,
            "max_results": 5
        });

        let response = self.client
            .post(url)
            .json(&body)
            .send()
            .await
            .map_err(|e| format!("Tavily request failed: {}", e))?;

        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await.unwrap_or_default();
            return Err(format!("Tavily API error {}: {}", status, text));
        }

        let tavily_response: TavilyResponse = response
            .json()
            .await
            .map_err(|e| format!("Failed to parse Tavily response: {}", e))?;

        info!("Tavily search completed: {} results for '{}'", 
              tavily_response.results.len(), query);

        Ok(WebSearchResponse {
            query: tavily_response.query,
            results: tavily_response.results.into_iter().map(|r| SearchResult {
                title: r.title,
                url: r.url,
                content: r.content,
                score: r.score,
            }).collect(),
            answer: tavily_response.answer,
            source: "tavily".to_string(),
        })
    }

    /// Search using Serper API (fallback)
    async fn search_serper(&self, query: &str, api_key: &str) -> Result<WebSearchResponse, String> {
        let url = "https://google.serper.dev/search";
        
        let body = serde_json::json!({
            "q": query,
            "num": 5
        });

        let response = self.client
            .post(url)
            .header("X-API-KEY", api_key)
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await
            .map_err(|e| format!("Serper request failed: {}", e))?;

        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await.unwrap_or_default();
            return Err(format!("Serper API error {}: {}", status, text));
        }

        let serper_response: SerperResponse = response
            .json()
            .await
            .map_err(|e| format!("Failed to parse Serper response: {}", e))?;

        info!("Serper search completed: {} results for '{}'", 
              serper_response.organic.len(), query);

        // Extract answer from answer_box if available
        let answer = serper_response.answer_box.and_then(|ab| {
            ab.answer.or(ab.snippet)
        });

        Ok(WebSearchResponse {
            query: query.to_string(),
            results: serper_response.organic.into_iter().enumerate().map(|(i, r)| SearchResult {
                title: r.title,
                url: r.link,
                content: r.snippet,
                score: 1.0 - (i as f64 * 0.1), // Approximate score based on position
            }).collect(),
            answer,
            source: "serper".to_string(),
        })
    }

    /// Format search results for LLM consumption
    pub fn format_for_llm(&self, response: &WebSearchResponse) -> String {
        let mut output = format!("## Web Search Results for: \"{}\"\n\n", response.query);

        // Include direct answer if available
        if let Some(answer) = &response.answer {
            output.push_str(&format!("**Direct Answer:** {}\n\n", answer));
        }

        // Include top results
        output.push_str("**Sources:**\n\n");
        for (i, result) in response.results.iter().take(5).enumerate() {
            output.push_str(&format!(
                "{}. **{}**\n   URL: {}\n   {}\n\n",
                i + 1,
                result.title,
                result.url,
                result.content.chars().take(300).collect::<String>()
            ));
        }

        output
    }
}

impl Default for WebSearchTool {
    fn default() -> Self {
        Self::new()
    }
}

/// Convenience function for quick research tasks
/// 
/// This is the primary entry point for autonomous research.
/// It performs a web search and returns formatted findings.
pub async fn research_task(query: &str) -> Result<String, String> {
    let tool = WebSearchTool::new();
    
    if !tool.is_available() {
        return Err("Web search not available: No API keys configured".to_string());
    }

    let response = tool.search(query).await?;
    Ok(tool.format_for_llm(&response))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_web_search_tool_creation() {
        let tool = WebSearchTool::new();
        // Tool should be created even without API keys
        assert!(true);
    }

    #[test]
    fn test_format_for_llm() {
        let tool = WebSearchTool::new();
        let response = WebSearchResponse {
            query: "test query".to_string(),
            results: vec![
                SearchResult {
                    title: "Test Result".to_string(),
                    url: "https://example.com".to_string(),
                    content: "This is test content".to_string(),
                    score: 0.9,
                }
            ],
            answer: Some("Direct answer here".to_string()),
            source: "test".to_string(),
        };

        let formatted = tool.format_for_llm(&response);
        assert!(formatted.contains("test query"));
        assert!(formatted.contains("Direct answer here"));
        assert!(formatted.contains("Test Result"));
    }
}
