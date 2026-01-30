// phoenix-web/src/tools/mod.rs
//
// Tool modules for Sola's autonomous capabilities
//
// These tools enable Level 5 Autonomy by providing:
// - Web search for real-time information
// - Filesystem search for local discovery
// - System context for environmental awareness

pub mod web_search;
pub mod filesystem;

pub use web_search::{WebSearchTool, SearchResult, research_task};
pub use filesystem::{FileSystemTool, FileSearchResult, search_files, find_files, crawl_directory};
