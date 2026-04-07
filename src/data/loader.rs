use anyhow::{bail, Result};
use std::io::Read;
use std::path::Path;

#[derive(Debug, Clone, PartialEq)]
pub enum FileFormat {
    Csv,
    Tsv,
    Json,
    JsonLines,
    Parquet,
    Excel,
}

impl FileFormat {
    pub fn as_str(&self) -> &str {
        match self {
            FileFormat::Csv => "csv",
            FileFormat::Tsv => "tsv",
            FileFormat::Json => "json",
            FileFormat::JsonLines => "jsonl",
            FileFormat::Parquet => "parquet",
            FileFormat::Excel => "xlsx",
        }
    }

    pub fn icon(&self) -> &str {
        match self {
            FileFormat::Csv | FileFormat::Tsv => "CSV",
            FileFormat::Json | FileFormat::JsonLines => "JSON",
            FileFormat::Parquet => "PRQ",
            FileFormat::Excel => "XLS",
        }
    }
}

pub fn detect_format(path: &str) -> Result<FileFormat> {
    let ext = Path::new(path)
        .extension()
        .and_then(|e| e.to_str())
        .map(|e| e.to_lowercase());

    if let Some(ext) = &ext {
        match ext.as_str() {
            "csv" => return Ok(FileFormat::Csv),
            "tsv" | "tab" => return Ok(FileFormat::Tsv),
            "json" => return Ok(FileFormat::Json),
            "jsonl" | "ndjson" => return Ok(FileFormat::JsonLines),
            "parquet" | "pq" => return Ok(FileFormat::Parquet),
            "xlsx" | "xls" => return Ok(FileFormat::Excel),
            _ => {}
        }
    }

    // Try magic bytes
    let mut file = std::fs::File::open(path)?;
    let mut buf = [0u8; 8];
    let n = file.read(&mut buf)?;

    if n >= 4 && &buf[0..4] == b"PAR1" {
        return Ok(FileFormat::Parquet);
    }
    if n >= 4 && &buf[0..4] == [0x50, 0x4B, 0x03, 0x04] {
        return Ok(FileFormat::Excel);
    }

    // Check if it looks like JSON
    let mut content = String::new();
    file = std::fs::File::open(path)?;
    file.read_to_string(&mut content)?;
    let trimmed = content.trim_start();
    if trimmed.starts_with('[') || trimmed.starts_with('{') {
        if trimmed.contains('\n') && !trimmed.starts_with('[') {
            return Ok(FileFormat::JsonLines);
        }
        return Ok(FileFormat::Json);
    }

    // Check for TSV vs CSV
    let first_line = content.lines().next().unwrap_or("");
    if first_line.contains('\t') && !first_line.contains(',') {
        return Ok(FileFormat::Tsv);
    }

    // Default to CSV
    Ok(FileFormat::Csv)
}

/// List supported data files in the current working directory (non-recursive),
/// sorted by most-recently-modified first. Hidden files (dotfiles) are skipped.
pub fn list_data_files_in_cwd() -> Result<Vec<std::path::PathBuf>> {
    let cwd = std::env::current_dir()?;
    let mut entries: Vec<(std::path::PathBuf, std::time::SystemTime)> = Vec::new();

    for entry in std::fs::read_dir(&cwd)?.flatten() {
        let path = entry.path();
        if !path.is_file() {
            continue;
        }
        if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
            if name.starts_with('.') {
                continue;
            }
        }
        let ext = path
            .extension()
            .and_then(|e| e.to_str())
            .map(|e| e.to_lowercase());
        let is_supported = matches!(
            ext.as_deref(),
            Some("csv") | Some("tsv") | Some("tab") | Some("json") | Some("jsonl")
            | Some("ndjson") | Some("parquet") | Some("pq") | Some("xlsx") | Some("xls")
        );
        if !is_supported {
            continue;
        }
        let mtime = entry
            .metadata()
            .and_then(|m| m.modified())
            .unwrap_or(std::time::SystemTime::UNIX_EPOCH);
        entries.push((path, mtime));
    }

    entries.sort_by(|a, b| b.1.cmp(&a.1));
    Ok(entries.into_iter().map(|(p, _)| p).collect())
}

pub fn load_from_stdin() -> Result<String> {
    use std::io::{self, IsTerminal};

    if io::stdin().is_terminal() {
        bail!("No file specified and stdin is a terminal. Usage: xeli <file>");
    }

    let mut content = String::new();
    io::stdin().read_to_string(&mut content)?;

    // Write to temp file
    let tmp = std::env::temp_dir().join("xeli_stdin.csv");
    std::fs::write(&tmp, &content)?;

    Ok(tmp.to_string_lossy().to_string())
}
