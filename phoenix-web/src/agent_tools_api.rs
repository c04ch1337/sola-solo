// phoenix-web/src/agent_tools_api.rs
//
// API handlers for Sola's autonomous tools
//
// These endpoints expose the Level 5 Autonomy tools:
// - Web search (Tavily/Serper)
// - Filesystem search (grep/find)
// - System context

use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

use crate::tools::{
    web_search::{WebSearchTool, research_task},
    filesystem::{FileSystemTool, search_files, find_files, crawl_directory},
};
use crate::system_info::{get_local_info, get_system_context_prompt, SystemContext};

/// Request for web search
#[derive(Debug, Deserialize)]
pub struct WebSearchRequest {
    pub query: String,
}

/// Request for filesystem search
#[derive(Debug, Deserialize)]
pub struct FileSearchRequest {
    pub pattern: String,
    pub path: Option<String>,
}

/// Request for file find
#[derive(Debug, Deserialize)]
pub struct FileFindRequest {
    pub name: String,
    pub path: Option<String>,
}

/// Request for directory crawl
#[derive(Debug, Deserialize)]
pub struct DirectoryCrawlRequest {
    pub path: Option<String>,
    pub max_depth: Option<usize>,
}

/// Response wrapper
#[derive(Debug, Serialize)]
pub struct ToolResponse {
    pub success: bool,
    pub tool: String,
    pub result: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

/// GET /api/agent/search - Web search endpoint
pub async fn api_agent_search(query: web::Query<WebSearchRequest>) -> impl Responder {
    let tool = WebSearchTool::new();
    
    if !tool.is_available() {
        return HttpResponse::ServiceUnavailable().json(ToolResponse {
            success: false,
            tool: "web_search".to_string(),
            result: serde_json::json!(null),
            error: Some("Web search not available: No API keys configured (TAVILY_API_KEY or SERPER_API_KEY)".to_string()),
        });
    }

    match tool.search(&query.query).await {
        Ok(response) => {
            let formatted = tool.format_for_llm(&response);
            HttpResponse::Ok().json(ToolResponse {
                success: true,
                tool: "web_search".to_string(),
                result: serde_json::json!({
                    "query": response.query,
                    "source": response.source,
                    "answer": response.answer,
                    "results": response.results,
                    "formatted": formatted,
                }),
                error: None,
            })
        }
        Err(e) => {
            HttpResponse::InternalServerError().json(ToolResponse {
                success: false,
                tool: "web_search".to_string(),
                result: serde_json::json!(null),
                error: Some(e),
            })
        }
    }
}

/// POST /api/agent/search - Web search endpoint (POST version)
pub async fn api_agent_search_post(body: web::Json<WebSearchRequest>) -> impl Responder {
    let tool = WebSearchTool::new();
    
    if !tool.is_available() {
        return HttpResponse::ServiceUnavailable().json(ToolResponse {
            success: false,
            tool: "web_search".to_string(),
            result: serde_json::json!(null),
            error: Some("Web search not available: No API keys configured".to_string()),
        });
    }

    match tool.search(&body.query).await {
        Ok(response) => {
            let formatted = tool.format_for_llm(&response);
            HttpResponse::Ok().json(ToolResponse {
                success: true,
                tool: "web_search".to_string(),
                result: serde_json::json!({
                    "query": response.query,
                    "source": response.source,
                    "answer": response.answer,
                    "results": response.results,
                    "formatted": formatted,
                }),
                error: None,
            })
        }
        Err(e) => {
            HttpResponse::InternalServerError().json(ToolResponse {
                success: false,
                tool: "web_search".to_string(),
                result: serde_json::json!(null),
                error: Some(e),
            })
        }
    }
}

/// GET /api/agent/filesystem/search - Filesystem content search
pub async fn api_agent_filesystem_search(query: web::Query<FileSearchRequest>) -> impl Responder {
    let tool = FileSystemTool::new();
    let response = tool.search_content(&query.pattern, query.path.as_deref());
    let formatted = tool.format_search_for_llm(&response);

    HttpResponse::Ok().json(ToolResponse {
        success: true,
        tool: "filesystem_search".to_string(),
        result: serde_json::json!({
            "query": response.query,
            "search_path": response.search_path,
            "total_matches": response.total_matches,
            "results": response.results,
            "formatted": formatted,
        }),
        error: None,
    })
}

/// POST /api/agent/filesystem/search - Filesystem content search (POST version)
pub async fn api_agent_filesystem_search_post(body: web::Json<FileSearchRequest>) -> impl Responder {
    let tool = FileSystemTool::new();
    let response = tool.search_content(&body.pattern, body.path.as_deref());
    let formatted = tool.format_search_for_llm(&response);

    HttpResponse::Ok().json(ToolResponse {
        success: true,
        tool: "filesystem_search".to_string(),
        result: serde_json::json!({
            "query": response.query,
            "search_path": response.search_path,
            "total_matches": response.total_matches,
            "results": response.results,
            "formatted": formatted,
        }),
        error: None,
    })
}

/// GET /api/agent/filesystem/find - Find files by name
pub async fn api_agent_filesystem_find(query: web::Query<FileFindRequest>) -> impl Responder {
    let tool = FileSystemTool::new();
    let results = tool.find_files(&query.name, query.path.as_deref());
    
    let response = crate::tools::filesystem::FileSystemSearchResponse {
        query: query.name.clone(),
        search_path: query.path.clone().unwrap_or_else(|| ".".to_string()),
        total_matches: results.len(),
        results: results.clone(),
    };
    let formatted = tool.format_search_for_llm(&response);

    HttpResponse::Ok().json(ToolResponse {
        success: true,
        tool: "filesystem_find".to_string(),
        result: serde_json::json!({
            "name_pattern": query.name,
            "search_path": query.path,
            "total_matches": results.len(),
            "results": results,
            "formatted": formatted,
        }),
        error: None,
    })
}

/// GET /api/agent/filesystem/crawl - Crawl directory structure
pub async fn api_agent_filesystem_crawl(query: web::Query<DirectoryCrawlRequest>) -> impl Responder {
    let tool = FileSystemTool::new();
    let entries = tool.crawl_directory(query.path.as_deref(), query.max_depth);
    let formatted = tool.format_crawl_for_llm(&entries);

    HttpResponse::Ok().json(ToolResponse {
        success: true,
        tool: "filesystem_crawl".to_string(),
        result: serde_json::json!({
            "path": query.path,
            "max_depth": query.max_depth,
            "total_entries": entries.len(),
            "entries": entries,
            "formatted": formatted,
        }),
        error: None,
    })
}

/// GET /api/agent/context - Get system context
pub async fn api_agent_context() -> impl Responder {
    let context = SystemContext::now();
    
    HttpResponse::Ok().json(ToolResponse {
        success: true,
        tool: "system_context".to_string(),
        result: serde_json::json!({
            "local_time": context.local_time,
            "utc_time": context.utc_time,
            "timezone_iana": context.timezone_iana,
            "utc_offset": context.utc_offset,
            "os": context.os,
            "arch": context.arch,
            "prompt_block": context.to_prompt_block(),
        }),
        error: None,
    })
}

/// GET /api/agent/tools - List available tools
pub async fn api_agent_tools_list() -> impl Responder {
    let web_search_tool = WebSearchTool::new();
    
    HttpResponse::Ok().json(serde_json::json!({
        "tools": [
            {
                "name": "web_search",
                "description": "Search the web for real-time information using Tavily or Serper API",
                "available": web_search_tool.is_available(),
                "endpoint": "/api/agent/search",
                "methods": ["GET", "POST"],
                "parameters": {
                    "query": "Search query string"
                }
            },
            {
                "name": "filesystem_search",
                "description": "Search for content within files (grep-like)",
                "available": true,
                "endpoint": "/api/agent/filesystem/search",
                "methods": ["GET", "POST"],
                "parameters": {
                    "pattern": "Search pattern",
                    "path": "Optional: Directory to search in"
                }
            },
            {
                "name": "filesystem_find",
                "description": "Find files by name pattern",
                "available": true,
                "endpoint": "/api/agent/filesystem/find",
                "methods": ["GET"],
                "parameters": {
                    "name": "File name pattern (supports * wildcards)",
                    "path": "Optional: Directory to search in"
                }
            },
            {
                "name": "filesystem_crawl",
                "description": "List directory structure recursively",
                "available": true,
                "endpoint": "/api/agent/filesystem/crawl",
                "methods": ["GET"],
                "parameters": {
                    "path": "Optional: Directory to crawl",
                    "max_depth": "Optional: Maximum depth (default: 3)"
                }
            },
            {
                "name": "system_context",
                "description": "Get current system context (time, timezone, OS)",
                "available": true,
                "endpoint": "/api/agent/context",
                "methods": ["GET"],
                "parameters": {}
            }
        ],
        "autonomy_level": 5,
        "description": "Level 5 Autonomy tools for persistent, self-evolving intelligence"
    }))
}

/// Configure routes for agent tools
pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/agent")
            // Existing evolution routes are in code_evolution module
            // These are the new autonomy tools
            .service(
                web::resource("/search")
                    .route(web::get().to(api_agent_search))
                    .route(web::post().to(api_agent_search_post)),
            )
            .service(
                web::resource("/context")
                    .route(web::get().to(api_agent_context)),
            )
            .service(
                web::resource("/tools")
                    .route(web::get().to(api_agent_tools_list)),
            )
            .service(
                web::scope("/filesystem")
                    .service(
                        web::resource("/search")
                            .route(web::get().to(api_agent_filesystem_search))
                            .route(web::post().to(api_agent_filesystem_search_post)),
                    )
                    .service(
                        web::resource("/find")
                            .route(web::get().to(api_agent_filesystem_find)),
                    )
                    .service(
                        web::resource("/crawl")
                            .route(web::get().to(api_agent_filesystem_crawl)),
                    ),
            ),
    );
}
