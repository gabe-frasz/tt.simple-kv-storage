use std::fs;
use std::io::Write;
use std::process::{Command, Stdio};

fn get_bin_path() -> &'static str {
    env!("CARGO_BIN_EXE_banco-memoria")
}

fn run_pipeline(commands: &[&str]) -> Vec<String> {
    let mut child = Command::new(get_bin_path())
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to start banco-memoria process");

    {
        let stdin = child.stdin.as_mut().expect("Failed to open stdin");
        for cmd in commands {
            writeln!(stdin, "{cmd}").expect("Failed to write to stdin");
        }
    }

    let output = child
        .wait_with_output()
        .expect("Failed to wait for process exit");

    let stdout = String::from_utf8_lossy(&output.stdout);
    stdout
        .lines()
        .map(str::trim)
        .filter(|l| !l.is_empty())
        .map(str::to_string)
        .collect()
}

#[test]
fn test_cli_pipe_basic() {
    let output = run_pipeline(&[
        "ADD key1 value1",
        "GET key1",
        "ADD key1 new_value",
        "GET key1",
        "EXIT",
    ]);

    assert_eq!(output, vec!["OK", "value1", "OK", "new_value"]);
}

#[test]
fn test_cli_pipe_errors() {
    let output = run_pipeline(&[
        "GET nonexistent_key",
        "INVALID_COMMAND",
        "ADD",
        "ADD key_without_value",
        "GET",
        "GET key extra",
        "EXIT",
    ]);

    assert_eq!(
        output,
        vec![
            "ERROR: key not found",
            "ERROR: unknown command: INVALID_COMMAND",
            "ERROR: key not provided",
            "ERROR: value not provided",
            "ERROR: key not provided",
            "ERROR: too many arguments",
        ]
    );
}

#[test]
fn test_cli_pipe_casos_teste() {
    let content = fs::read_to_string("casos_teste.txt")
        .expect("Could not read casos_teste.txt file");

    let mut commands = Vec::new();
    let mut expected = Vec::new();

    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }

        if let Some(rest) = trimmed.strip_prefix("# Expected:") {
            expected.push(rest.trim().to_string());
        } else if trimmed.starts_with('#') {
            continue;
        } else {
            commands.push(trimmed);
        }
    }

    let output = run_pipeline(&commands);

    assert_eq!(
        output.len(),
        expected.len(),
        "Output count ({}) differs from expected ({})",
        output.len(),
        expected.len()
    );

    for (i, (exp, act)) in expected.iter().zip(output.iter()).enumerate() {
        assert_eq!(
            exp, act,
            "Mismatch at line {i}: expected '{exp}', got '{act}'"
        );
    }
}

#[test]
fn test_cli_pipe_casos_teste_json() {
    let Ok(content) = fs::read_to_string("casos_teste_json.txt") else {
        return;
    };

    let mut commands = Vec::new();
    let mut expected = Vec::new();

    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }

        if let Some(rest) = trimmed.strip_prefix("# Expected:") {
            expected.push(rest.trim().to_string());
        } else if trimmed.starts_with('#') {
            continue;
        } else {
            commands.push(trimmed);
        }
    }

    let output = run_pipeline(&commands);

    assert_eq!(
        output.len(),
        expected.len(),
        "JSON output count ({}) differs from expected ({})",
        output.len(),
        expected.len()
    );

    for (i, (exp, act)) in expected.iter().zip(output.iter()).enumerate() {
        assert_eq!(
            exp, act,
            "JSON mismatch at line {i}: expected '{exp}', got '{act}'"
        );
    }
}
