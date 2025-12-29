//! Crash Logging Module
//!
//! Logs crash details to JSON files for post-mortem analysis.
//! Part of RES-3: Panic Hook + Crash Logging.

use std::fs::{self, File};
use std::io::Write;
use std::panic::PanicHookInfo;
use std::path::PathBuf;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Directory for crash logs
const CRASH_LOG_DIR: &str = "logs";

/// Crash report with all relevant diagnostic information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrashReport {
    /// Timestamp of the crash
    pub timestamp: DateTime<Utc>,

    /// Panic message
    pub message: String,

    /// Location where panic occurred (file:line:column)
    pub location: Option<String>,

    /// Backtrace (if available)
    pub backtrace: Option<String>,

    /// Cognitive state at time of crash (optional)
    pub cognitive_state: Option<CognitiveStateSnapshot>,

    /// DANEEL version
    pub version: String,
}

/// Snapshot of cognitive state at crash time
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CognitiveStateSnapshot {
    /// Number of cognitive cycles completed
    pub cycle_count: u64,

    /// Current salience weights (if available)
    pub salience_weights: Option<Vec<f32>>,

    /// Number of active memory windows
    pub active_windows: Option<usize>,

    /// Connection drive value
    pub connection_drive: Option<f32>,

    /// Current thought in progress
    pub current_thought: Option<String>,
}

impl CrashReport {
    /// Create a new crash report from panic info
    #[cfg_attr(coverage_nightly, coverage(off))]
    pub fn from_panic_info(panic_info: &PanicHookInfo<'_>) -> Self {
        let message = if let Some(s) = panic_info.payload().downcast_ref::<&str>() {
            (*s).to_string()
        } else if let Some(s) = panic_info.payload().downcast_ref::<String>() {
            s.clone()
        } else {
            "Unknown panic payload".to_string()
        };

        let location = panic_info
            .location()
            .map(|loc| format!("{}:{}:{}", loc.file(), loc.line(), loc.column()));

        // Capture backtrace
        let backtrace = std::backtrace::Backtrace::capture();
        let backtrace_str = match backtrace.status() {
            std::backtrace::BacktraceStatus::Captured => Some(backtrace.to_string()),
            _ => None,
        };

        Self {
            timestamp: Utc::now(),
            message,
            location,
            backtrace: backtrace_str,
            cognitive_state: None, // Will be filled by caller if available
            version: env!("CARGO_PKG_VERSION").to_string(),
        }
    }

    /// Add cognitive state snapshot to the report
    pub fn with_cognitive_state(mut self, state: CognitiveStateSnapshot) -> Self {
        self.cognitive_state = Some(state);
        self
    }

    /// Get the filename for this crash report
    pub fn filename(&self) -> String {
        format!("panic_{}.json", self.timestamp.format("%Y%m%d_%H%M%S"))
    }

    /// Save crash report to file
    #[cfg_attr(coverage_nightly, coverage(off))]
    pub fn save(&self) -> std::io::Result<PathBuf> {
        // Ensure logs directory exists
        fs::create_dir_all(CRASH_LOG_DIR)?;

        let path = PathBuf::from(CRASH_LOG_DIR).join(self.filename());
        let mut file = File::create(&path)?;

        let json = serde_json::to_string_pretty(self)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

        file.write_all(json.as_bytes())?;

        Ok(path)
    }
}

/// Log a panic to a crash file.
///
/// Called from the panic hook to record crash details.
#[cfg_attr(coverage_nightly, coverage(off))]
pub fn log_panic(panic_info: &PanicHookInfo<'_>) -> std::io::Result<PathBuf> {
    let report = CrashReport::from_panic_info(panic_info);
    report.save()
}

/// Detect if there was a previous crash.
///
/// Returns the most recent crash report if one exists.
#[cfg_attr(coverage_nightly, coverage(off))]
pub fn detect_previous_crash() -> Option<CrashReport> {
    let log_dir = PathBuf::from(CRASH_LOG_DIR);

    if !log_dir.exists() {
        return None;
    }

    // Find most recent panic log
    let mut crash_files: Vec<_> = fs::read_dir(&log_dir)
        .ok()?
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.file_name().to_string_lossy().starts_with("panic_"))
        .collect();

    // Sort by name (which includes timestamp) descending
    crash_files.sort_by(|a, b| b.file_name().cmp(&a.file_name()));

    // Read most recent
    let most_recent = crash_files.first()?;
    let contents = fs::read_to_string(most_recent.path()).ok()?;
    serde_json::from_str(&contents).ok()
}

/// Get all crash reports.
#[cfg_attr(coverage_nightly, coverage(off))]
pub fn get_all_crash_reports() -> Vec<CrashReport> {
    let log_dir = PathBuf::from(CRASH_LOG_DIR);

    if !log_dir.exists() {
        return Vec::new();
    }

    fs::read_dir(&log_dir)
        .into_iter()
        .flatten()
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.file_name().to_string_lossy().starts_with("panic_"))
        .filter_map(|entry| {
            let contents = fs::read_to_string(entry.path()).ok()?;
            serde_json::from_str(&contents).ok()
        })
        .collect()
}

/// Clear old crash logs (keep last N)
#[cfg_attr(coverage_nightly, coverage(off))]
pub fn cleanup_old_logs(keep_count: usize) -> std::io::Result<usize> {
    let log_dir = PathBuf::from(CRASH_LOG_DIR);

    if !log_dir.exists() {
        return Ok(0);
    }

    let mut crash_files: Vec<_> = fs::read_dir(&log_dir)?
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.file_name().to_string_lossy().starts_with("panic_"))
        .collect();

    // Sort by name descending (newest first)
    crash_files.sort_by(|a, b| b.file_name().cmp(&a.file_name()));

    let mut deleted = 0;
    for entry in crash_files.into_iter().skip(keep_count) {
        fs::remove_file(entry.path())?;
        deleted += 1;
    }

    Ok(deleted)
}

/// ADR-049: Test modules excluded from coverage
#[cfg(test)]
#[cfg_attr(coverage_nightly, coverage(off))]
mod tests {
    use super::*;

    fn create_test_report() -> CrashReport {
        CrashReport {
            timestamp: chrono::DateTime::parse_from_rfc3339("2025-12-19T10:30:00Z")
                .unwrap()
                .with_timezone(&Utc),
            message: "test panic".to_string(),
            location: Some("src/main.rs:42:10".to_string()),
            backtrace: None,
            cognitive_state: None,
            version: "0.1.0".to_string(),
        }
    }

    fn create_test_cognitive_state() -> CognitiveStateSnapshot {
        CognitiveStateSnapshot {
            cycle_count: 100,
            salience_weights: Some(vec![0.5, 0.7, 0.3]),
            active_windows: Some(5),
            connection_drive: Some(0.8),
            current_thought: Some("processing".to_string()),
        }
    }

    #[test]
    fn test_crash_report_serializes_correctly() {
        let report = CrashReport {
            timestamp: Utc::now(),
            message: "test panic".to_string(),
            location: Some("src/main.rs:42:10".to_string()),
            backtrace: None,
            cognitive_state: Some(CognitiveStateSnapshot {
                cycle_count: 100,
                salience_weights: Some(vec![0.5, 0.7, 0.3]),
                active_windows: Some(5),
                connection_drive: Some(0.8),
                current_thought: Some("processing".to_string()),
            }),
            version: "0.1.0".to_string(),
        };

        let json = serde_json::to_string(&report).unwrap();
        assert!(json.contains("test panic"));
        assert!(json.contains("cycle_count"));
        assert!(json.contains("connection_drive"));

        // Roundtrip
        let parsed: CrashReport = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.message, "test panic");
        assert_eq!(parsed.cognitive_state.unwrap().cycle_count, 100);
    }

    #[test]
    fn test_crash_report_filename_format() {
        let report = create_test_report();

        let filename = report.filename();
        assert!(filename.starts_with("panic_"));
        assert!(std::path::Path::new(&filename)
            .extension()
            .is_some_and(|ext| ext.eq_ignore_ascii_case("json")));
        assert!(filename.contains("20251219"));
        assert_eq!(filename, "panic_20251219_103000.json");
    }

    #[test]
    fn test_crash_report_with_cognitive_state() {
        let report = create_test_report();
        assert!(report.cognitive_state.is_none());

        let state = create_test_cognitive_state();
        let report_with_state = report.with_cognitive_state(state);

        assert!(report_with_state.cognitive_state.is_some());
        let cognitive_state = report_with_state.cognitive_state.unwrap();
        assert_eq!(cognitive_state.cycle_count, 100);
        assert_eq!(cognitive_state.connection_drive, Some(0.8));
        assert_eq!(cognitive_state.active_windows, Some(5));
        assert_eq!(
            cognitive_state.current_thought,
            Some("processing".to_string())
        );
        assert_eq!(cognitive_state.salience_weights, Some(vec![0.5, 0.7, 0.3]));
    }

    #[test]
    fn test_crash_report_with_cognitive_state_preserves_other_fields() {
        let report = CrashReport {
            timestamp: chrono::DateTime::parse_from_rfc3339("2025-12-19T10:30:00Z")
                .unwrap()
                .with_timezone(&Utc),
            message: "original message".to_string(),
            location: Some("src/lib.rs:100:5".to_string()),
            backtrace: Some("backtrace info".to_string()),
            cognitive_state: None,
            version: "1.2.3".to_string(),
        };

        let state = CognitiveStateSnapshot::default();
        let report_with_state = report.with_cognitive_state(state);

        assert_eq!(report_with_state.message, "original message");
        assert_eq!(
            report_with_state.location,
            Some("src/lib.rs:100:5".to_string())
        );
        assert_eq!(
            report_with_state.backtrace,
            Some("backtrace info".to_string())
        );
        assert_eq!(report_with_state.version, "1.2.3");
    }

    #[test]
    fn test_cognitive_state_snapshot_default() {
        let state = CognitiveStateSnapshot::default();
        assert_eq!(state.cycle_count, 0);
        assert!(state.salience_weights.is_none());
        assert!(state.active_windows.is_none());
        assert!(state.connection_drive.is_none());
        assert!(state.current_thought.is_none());
    }

    #[test]
    fn test_cognitive_state_snapshot_clone() {
        let state = create_test_cognitive_state();
        let cloned = state.clone();

        assert_eq!(cloned.cycle_count, state.cycle_count);
        assert_eq!(cloned.salience_weights, state.salience_weights);
        assert_eq!(cloned.active_windows, state.active_windows);
        assert_eq!(cloned.connection_drive, state.connection_drive);
        assert_eq!(cloned.current_thought, state.current_thought);
    }

    #[test]
    fn test_crash_report_clone() {
        let report = create_test_report().with_cognitive_state(create_test_cognitive_state());
        let cloned = report.clone();

        assert_eq!(cloned.message, report.message);
        assert_eq!(cloned.location, report.location);
        assert_eq!(cloned.version, report.version);
        assert!(cloned.cognitive_state.is_some());
    }

    #[test]
    fn test_crash_report_debug() {
        let report = create_test_report();
        let debug_str = format!("{:?}", report);
        assert!(debug_str.contains("CrashReport"));
        assert!(debug_str.contains("test panic"));
    }

    #[test]
    fn test_cognitive_state_snapshot_debug() {
        let state = create_test_cognitive_state();
        let debug_str = format!("{:?}", state);
        assert!(debug_str.contains("CognitiveStateSnapshot"));
        assert!(debug_str.contains("100"));
    }

    #[test]
    fn test_crash_report_deserialize_minimal() {
        let json = r#"{
            "timestamp": "2025-12-19T10:30:00Z",
            "message": "minimal crash",
            "location": null,
            "backtrace": null,
            "cognitive_state": null,
            "version": "0.1.0"
        }"#;

        let report: CrashReport = serde_json::from_str(json).unwrap();
        assert_eq!(report.message, "minimal crash");
        assert!(report.location.is_none());
        assert!(report.backtrace.is_none());
        assert!(report.cognitive_state.is_none());
    }

    #[test]
    fn test_crash_report_deserialize_full() {
        let json = r#"{
            "timestamp": "2025-12-19T10:30:00Z",
            "message": "full crash",
            "location": "src/main.rs:1:1",
            "backtrace": "stack trace here",
            "cognitive_state": {
                "cycle_count": 50,
                "salience_weights": [0.1, 0.2],
                "active_windows": 3,
                "connection_drive": 0.5,
                "current_thought": "thinking"
            },
            "version": "2.0.0"
        }"#;

        let report: CrashReport = serde_json::from_str(json).unwrap();
        assert_eq!(report.message, "full crash");
        assert_eq!(report.location, Some("src/main.rs:1:1".to_string()));
        assert_eq!(report.backtrace, Some("stack trace here".to_string()));
        assert_eq!(report.version, "2.0.0");

        let state = report.cognitive_state.unwrap();
        assert_eq!(state.cycle_count, 50);
        assert_eq!(state.salience_weights, Some(vec![0.1, 0.2]));
        assert_eq!(state.active_windows, Some(3));
        assert_eq!(state.connection_drive, Some(0.5));
        assert_eq!(state.current_thought, Some("thinking".to_string()));
    }

    #[test]
    fn test_filename_uses_timestamp() {
        let report1 = CrashReport {
            timestamp: chrono::DateTime::parse_from_rfc3339("2025-01-01T00:00:00Z")
                .unwrap()
                .with_timezone(&Utc),
            message: "test".to_string(),
            location: None,
            backtrace: None,
            cognitive_state: None,
            version: "0.1.0".to_string(),
        };

        let report2 = CrashReport {
            timestamp: chrono::DateTime::parse_from_rfc3339("2025-06-15T23:59:59Z")
                .unwrap()
                .with_timezone(&Utc),
            message: "test".to_string(),
            location: None,
            backtrace: None,
            cognitive_state: None,
            version: "0.1.0".to_string(),
        };

        assert_eq!(report1.filename(), "panic_20250101_000000.json");
        assert_eq!(report2.filename(), "panic_20250615_235959.json");
    }
}
