use std::fs::{self, OpenOptions};
use std::io::Write;
use std::process::{Command, Stdio};
use std::thread;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

#[test]
fn appended_record_is_processed_before_the_status_heartbeat() {
    let unique = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let directory = std::env::temp_dir().join(format!("nimble-watch-{unique}"));
    fs::create_dir_all(&directory).unwrap();
    let log = directory.join("nimble.log");
    fs::write(&log, b"").unwrap();

    let started = Instant::now();
    let child = Command::new(env!("CARGO_BIN_EXE_nimble-log-watch"))
        .args([
            "--dir",
            directory.to_str().unwrap(),
            "--levels",
            "E",
            "--duration-seconds",
            "2",
            "--color",
            "never",
        ])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();

    thread::sleep(Duration::from_millis(150));
    let mut output_log = OpenOptions::new().append(true).open(&log).unwrap();
    for index in 0..100 {
        writeln!(
            output_log,
            "[2026-10-05 10:00:00 P1-T2] [test] I: irrelevant heartbeat noise {index}"
        )
        .unwrap();
    }
    writeln!(
        output_log,
        "[2026-10-05 10:00:01 P1-T2] [test] E: continuous append marker"
    )
    .unwrap();
    drop(output_log);

    let output = child.wait_with_output().unwrap();
    let elapsed = started.elapsed();
    fs::remove_dir_all(&directory).unwrap();
    assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert_eq!(stdout.matches("[WATCH][STATE] status=waiting").count(), 1, "{stdout}");
    assert_eq!(stdout.matches("[WATCH][STATE] status=parsing").count(), 1, "{stdout}");
    assert!(stdout.contains("heartbeat_seconds=10"), "{stdout}");
    assert!(stdout.contains("continuous append marker"), "{stdout}");
    assert!(!stdout.contains("irrelevant heartbeat noise"), "{stdout}");
    assert_eq!(stdout.matches("[WATCH][BATCH]").count(), 1, "{stdout}");
    assert!(stdout.contains("status=waiting_for_file_changes"), "{stdout}");
    assert!(stdout.contains("[WATCH][PERF]"), "{stdout}");
    assert!(stdout.contains("read_us="), "{stdout}");
    assert!(stdout.contains("parse_us="), "{stdout}");
    assert!(stdout.contains("group_us="), "{stdout}");
    assert!(stdout.contains("catalog_us="), "{stdout}");
    assert!(stdout.contains("output_us="), "{stdout}");
    assert!(!stdout.contains('\u{1b}'), "color=never must suppress ANSI codes");
    assert!(elapsed < Duration::from_secs(4), "watch took {elapsed:?}");
}

#[test]
fn metrics_only_suppresses_message_content_but_reports_timings() {
    let unique = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let directory = std::env::temp_dir().join(format!("nimble-watch-metrics-{unique}"));
    fs::create_dir_all(&directory).unwrap();
    let log = directory.join("nimble.log");
    fs::write(
        &log,
        b"[2026-10-05 10:00:00 P1-T2] [test] E: secret synthetic marker\n",
    )
    .unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_nimble-log-watch"))
        .args([
            "--dir",
            directory.to_str().unwrap(),
            "--include-existing",
            "--metrics-only",
            "--duration-seconds",
            "1",
            "--status-interval-seconds",
            "10",
            "--color",
            "never",
        ])
        .output()
        .unwrap();
    fs::remove_dir_all(&directory).unwrap();
    assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("[WATCH][PERF]"), "{stdout}");
    assert!(stdout.contains("catalog_us="), "{stdout}");
    assert!(!stdout.contains("secret synthetic marker"), "{stdout}");
}

#[test]
fn large_lag_resumes_at_next_complete_nimble_record() {
    let unique = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let directory = std::env::temp_dir().join(format!("nimble-watch-lag-{unique}"));
    fs::create_dir_all(&directory).unwrap();
    let log = directory.join("nimble.log");
    let mut contents = b"[2026-10-05 10:00:00 P1-T2] [test] E: old record\n".to_vec();
    contents.extend(std::iter::repeat(b'x').take(33000));
    contents.extend_from_slice(
        b"\nnot a timestamp record\n[2026-10-05 10:00:01 P1-T2] [test] E: recovered record\n",
    );
    fs::write(&log, contents).unwrap();

    let output = Command::new(env!("CARGO_BIN_EXE_nimble-log-watch"))
        .args([
            "--dir",
            directory.to_str().unwrap(),
            "--levels",
            "E",
            "--include-existing",
            "--duration-seconds",
            "1",
            "--color",
            "never",
        ])
        .output()
        .unwrap();
    fs::remove_dir_all(&directory).unwrap();
    assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("lag_bytes="), "{stdout}");
    assert!(stdout.contains("resumed_at="), "{stdout}");
    assert!(stdout.contains("recovered record"), "{stdout}");
    assert!(!stdout.contains("old record"), "{stdout}");
    assert!(stdout.contains("status=parsing"), "{stdout}");
}
