// phoenix-web/src/tools/filesystem.rs
//
// Filesystem Search Tool for Sola's Autonomous Capabilities
//
// This module provides filesystem search functionality including:
// - grep-like content search
// - find-like file discovery
// - directory crawling
//
// Part of Level 5 Autonomy: "Unlimited Discovery"

use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::process::Command;
use tracing::{info, warn};

/// File search result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileSearchResult {
    pub path: String,
    pub line_number: Option<usize>,
    pub content: Option<String>,
    pub file_type: String,
}

/// Directory listing result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DirectoryEntry {
    pub path: String,
    pub name: String,
    pub is_dir: bool,
    pub size: Option<u64>,
}

/// Filesystem search response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileSystemSearchResponse {
    pub query: String,
    pub search_path: String,
    pub results: Vec<FileSearchResult>,
    pub total_matches: usize,
}

/// Filesystem Tool for autonomous file discovery
pub struct FileSystemTool {
    workspace_root: PathBuf,
    max_results: usize,
    max_file_size: u64,
}

impl FileSystemTool {
    /// Create a new FileSystemTool
    pub fn new() -> Self {
        Self {
            workspace_root: std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")),
            max_results: 100,
            max_file_size: 10 * 1024 * 1024, // 10MB max file size to search
        }
    }

    /// Create with custom workspace root
    pub fn with_root(root: PathBuf) -> Self {
        Self {
            workspace_root: root,
            max_results: 100,
            max_file_size: 10 * 1024 * 1024,
        }
    }

    /// Search for content within files (grep-like)
    pub fn search_content(&self, pattern: &str, search_path: Option<&str>) -> FileSystemSearchResponse {
        let base_path = search_path
            .map(|p| self.workspace_root.join(p))
            .unwrap_or_else(|| self.workspace_root.clone());

        let mut results = Vec::new();
        self.search_recursive(&base_path, pattern, &mut results);

        info!("Filesystem search for '{}' found {} matches", pattern, results.len());

        FileSystemSearchResponse {
            query: pattern.to_string(),
            search_path: base_path.to_string_lossy().to_string(),
            total_matches: results.len(),
            results,
        }
    }

    /// Recursive search implementation
    fn search_recursive(&self, path: &Path, pattern: &str, results: &mut Vec<FileSearchResult>) {
        if results.len() >= self.max_results {
            return;
        }

        if path.is_file() {
            self.search_file(path, pattern, results);
        } else if path.is_dir() {
            // Skip common non-searchable directories
            let dir_name = path.file_name().map(|n| n.to_string_lossy().to_string());
            if let Some(name) = &dir_name {
                if self.should_skip_directory(name) {
                    return;
                }
            }

            if let Ok(entries) = fs::read_dir(path) {
                for entry in entries.flatten() {
                    self.search_recursive(&entry.path(), pattern, results);
                }
            }
        }
    }

    /// Search within a single file
    fn search_file(&self, path: &Path, pattern: &str, results: &mut Vec<FileSearchResult>) {
        // Skip binary and large files
        if let Ok(metadata) = fs::metadata(path) {
            if metadata.len() > self.max_file_size {
                return;
            }
        }

        // Skip non-text files
        let extension = path.extension().map(|e| e.to_string_lossy().to_string());
        if !self.is_searchable_extension(&extension) {
            return;
        }

        let file = match fs::File::open(path) {
            Ok(f) => f,
            Err(_) => return,
        };

        let reader = BufReader::new(file);
        let pattern_lower = pattern.to_lowercase();

        for (line_num, line) in reader.lines().enumerate() {
            if results.len() >= self.max_results {
                break;
            }

            if let Ok(line_content) = line {
                if line_content.to_lowercase().contains(&pattern_lower) {
                    results.push(FileSearchResult {
                        path: path.to_string_lossy().to_string(),
                        line_number: Some(line_num + 1),
                        content: Some(line_content.chars().take(200).collect()),
                        file_type: extension.clone().unwrap_or_else(|| "unknown".to_string()),
                    });
                }
            }
        }
    }

    /// Find files by name pattern
    pub fn find_files(&self, name_pattern: &str, search_path: Option<&str>) -> Vec<FileSearchResult> {
        let base_path = search_path
            .map(|p| self.workspace_root.join(p))
            .unwrap_or_else(|| self.workspace_root.clone());

        let mut results = Vec::new();
        self.find_recursive(&base_path, name_pattern, &mut results);

        info!("Find files for '{}' found {} matches", name_pattern, results.len());
        results
    }

    /// Recursive find implementation
    fn find_recursive(&self, path: &Path, pattern: &str, results: &mut Vec<FileSearchResult>) {
        if results.len() >= self.max_results {
            return;
        }

        if path.is_dir() {
            let dir_name = path.file_name().map(|n| n.to_string_lossy().to_string());
            if let Some(name) = &dir_name {
                if self.should_skip_directory(name) {
                    return;
                }
            }

            if let Ok(entries) = fs::read_dir(path) {
                for entry in entries.flatten() {
                    let entry_path = entry.path();
                    let file_name = entry_path.file_name()
                        .map(|n| n.to_string_lossy().to_string())
                        .unwrap_or_default();

                    // Check if name matches pattern
                    if self.matches_pattern(&file_name, pattern) {
                        let extension = entry_path.extension()
                            .map(|e| e.to_string_lossy().to_string())
                            .unwrap_or_else(|| "dir".to_string());

                        results.push(FileSearchResult {
                            path: entry_path.to_string_lossy().to_string(),
                            line_number: None,
                            content: None,
                            file_type: if entry_path.is_dir() { "directory".to_string() } else { extension },
                        });
                    }

                    // Recurse into directories
                    if entry_path.is_dir() {
                        self.find_recursive(&entry_path, pattern, results);
                    }
                }
            }
        }
    }

    /// Crawl directory and list all files
    pub fn crawl_directory(&self, path: Option<&str>, max_depth: Option<usize>) -> Vec<DirectoryEntry> {
        let base_path = path
            .map(|p| self.workspace_root.join(p))
            .unwrap_or_else(|| self.workspace_root.clone());

        let mut entries = Vec::new();
        self.crawl_recursive(&base_path, 0, max_depth.unwrap_or(5), &mut entries);

        info!("Directory crawl found {} entries", entries.len());
        entries
    }

    /// Recursive crawl implementation
    fn crawl_recursive(&self, path: &Path, depth: usize, max_depth: usize, entries: &mut Vec<DirectoryEntry>) {
        if depth > max_depth || entries.len() >= self.max_results {
            return;
        }

        if let Ok(dir_entries) = fs::read_dir(path) {
            for entry in dir_entries.flatten() {
                let entry_path = entry.path();
                let file_name = entry_path.file_name()
                    .map(|n| n.to_string_lossy().to_string())
                    .unwrap_or_default();

                // Skip hidden and common non-useful directories
                if file_name.starts_with('.') || self.should_skip_directory(&file_name) {
                    continue;
                }

                let is_dir = entry_path.is_dir();
                let size = if is_dir {
                    None
                } else {
                    fs::metadata(&entry_path).ok().map(|m| m.len())
                };

                entries.push(DirectoryEntry {
                    path: entry_path.to_string_lossy().to_string(),
                    name: file_name.clone(),
                    is_dir,
                    size,
                });

                if is_dir {
                    self.crawl_recursive(&entry_path, depth + 1, max_depth, entries);
                }
            }
        }
    }

    /// Check if directory should be skipped
    fn should_skip_directory(&self, name: &str) -> bool {
        let skip_dirs = [
            "node_modules", "target", ".git", ".svn", ".hg",
            "__pycache__", ".pytest_cache", ".mypy_cache",
            "dist", "build", ".next", ".nuxt", "coverage",
            "vendor", ".cargo", ".rustup"
        ];
        skip_dirs.contains(&name)
    }

    /// Check if file extension is searchable
    fn is_searchable_extension(&self, ext: &Option<String>) -> bool {
        let searchable = [
            "rs", "ts", "tsx", "js", "jsx", "py", "go", "java", "c", "cpp", "h",
            "md", "txt", "json", "yaml", "yml", "toml", "xml", "html", "css",
            "sql", "sh", "bash", "zsh", "ps1", "bat", "cmd",
            "env", "gitignore", "dockerignore", "editorconfig"
        ];

        match ext {
            Some(e) => searchable.contains(&e.as_str()),
            None => true, // Search files without extension (like Makefile)
        }
    }

    /// Simple glob pattern matching
    fn matches_pattern(&self, name: &str, pattern: &str) -> bool {
        let name_lower = name.to_lowercase();
        let pattern_lower = pattern.to_lowercase();

        if pattern_lower.starts_with('*') && pattern_lower.ends_with('*') {
            // *pattern* - contains
            let inner = &pattern_lower[1..pattern_lower.len()-1];
            name_lower.contains(inner)
        } else if pattern_lower.starts_with('*') {
            // *.ext - ends with
            let suffix = &pattern_lower[1..];
            name_lower.ends_with(suffix)
        } else if pattern_lower.ends_with('*') {
            // prefix* - starts with
            let prefix = &pattern_lower[..pattern_lower.len()-1];
            name_lower.starts_with(prefix)
        } else {
            // Exact match or contains
            name_lower.contains(&pattern_lower)
        }
    }

    /// Format results for LLM consumption
    pub fn format_search_for_llm(&self, response: &FileSystemSearchResponse) -> String {
        let mut output = format!(
            "## Filesystem Search Results\n\n**Query:** \"{}\"\n**Path:** {}\n**Matches:** {}\n\n",
            response.query, response.search_path, response.total_matches
        );

        for result in response.results.iter().take(20) {
            if let Some(line_num) = result.line_number {
                output.push_str(&format!(
                    "- `{}` (line {}): {}\n",
                    result.path,
                    line_num,
                    result.content.as_deref().unwrap_or("")
                ));
            } else {
                output.push_str(&format!("- `{}` [{}]\n", result.path, result.file_type));
            }
        }

        if response.total_matches > 20 {
            output.push_str(&format!("\n... and {} more matches\n", response.total_matches - 20));
        }

        output
    }

    /// Format directory listing for LLM consumption
    pub fn format_crawl_for_llm(&self, entries: &[DirectoryEntry]) -> String {
        let mut output = format!("## Directory Listing ({} entries)\n\n", entries.len());

        for entry in entries.iter().take(50) {
            let icon = if entry.is_dir { "📁" } else { "📄" };
            let size_str = entry.size
                .map(|s| format!(" ({} bytes)", s))
                .unwrap_or_default();
            output.push_str(&format!("{} `{}`{}\n", icon, entry.path, size_str));
        }

        if entries.len() > 50 {
            output.push_str(&format!("\n... and {} more entries\n", entries.len() - 50));
        }

        output
    }
}

impl Default for FileSystemTool {
    fn default() -> Self {
        Self::new()
    }
}

/// Convenience function: Search for content in files
pub fn search_files(pattern: &str, path: Option<&str>) -> String {
    let tool = FileSystemTool::new();
    let response = tool.search_content(pattern, path);
    tool.format_search_for_llm(&response)
}

/// Convenience function: Find files by name
pub fn find_files(name_pattern: &str, path: Option<&str>) -> String {
    let tool = FileSystemTool::new();
    let results = tool.find_files(name_pattern, path);
    
    let response = FileSystemSearchResponse {
        query: name_pattern.to_string(),
        search_path: path.unwrap_or(".").to_string(),
        total_matches: results.len(),
        results,
    };
    
    tool.format_search_for_llm(&response)
}

/// Convenience function: Crawl directory
pub fn crawl_directory(path: Option<&str>) -> String {
    let tool = FileSystemTool::new();
    let entries = tool.crawl_directory(path, Some(3));
    tool.format_crawl_for_llm(&entries)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_filesystem_tool_creation() {
        let tool = FileSystemTool::new();
        assert!(tool.max_results > 0);
    }

    #[test]
    fn test_pattern_matching() {
        let tool = FileSystemTool::new();
        
        assert!(tool.matches_pattern("test.rs", "*.rs"));
        assert!(tool.matches_pattern("test.rs", "test*"));
        assert!(tool.matches_pattern("test.rs", "*est*"));
        assert!(!tool.matches_pattern("test.rs", "*.py"));
    }

    #[test]
    fn test_skip_directories() {
        let tool = FileSystemTool::new();
        
        assert!(tool.should_skip_directory("node_modules"));
        assert!(tool.should_skip_directory("target"));
        assert!(!tool.should_skip_directory("src"));
    }
}
