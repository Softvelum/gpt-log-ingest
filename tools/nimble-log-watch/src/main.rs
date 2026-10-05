use nimble_log_watch::{group_records, parse_record, CatalogIndex, Level, MessageGroup, Record};
use std::collections::{HashMap, HashSet};
use std::env;
use std::fs::{self, File, Metadata};
use std::io::{self, BufRead, BufReader, Read, Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};
use std::sync::mpsc;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};
use notify::{event::EventKind, RecursiveMode, Watcher};

const MAX_BYTES_PER_FILE_PER_POLL: u64 = 4 * 1024 * 1024;
const MAX_LAG_BYTES: u64 = 32 * 1024;

fn emit(color: bool, tag: &str, message: &str) {
    let code = match tag { "STATE" => "\x1b[36m", "BATCH" => "\x1b[34m", "PERF" => "\x1b[35m", "GAP" => "\x1b[33m", "RECORD" | "ERROR" => "\x1b[31m", _ => "" };
    if color { println!("{code}[WATCH][{tag}]\x1b[0m {message}"); }
    else { println!("[WATCH][{tag}] {message}"); }
}

fn color_enabled(setting: &str) -> bool {
    use std::io::IsTerminal;
    setting == "always" || (setting == "auto" && io::stdout().is_terminal())
}

struct Options {
    directory: PathBuf,
    map: PathBuf,
    status_interval: Duration,
    duration: Option<Duration>,
    levels: HashSet<Level>,
    contains: Option<String>,
    include_existing: bool,
    color: String,
    metrics_only: bool,
}

#[derive(Clone)]
struct FileState {
    path: PathBuf,
    offset: u64,
    size: u64,
}

fn usage() -> &'static str {
    "Usage: nimble-log-watch --dir PATH [--levels E,V,D,I] [--status-interval-seconds N] [--duration-seconds N] [--contains TEXT] [--include-existing] [--color auto|always|never] [--metrics-only] [--map PATH]\n\
     Rust reacts to file changes; the status interval is only a heartbeat. Defaults: levels E, heartbeat 10 seconds, start at current file ends, no duration (until Ctrl-C)."
}

fn parse_options() -> Result<Options, String> {
    let mut args = env::args().skip(1);
    let mut directory = None;
    let mut map = None;
    let mut interval = 10u64;
    let mut duration = None;
    let mut levels = "E".to_owned();
    let mut contains = None;
    let mut include_existing = false;
    let mut color = "auto".to_owned();
    let mut metrics_only = false;
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--help" | "-h" => {
                println!("{}", usage());
                std::process::exit(0);
            }
            "--dir" => directory = Some(PathBuf::from(args.next().ok_or("--dir needs a path")?)),
            "--map" => map = Some(PathBuf::from(args.next().ok_or("--map needs a path")?)),
            "--interval-seconds" | "--status-interval-seconds" => {
                interval = args
                    .next()
                    .ok_or("--interval-seconds needs a positive integer")?
                    .parse()
                    .map_err(|_| "--interval-seconds needs a positive integer")?;
            }
            "--duration-seconds" => {
                duration = Some(Duration::from_secs(
                    args.next()
                        .ok_or("--duration-seconds needs a positive integer")?
                        .parse()
                        .map_err(|_| "--duration-seconds needs a positive integer")?,
                ));
            }
            "--levels" => levels = args.next().ok_or("--levels needs comma-separated codes")?,
            "--contains" => contains = Some(args.next().ok_or("--contains needs text")?),
            "--include-existing" => include_existing = true,
            "--color" => color = args.next().ok_or("--color needs auto, always, or never")?,
            "--metrics-only" => metrics_only = true,
            _ => return Err(format!("unknown option: {arg}\n{}", usage())),
        }
    }
    if interval == 0 || duration.is_some_and(|value| value.is_zero()) {
        return Err("status interval and duration must be positive".to_owned());
    }
    if !["auto", "always", "never"].contains(&color.as_str()) {
        return Err("--color must be auto, always, or never".to_owned());
    }
    let mut selected = HashSet::new();
    for code in levels.split(',') {
        let mut chars = code.trim().chars();
        let level = chars
            .next()
            .and_then(Level::from_code)
            .ok_or_else(|| format!("unsupported level code: {code}; use E, V, D, or I"))?;
        if chars.next().is_some() {
            return Err(format!("level must be one code: {code}"));
        }
        selected.insert(level);
    }
    if selected.is_empty() {
        return Err("select at least one level code".to_owned());
    }
    let directory = directory.ok_or_else(|| format!("--dir is required\n{}", usage()))?;
    let default_map = PathBuf::from("logger/search-index.json");
    let map = map.unwrap_or_else(|| {
        if default_map.is_file() {
            default_map
        } else {
            Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("../..")
                .join("logger/search-index.json")
        }
    });
    Ok(Options {
        directory,
        map,
        status_interval: Duration::from_secs(interval),
        duration,
        levels: selected,
        contains,
        include_existing,
        color,
        metrics_only,
    })
}

fn is_runtime_log(path: &Path) -> bool {
    let Some(name) = path.file_name().and_then(|value| value.to_str()) else {
        return false;
    };
    name == "nimble.log" || (name.starts_with("nimble-") && name.ends_with(".log"))
}

#[cfg(unix)]
fn identity(_path: &Path, metadata: &Metadata) -> String {
    use std::os::unix::fs::MetadataExt;
    format!("{}:{}", metadata.dev(), metadata.ino())
}

#[cfg(not(unix))]
fn identity(path: &Path, _metadata: &Metadata) -> String {
    path.canonicalize()
        .unwrap_or_else(|_| path.to_path_buf())
        .to_string_lossy()
        .into_owned()
}

fn list_files(directory: &Path) -> io::Result<Vec<PathBuf>> {
    let mut files = Vec::new();
    for entry in fs::read_dir(directory)? {
        let entry = entry?;
        let path = entry.path();
        if is_runtime_log(&path) && entry.file_type()?.is_file() {
            files.push(path);
        }
    }
    files.sort();
    Ok(files)
}

fn timestamp_now() -> String {
    let seconds = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    format!("unix:{seconds}")
}

fn initialize(options: &Options) -> Result<HashMap<String, FileState>, String> {
    let mut states = HashMap::new();
    for path in list_files(&options.directory).map_err(|error| error.to_string())? {
        let metadata = fs::metadata(&path).map_err(|error| format!("{}: {error}", path.display()))?;
        let id = identity(&path, &metadata);
        let offset = if options.include_existing { 0 } else { metadata.len() };
        states.insert(
            id,
            FileState {
                path,
                offset,
                size: metadata.len(),
            },
        );
    }
    Ok(states)
}

fn nimble_record_prefix(line: &[u8]) -> bool {
    line.len() >= 21
        && line[0] == b'['
        && line[5] == b'-'
        && line[8] == b'-'
        && line[11] == b' '
        && line[14] == b':'
        && line[17] == b':'
        && line[20] == b' '
        && line[1..5].iter().all(u8::is_ascii_digit)
        && line[6..8].iter().all(u8::is_ascii_digit)
        && line[9..11].iter().all(u8::is_ascii_digit)
        && line[12..14].iter().all(u8::is_ascii_digit)
        && line[15..17].iter().all(u8::is_ascii_digit)
        && line[18..20].iter().all(u8::is_ascii_digit)
}

fn resync_to_record(path: &Path, offset: u64, size: u64) -> Result<u64, String> {
    let start = offset.max(size.saturating_sub(MAX_LAG_BYTES));
    let mut file = File::open(path).map_err(|error| format!("{}: {error}", path.display()))?;
    let at_line_start = if start > offset && start > 0 {
        file.seek(SeekFrom::Start(start - 1))
            .map_err(|error| format!("{}: {error}", path.display()))?;
        let mut previous = [0u8; 1];
        file.read_exact(&mut previous)
            .map_err(|error| format!("{}: {error}", path.display()))?;
        previous[0] == b'\n'
    } else {
        true
    };
    file.seek(SeekFrom::Start(start))
        .map_err(|error| format!("{}: {error}", path.display()))?;
    let mut data = Vec::with_capacity(size.saturating_sub(start) as usize);
    file.take(size.saturating_sub(start))
        .read_to_end(&mut data)
        .map_err(|error| format!("{}: {error}", path.display()))?;
    let mut cursor = 0usize;
    if !at_line_start {
        let Some(newline) = data.iter().position(|byte| *byte == b'\n') else {
            return Ok(size);
        };
        cursor = newline + 1;
    }
    while cursor < data.len() {
        let rest = &data[cursor..];
        let line_end = rest.iter().position(|byte| *byte == b'\n').unwrap_or(rest.len());
        if nimble_record_prefix(&rest[..line_end]) {
            return Ok(start + cursor as u64);
        }
        if line_end == rest.len() {
            break;
        }
        cursor += line_end + 1;
    }
    Ok(size)
}

fn scan(
    options: &Options,
    states: &mut HashMap<String, FileState>,
    index: &CatalogIndex,
    color: bool,
    metrics_only: bool,
    report_state: bool,
) -> Result<(), String> {
    let total_started = Instant::now();
    let paths = list_files(&options.directory).map_err(|error| error.to_string())?;
    if report_state {
        emit(color, "STATE", &format!("status=parsing observed={} files_pending={}", timestamp_now(), paths.len()));
    }
    let mut current_ids = HashSet::new();
    let mut records = Vec::new();
    let mut complete_lines = 0usize;
    let mut unclassified = 0usize;
    let mut new_files = 0usize;
    let mut gaps = Vec::new();
    let mut read_time = Duration::ZERO;
    let mut parse_time = Duration::ZERO;
    let mut bytes_read = 0u64;

    for path in paths {
        let metadata = match fs::metadata(&path) {
            Ok(value) => value,
            Err(error) => {
                gaps.push(format!("cannot stat {}: {error}", path.display()));
                continue;
            }
        };
        let id = identity(&path, &metadata);
        current_ids.insert(id.clone());
        let mut state = match states.remove(&id) {
            Some(mut existing) => {
                existing.path = path.clone();
                existing
            }
            None => {
                new_files += 1;
                FileState {
                    path: path.clone(),
                    offset: 0,
                    size: 0,
                }
            }
        };
        if metadata.len() < state.offset {
            gaps.push(format!(
                "truncated {} from offset {} to size {}; resumed at start",
                path.display(),
                state.offset,
                metadata.len()
            ));
            state.offset = 0;
        }
        let lag = metadata.len().saturating_sub(state.offset);
        if lag > MAX_LAG_BYTES {
            let resumed_at = resync_to_record(&path, state.offset, metadata.len())?;
            gaps.push(format!(
                "lag_bytes={lag} path={:?} skipped_range={}:{} skipped_bytes={} resumed_at={resumed_at}",
                path,
                state.offset,
                resumed_at,
                resumed_at.saturating_sub(state.offset)
            ));
            state.offset = resumed_at;
        }
        let start = state.offset;
        let limit = metadata.len().saturating_sub(start).min(MAX_BYTES_PER_FILE_PER_POLL);
        if limit > 0 {
            let mut file = File::open(&path).map_err(|error| format!("{}: {error}", path.display()))?;
            file.seek(SeekFrom::Start(start))
                .map_err(|error| format!("{}: {error}", path.display()))?;
            let mut reader = BufReader::new(file.take(limit));
            let mut consumed = 0u64;
            loop {
                let mut bytes = Vec::new();
                let read_started = Instant::now();
                let length = reader
                    .read_until(b'\n', &mut bytes)
                    .map_err(|error| format!("{}: {error}", path.display()))?;
                read_time += read_started.elapsed();
                bytes_read += length as u64;
                if length == 0 {
                    break;
                }
                if bytes.last() != Some(&b'\n') {
                    break; // Keep the incomplete line at the same offset for the next poll.
                }
                let line_offset = start + consumed;
                consumed += length as u64;
                complete_lines += 1;
                let parse_started = Instant::now();
                let line = String::from_utf8_lossy(&bytes[..bytes.len() - 1]);
                let parsed = parse_record(line.trim_end_matches('\r'));
                parse_time += parse_started.elapsed();
                let Some(parsed) = parsed else {
                    unclassified += 1;
                    continue;
                };
                if !options.levels.contains(&parsed.level) {
                    continue;
                }
                if options
                    .contains
                    .as_ref()
                    .is_some_and(|needle| !parsed.body.contains(needle))
                {
                    continue;
                }
                records.push(Record::new(
                    parsed.level,
                    &parsed.component,
                    &parsed.body,
                    &parsed.timestamp,
                    &path.display().to_string(),
                    line_offset,
                ));
            }
            state.offset = start + consumed;
        }
        state.size = metadata.len();
        states.insert(id, state);
    }

    let mut missing_ids = Vec::new();
    for (id, state) in states.iter() {
        if !current_ids.contains(id) && state.size > 0 {
            gaps.push(format!(
                "previously watched file identity is no longer available: {} at offset {}",
                state.path.display(),
                state.offset
            ));
            missing_ids.push(id.clone());
        }
    }
    for id in missing_ids {
        states.remove(&id);
    }

    let group_started = Instant::now();
    let groups = group_records(records);
    let group_time = group_started.elapsed();
    if groups.is_empty() && gaps.is_empty() && !metrics_only {
        if report_state {
            emit(color, "STATE", &format!("status=waiting_for_file_changes observed={} heartbeat_seconds={}", timestamp_now(), options.status_interval.as_secs()));
        }
        return Ok(());
    }
    emit(color, "BATCH", &format!(
        "observed={} files={} new_files={} complete_lines={} selected_groups={} unclassified={} gaps={}",
        timestamp_now(),
        current_ids.len(),
        new_files,
        complete_lines,
        groups.len(),
        unclassified,
        gaps.len()
    ));
    for gap in gaps {
        emit(color, "GAP", &format!("coverage_gap={gap:?}"));
    }
    let output_started = Instant::now();
    let mut catalog_time = Duration::ZERO;
    for group in groups {
        catalog_time += print_group(&group, index, color, metrics_only);
    }
    io::stdout().flush().map_err(|error| error.to_string())?;
    let output_time = output_started.elapsed().saturating_sub(catalog_time);
    let total = total_started.elapsed();
    emit(color, "PERF", &format!("bytes_read={bytes_read} read_us={} parse_us={} group_us={} catalog_us={} output_us={} total_us={} throughput_Bps={:.0}",
        read_time.as_micros(), parse_time.as_micros(), group_time.as_micros(), catalog_time.as_micros(), output_time.as_micros(), total.as_micros(),
        if total.is_zero() { 0.0 } else { bytes_read as f64 / total.as_secs_f64() }));
    if report_state {
        emit(color, "STATE", &format!("status=waiting_for_file_changes observed={} heartbeat_seconds={} last_action=scan_completed", timestamp_now(), options.status_interval.as_secs()));
    }
    Ok(())
}

fn print_group(group: &MessageGroup, index: &CatalogIndex, color: bool, metrics_only: bool) -> Duration {
    let started = Instant::now();
    let candidates = index.candidates(&group.body, group.level, 3);
    let catalog_elapsed = started.elapsed();
    if metrics_only { return catalog_elapsed; }
    let candidate_text = candidates
        .iter()
        .map(|candidate| {
            format!(
                "{}:{}:{}:{:?}",
                candidate.entry.id,
                candidate.score,
                if candidate.level_agrees { "level-match" } else { "level-diff" },
                candidate.entry.title
            )
        })
        .collect::<Vec<_>>()
        .join(",");
    emit(color, "RECORD", &format!(
        "message level={} component={:?} count={} first={:?}@{} {} last={:?}@{} {} catalog_candidates=[{}] body={:?}",
        group.level.code(),
        group.component,
        group.count,
        group.first.path,
        group.first.offset,
        group.first.timestamp,
        group.last.path,
        group.last.offset,
        group.last.timestamp,
        candidate_text,
        group.body
    ));
    catalog_elapsed
}

fn run() -> Result<(), String> {
    let options = parse_options()?;
    if !options.directory.is_dir() {
        return Err(format!("log directory is not readable: {}", options.directory.display()));
    }
    let index = CatalogIndex::load(&options.map)?;
    let mut states = initialize(&options)?;
    let color = color_enabled(&options.color);
    let (sender, receiver) = mpsc::channel();
    let mut watcher = notify::recommended_watcher(move |event| { let _ = sender.send(event); })
        .map_err(|error| format!("cannot create filesystem watcher: {error}"))?;
    watcher.watch(&options.directory, RecursiveMode::NonRecursive)
        .map_err(|error| format!("cannot watch {}: {error}", options.directory.display()))?;
    let mut level_codes: Vec<_> = options.levels.iter().map(|level| level.code()).collect();
    level_codes.sort_unstable();
    emit(color, "STATE", &format!(
        "status=starting observed={} directory={:?} files={} levels={:?} backend={:?} heartbeat_seconds={} backlog={}",
        timestamp_now(),
        options.directory,
        states.len(),
        level_codes,
        notify::RecommendedWatcher::kind(),
        options.status_interval.as_secs(),
        if options.include_existing { "included" } else { "excluded" }
    ));
    io::stdout().flush().map_err(|error| error.to_string())?;
    let start = Instant::now();
    scan(&options, &mut states, &index, color, options.metrics_only, true)?;
    let mut last_heartbeat = Instant::now();
    loop {
        if options.duration.is_some_and(|duration| start.elapsed() >= duration) {
            break;
        }
        let until_heartbeat = options.status_interval.saturating_sub(last_heartbeat.elapsed());
        let timeout = options.duration.map_or(until_heartbeat, |duration| until_heartbeat.min(duration.saturating_sub(start.elapsed())));
        match receiver.recv_timeout(timeout) {
            Ok(Ok(event)) => {
                let mut should_scan = !matches!(&event.kind, EventKind::Access(_));
                while let Ok(queued) = receiver.try_recv() {
                    match queued {
                        Ok(next) => {
                            should_scan |= !matches!(&next.kind, EventKind::Access(_));
                        }
                        Err(error) => emit(color, "ERROR", &format!("filesystem notification error; coverage may be incomplete: {error}")),
                    }
                }
                let heartbeat_due = last_heartbeat.elapsed() >= options.status_interval;
                if should_scan || heartbeat_due {
                    scan(&options, &mut states, &index, color, options.metrics_only, heartbeat_due)?;
                }
                if last_heartbeat.elapsed() >= options.status_interval {
                    if !heartbeat_due {
                        emit(color, "STATE", &format!(
                            "status=waiting_for_file_changes observed={} heartbeat_seconds={} last_action=busy_reconciled",
                            timestamp_now(), options.status_interval.as_secs()
                        ));
                    }
                    last_heartbeat = Instant::now();
                }
            }
            Ok(Err(error)) => emit(color, "ERROR", &format!("filesystem notification error; coverage may be incomplete: {error}")),
            Err(mpsc::RecvTimeoutError::Timeout) => {
                if last_heartbeat.elapsed() >= options.status_interval {
                    scan(&options, &mut states, &index, color, options.metrics_only, true)?;
                    last_heartbeat = Instant::now();
                }
            }
            Err(mpsc::RecvTimeoutError::Disconnected) => return Err("filesystem notification channel disconnected".to_owned()),
        }
        if options.duration.is_some_and(|duration| start.elapsed() >= duration) {
            break;
        }
    }
    emit(color, "STATE", &format!("status=stopped observed={}", timestamp_now()));
    Ok(())
}

fn main() {
    if let Err(error) = run() {
        eprintln!("[WATCH][ERROR] nimble-log-watch: {error}");
        std::process::exit(2);
    }
}
