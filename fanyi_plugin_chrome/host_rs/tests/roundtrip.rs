//! End-to-end tests that drive the built host over stdio with a fake opencode.

use serde_json::{json, Value};
use std::io::{Read, Write};
use std::process::{Command, Stdio};

const HOST: &str = env!("CARGO_BIN_EXE_oc-translate-host");
const FAKE: &str = env!("CARGO_BIN_EXE_fake-opencode");

fn frame(message: &Value) -> Vec<u8> {
    let json = serde_json::to_vec(message).unwrap();
    let mut bytes = Vec::with_capacity(4 + json.len());
    bytes.extend_from_slice(&(json.len() as u32).to_le_bytes());
    bytes.extend_from_slice(&json);
    bytes
}

fn parse_frames(bytes: &[u8]) -> Vec<Value> {
    let mut frames = Vec::new();
    let mut offset = 0;
    while offset + 4 <= bytes.len() {
        let length = u32::from_le_bytes([
            bytes[offset],
            bytes[offset + 1],
            bytes[offset + 2],
            bytes[offset + 3],
        ]) as usize;
        offset += 4;
        if offset + length > bytes.len() {
            break;
        }
        if let Ok(value) = serde_json::from_slice::<Value>(&bytes[offset..offset + length]) {
            frames.push(value);
        }
        offset += length;
    }
    frames
}

/// Run the host with the given extra environment, write the requests, close
/// stdin and collect every response frame.
fn run_host(extra_env: &[(&str, &str)], requests: &[Value]) -> Vec<Value> {
    let mut command = Command::new(HOST);
    command
        .env("OPENCODE_BIN", FAKE)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::null());
    for (key, value) in extra_env {
        command.env(key, value);
    }

    let mut child = command.spawn().expect("spawn host");
    {
        let mut stdin = child.stdin.take().expect("host stdin");
        for request in requests {
            stdin.write_all(&frame(request)).expect("write request");
        }
        // Dropping stdin closes the channel; the host drains then exits.
    }

    let mut stdout = Vec::new();
    child
        .stdout
        .take()
        .expect("host stdout")
        .read_to_end(&mut stdout)
        .expect("read host stdout");
    let _ = child.wait();
    parse_frames(&stdout)
}

fn terminal<'a>(frames: &'a [Value], id: &str) -> &'a Value {
    frames
        .iter()
        .find(|frame| frame["id"] == json!(id) && frame["last"] == json!(true))
        .unwrap_or_else(|| panic!("no terminal frame for {id}: {frames:?}"))
}

#[test]
fn health_and_translate_round_trip() {
    let frames = run_host(
        &[("FAKE_MODE", "aligned")],
        &[
            json!({ "id": "h1", "type": "health" }),
            json!({
                "id": "t1",
                "type": "translate",
                "segments": [{ "index": 0, "text": "hello" }, { "index": 1, "text": "world" }],
                "options": { "targetLang": "zh-CN" }
            }),
        ],
    );

    let health = terminal(&frames, "h1");
    assert_eq!(health["type"], json!("health"));
    assert_eq!(health["data"]["host"]["ok"], json!(true));
    assert_eq!(health["data"]["opencode"]["ok"], json!(true));

    let translate = terminal(&frames, "t1");
    assert_eq!(translate["type"], json!("translate"));
    assert_eq!(
        translate["translations"],
        json!([{ "index": 0, "text": "T:hello" }, { "index": 1, "text": "T:world" }])
    );
}

#[test]
fn received_progress_frames_precede_the_terminal_frame() {
    let frames = run_host(
        &[("FAKE_MODE", "aligned")],
        &[json!({
            "id": "p1",
            "type": "translate",
            "segments": [{ "index": 0, "text": "a" }],
            "options": {}
        })],
    );

    let progress: Vec<&Value> = frames
        .iter()
        .filter(|frame| frame["id"] == json!("p1") && frame["type"] == json!("progress"))
        .collect();
    assert!(!progress.is_empty());
    assert!(progress.iter().all(|frame| frame["last"] == json!(false)));

    let terminal = terminal(&frames, "p1");
    assert_eq!(terminal["last"], json!(true));
    // Sequence numbers strictly increase within a request.
    let mut seqs: Vec<u64> = frames
        .iter()
        .filter(|frame| frame["id"] == json!("p1"))
        .map(|frame| frame["seq"].as_u64().unwrap())
        .collect();
    let sorted = {
        let mut copy = seqs.clone();
        copy.sort_unstable();
        copy
    };
    seqs.sort_unstable();
    assert_eq!(seqs, sorted);
}

#[test]
fn identical_segments_are_deduplicated() {
    let frames = run_host(
        &[("FAKE_MODE", "aligned")],
        &[json!({
            "id": "d1",
            "type": "translate",
            "segments": [{ "index": 0, "text": "same" }, { "index": 1, "text": "same" }],
            "options": {}
        })],
    );

    let translate = terminal(&frames, "d1");
    assert_eq!(
        translate["translations"],
        json!([{ "index": 0, "text": "T:same" }, { "index": 1, "text": "T:same" }])
    );
}

#[test]
fn region_errors_surface_as_model_unavailable() {
    let frames = run_host(
        &[("FAKE_MODE", "region")],
        &[json!({
            "id": "r1",
            "type": "translate",
            "segments": [{ "index": 0, "text": "x" }],
            "options": {}
        })],
    );

    let error = terminal(&frames, "r1");
    assert_eq!(error["type"], json!("error"));
    assert_eq!(error["error"]["code"], json!("MODEL_UNAVAILABLE"));
}

#[test]
fn omitted_segments_are_recovered_on_retry() {
    let frames = run_host(
        &[("FAKE_MODE", "missing")],
        &[json!({
            "id": "m1",
            "type": "translate",
            "segments": [{ "index": 0, "text": "a" }, { "index": 1, "text": "b" }],
            "options": {}
        })],
    );

    let translate = terminal(&frames, "m1");
    assert_eq!(
        translate["translations"],
        json!([{ "index": 0, "text": "T:a" }, { "index": 1, "text": "T:b" }])
    );
}

#[test]
fn a_hanging_opencode_is_reported_as_timeout() {
    let frames = run_host(
        &[("FAKE_MODE", "hang"), ("TRANSLATE_TIMEOUT_MS", "400")],
        &[json!({
            "id": "x1",
            "type": "translate",
            "segments": [{ "index": 0, "text": "a" }],
            "options": {}
        })],
    );

    let error = terminal(&frames, "x1");
    assert_eq!(error["type"], json!("error"));
    assert_eq!(error["error"]["code"], json!("TIMEOUT"));
}

#[test]
fn unknown_request_type_is_bad_request() {
    let frames = run_host(&[], &[json!({ "id": "b1", "type": "nope" })]);
    let error = terminal(&frames, "b1");
    assert_eq!(error["error"]["code"], json!("BAD_REQUEST"));
}

#[test]
fn the_fake_passes_model_and_agent_through() {
    let args_file = std::env::temp_dir().join("host_rs_fake_args.json");
    if args_file.exists() {
        std::fs::remove_file(&args_file).unwrap();
    }
    let args_path = args_file.to_string_lossy().to_string();

    run_host(
        &[("FAKE_MODE", "aligned"), ("FAKE_ARGS_FILE", &args_path)],
        &[json!({
            "id": "a1",
            "type": "translate",
            "segments": [{ "index": 0, "text": "a" }],
            "options": { "model": "prov/explicit#high", "agent": "translator" }
        })],
    );

    let recorded: Vec<String> =
        serde_json::from_str(&std::fs::read_to_string(&args_file).expect("args file")).unwrap();
    assert!(recorded.contains(&"run".to_string()));
    assert!(recorded.contains(&"--model".to_string()));
    assert!(recorded.contains(&"prov/explicit#high".to_string()));
    assert!(recorded.contains(&"--agent".to_string()));
    assert!(recorded.contains(&"translator".to_string()));
    let _ = std::fs::remove_file(&args_file);
}
