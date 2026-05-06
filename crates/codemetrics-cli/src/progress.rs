//!
//! Progress indicators and TTY detection.
//! Provides spinners, progress bars, and terminal detection for CLI feedback.

use std::time::{Duration, Instant};

/// Detect whether stderr is a real TTY (not CI, not piped).
pub fn is_tty() -> bool {
    if std::env::var("CI").is_ok()
        || std::env::var("NO_COLOR").is_ok()
        || std::env::var("CODEMETRICS_NO_PROGRESS").is_ok()
    {
        return false;
    }
    // Check if stderr fd 2 is a terminal via isatty syscall
    #[cfg(unix)]
    {
        unsafe { libc_isatty(2) }
    }
    #[cfg(not(unix))]
    {
        false
    }
}

#[cfg(unix)]
unsafe fn libc_isatty(fd: i32) -> bool {
    extern "C" {
        fn isatty(fd: i32) -> i32;
    }
    isatty(fd) != 0
}

const SPINNER_FRAMES: &[&str] = &["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];

/// An overall progress bar for multi-step operations (run_batch).
pub struct Bar {
    total: usize,
    done: usize,
    start: Instant,
    tty: bool,
    last_len: usize,
    current_tool: String,
}

impl Bar {
    /// Create a new progress bar.
    pub fn new(total: usize) -> Self {
        let tty = is_tty();
        Self {
            total,
            done: 0,
            start: Instant::now(),
            tty,
            last_len: 0,
            current_tool: String::new(),
        }
    }

    /// Set the currently running tool name.
    pub fn set_current(&mut self, tool: &str) {
        self.current_tool = tool.to_string();
        self.render();
    }

    /// Advance the progress bar after completing a tool.
    pub fn advance(&mut self, tool: &str, passed: bool, duration_ms: u64) {
        self.done += 1;
        let icon = if passed {
            "  ✓".green().bold()
        } else {
            "  ✗".red().bold()
        };
        let name_col = if passed { tool.normal() } else { tool.red() };
        let dur_str = format_ms(duration_ms);
        if self.tty {
            // Clear spinner line, print result
            eprintln!("\r{:<width$}", "", width = self.last_len);
            eprintln!("\r{} {:<18}  {}", icon, name_col, dur_str.bright_black());
        } else {
            let ci_icon = if passed { "✓" } else { "✗" };
            eprintln!("  {} {:<18}  {}", ci_icon, tool, dur_str);
        }
        self.render();
    }

    /// Render the progress bar (TTY only).
    fn render(&mut self) {
        if !self.tty {
            return;
        }
        let pct = if self.total > 0 {
            self.done * 100 / self.total
        } else {
            0
        };
        let bar_width = 28usize;
        let filled = bar_width * self.done / self.total.max(1);
        let bar: String = "█".repeat(filled) + &"░".repeat(bar_width - filled);
        let elapsed = self.start.elapsed();
        let eta_str = if self.done > 0 {
            let per_item = elapsed / self.done as u32;
            let remaining = per_item * (self.total - self.done) as u32;
            format!("ETA {}", format_duration(remaining))
        } else {
            "ETA --:--".to_string()
        };
        let frame = SPINNER_FRAMES[self.done % SPINNER_FRAMES.len()];
        let running = if self.current_tool.is_empty() {
            String::new()
        } else {
            format!(
                "  {} Running: {}  ({})",
                frame.cyan(),
                self.current_tool.bold(),
                format_elapsed(elapsed)
            )
        };
        let line = format!(
            "\r  [{}] {}  {}%   {}   {}",
            bar.cyan(),
            self.done,
            self.total,
            pct,
            running
        );
        self.last_len = line.len();
        eprint!("{}", line);
    }
}

/// Format milliseconds as a human-readable string.
pub fn format_ms(ms: u64) -> String {
    if ms < 1000 {
        format!("{}ms", ms)
    } else if ms < 60000 {
        format!("{:.1}s", ms as f64 / 1000.0)
    } else {
        format_duration(Duration::from_millis(ms))
    }
}

/// Format a duration as mm:ss.
pub fn format_duration(d: Duration) -> String {
    let secs = d.as_secs();
    let m = secs / 60;
    let s = secs % 60;
    format!("{}:{:02}", m, s)
}

/// Format elapsed time as mm:ss.ms.
pub fn format_elapsed(d: Duration) -> String {
    let secs = d.as_secs();
    let ms = d.subsec_millis();
    let m = secs / 60;
    let s = secs % 60;
    format!("{}:{:02}.{}", m, s, ms / 100)
}

/// Run a function with a spinner (TTY only).
pub fn run_with_spinner<F>(label: &str, f: F) -> i32
where
    F: FnOnce() -> Option<i32>,
{
    let tty = is_tty();
    if tty {
        let done = std::sync::atomic::AtomicBool::new(false);
        let done_ref = &done;
        // Spawn spinner thread
        let label = label.to_string();
        let handle = std::thread::spawn(move || {
            let frames = ["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];
            let mut i = 0;
            while !done_ref.load(std::sync::atomic::Ordering::Relaxed) {
                eprint!("\r  {} {}  ", frames[i % frames.len()].cyan(), label);
                i += 1;
                std::thread::sleep(Duration::from_millis(80));
            }
        });
        let result = f();
        done.store(true, std::sync::atomic::Ordering::Relaxed);
        let _ = handle.join();
        result.unwrap_or(1)
    } else {
        f().unwrap_or(1)
    }
}
