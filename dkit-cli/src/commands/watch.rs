use std::path::{Path, PathBuf};
use std::sync::mpsc;
use std::time::{Duration, Instant};

use anyhow::{bail, Result};
use colored::Colorize;
use notify::{Event, EventKind, RecommendedWatcher, RecursiveMode, Watcher};

/// 디바운스 대기 시간 (ms)
const DEBOUNCE_MS: u64 = 300;

/// watch 모드: 파일 변경을 감지하여 콜백을 재실행한다.
///
/// `watch_paths`는 감시할 파일/디렉토리 목록이다.
/// `run_fn`은 변경 감지 시 호출할 클로저이다.
pub fn run_watch<F>(watch_paths: &[PathBuf], mut run_fn: F) -> Result<()>
where
    F: FnMut() -> Result<()>,
{
    if watch_paths.is_empty() {
        bail!("--watch requires at least one file input to watch");
    }

    // Verify all paths exist
    for p in watch_paths {
        if !p.exists() {
            bail!("Watch path does not exist: {}", p.display());
        }
    }

    // Set up Ctrl+C handler
    let (ctrlc_tx, ctrlc_rx) = mpsc::channel();
    ctrlc::set_handler(move || {
        let _ = ctrlc_tx.send(());
    })
    .map_err(|e| anyhow::anyhow!("Failed to set Ctrl+C handler: {e}"))?;

    // Initial run
    eprintln!(
        "{} Watching {} path(s) for changes. Press {} to stop.",
        "watch:".cyan().bold(),
        watch_paths.len(),
        "Ctrl+C".yellow()
    );
    print_separator();
    if let Err(e) = run_fn() {
        eprintln!("{} {e:#}", "error:".red().bold());
    }

    // Set up file watcher
    let (tx, rx) = mpsc::channel();
    let mut watcher: RecommendedWatcher =
        notify::recommended_watcher(move |res: notify::Result<Event>| {
            if let Ok(event) = res {
                if is_relevant_event(&event) {
                    let _ = tx.send(event);
                }
            }
        })?;

    for p in watch_paths {
        let mode = if p.is_dir() {
            RecursiveMode::Recursive
        } else {
            RecursiveMode::NonRecursive
        };
        // Watch the parent directory for file-level events
        let watch_target = if p.is_file() {
            p.parent().unwrap_or(p)
        } else {
            p
        };
        watcher.watch(watch_target, mode)?;
    }

    // Collect which file paths we care about (for file-level filtering)
    let watched_files: Vec<PathBuf> = watch_paths
        .iter()
        .filter(|p| p.is_file())
        .map(|p| p.canonicalize().unwrap_or_else(|_| p.to_path_buf()))
        .collect();
    let watched_dirs: Vec<PathBuf> = watch_paths
        .iter()
        .filter(|p| p.is_dir())
        .map(|p| p.canonicalize().unwrap_or_else(|_| p.to_path_buf()))
        .collect();

    let mut last_run = Instant::now();

    loop {
        // Check for Ctrl+C
        if ctrlc_rx.try_recv().is_ok() {
            eprintln!();
            eprintln!("{} Stopped watching.", "watch:".cyan().bold());
            return Ok(());
        }

        // Wait for file change events with timeout
        match rx.recv_timeout(Duration::from_millis(200)) {
            Ok(event) => {
                // Check if the event is for a watched path
                if !is_watched_path(&event, &watched_files, &watched_dirs) {
                    continue;
                }

                // Debounce: drain remaining events
                let now = Instant::now();
                if now.duration_since(last_run) < Duration::from_millis(DEBOUNCE_MS) {
                    continue;
                }

                // Drain pending events
                while rx.try_recv().is_ok() {}

                // Small delay to let the filesystem settle
                std::thread::sleep(Duration::from_millis(50));

                last_run = Instant::now();

                // Clear screen and re-run
                clear_screen();
                print_change_notice(&event);
                print_separator();
                if let Err(e) = run_fn() {
                    eprintln!("{} {e:#}", "error:".red().bold());
                }
            }
            Err(mpsc::RecvTimeoutError::Timeout) => continue,
            Err(mpsc::RecvTimeoutError::Disconnected) => break,
        }
    }

    Ok(())
}

/// 변경 이벤트가 실제로 관심 있는 이벤트인지 확인
fn is_relevant_event(event: &Event) -> bool {
    matches!(
        event.kind,
        EventKind::Modify(_) | EventKind::Create(_) | EventKind::Remove(_)
    )
}

/// 이벤트의 경로가 감시 대상에 해당하는지 확인
fn is_watched_path(event: &Event, watched_files: &[PathBuf], watched_dirs: &[PathBuf]) -> bool {
    // If no specific files/dirs (watching parent dirs), all events are relevant
    if watched_files.is_empty() && watched_dirs.is_empty() {
        return true;
    }

    for event_path in &event.paths {
        let canonical = event_path
            .canonicalize()
            .unwrap_or_else(|_| event_path.to_path_buf());

        // Check file match
        for wf in watched_files {
            if &canonical == wf {
                return true;
            }
        }

        // Check dir match (event path is under a watched directory)
        for wd in watched_dirs {
            if canonical.starts_with(wd) {
                return true;
            }
        }
    }

    false
}

/// 터미널 화면을 클리어한다.
fn clear_screen() {
    // ANSI escape: clear screen + move cursor to top-left
    eprint!("\x1B[2J\x1B[H");
}

/// 변경 파일 정보를 출력한다.
fn print_change_notice(event: &Event) {
    let changed_files: Vec<String> = event
        .paths
        .iter()
        .filter_map(|p| p.file_name())
        .map(|f| f.to_string_lossy().to_string())
        .collect();

    if changed_files.is_empty() {
        eprintln!("{} File changed, re-running...", "watch:".cyan().bold());
    } else {
        eprintln!(
            "{} {} changed, re-running...",
            "watch:".cyan().bold(),
            changed_files.join(", ")
        );
    }
}

fn print_separator() {
    eprintln!("{}", "─".repeat(40).dimmed());
}

/// 입력 파일 경로를 감시 대상 경로 목록으로 변환한다.
pub fn collect_watch_targets(input_paths: &[PathBuf], extra_paths: &[PathBuf]) -> Vec<PathBuf> {
    let mut targets = Vec::new();
    for p in input_paths {
        if p.as_os_str() != "-" && p.exists() {
            targets.push(p.clone());
        }
    }
    for p in extra_paths {
        targets.push(p.clone());
    }
    targets
}

/// 단일 입력 문자열에서 감시 대상 경로 목록을 생성한다.
pub fn collect_watch_targets_from_input(input: &str, extra_paths: &[PathBuf]) -> Vec<PathBuf> {
    let mut targets = Vec::new();
    if input != "-" {
        let p = Path::new(input);
        if p.exists() {
            targets.push(p.to_path_buf());
        }
    }
    for p in extra_paths {
        targets.push(p.clone());
    }
    targets
}
