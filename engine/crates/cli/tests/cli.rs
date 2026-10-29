//! The `opence` tool against a control socket: a thread here stands in for
//! the game's console, answering each command line as the runtime's
//! control server does (`{"ok": true, "text": ...}`, or a refusal).

use std::io::{BufRead, BufReader, Write};
use std::os::unix::net::UnixListener;
use std::path::PathBuf;
use std::process::{Command, Stdio};

use opence_cli::{parse_size, Connection, Error};

type Result = std::result::Result<(), Box<dyn std::error::Error>>;

/// A socket in a folder of its own, answered by [`console`].
struct Standing {
    socket: PathBuf,
}

impl Standing {
    fn start(name: &str) -> std::result::Result<Self, Box<dyn std::error::Error>> {
        let folder = std::env::temp_dir().join(format!("ocl{}{name}", std::process::id()));
        std::fs::create_dir_all(&folder)?;
        let socket = folder.join("c.sock");
        let _ = std::fs::remove_file(&socket);
        let listener = UnixListener::bind(&socket)?;
        std::thread::spawn(move || {
            for stream in listener.incoming() {
                let Ok(stream) = stream else { continue };
                std::thread::spawn(move || console(stream));
            }
        });
        Ok(Self { socket })
    }

    fn opence(&self, arguments: &[&str]) -> std::io::Result<std::process::Output> {
        Command::new(env!("CARGO_BIN_EXE_opence"))
            .arg("--socket")
            .arg(&self.socket)
            .args(arguments)
            .output()
    }
}

impl Drop for Standing {
    fn drop(&mut self) {
        if let Some(folder) = self.socket.parent() {
            let _ = std::fs::remove_dir_all(folder);
        }
    }
}

/// The stand-in console: a command line comes back as `ran <line>`, but
/// `time 99` is refused, as is any structured request.
fn console(stream: std::os::unix::net::UnixStream) {
    let Ok(mut writer) = stream.try_clone() else {
        return;
    };
    for line in BufReader::new(stream).lines() {
        let Ok(line) = line else { return };
        let reply = match serde_json::from_str::<serde_json::Value>(&line) {
            Ok(serde_json::Value::String(command)) if command != "time 99" => {
                serde_json::json!({"ok": true, "text": format!("ran {command}")})
            }
            Ok(serde_json::Value::String(command)) => {
                serde_json::json!({"ok": false, "error": format!("{command}: out of range")})
            }
            _ => serde_json::json!({"ok": false, "error": "command lines only"}),
        };
        if writeln!(writer, "{reply}").is_err() {
            return;
        }
    }
}

fn text(output: &std::process::Output) -> String {
    String::from_utf8_lossy(&output.stdout).into_owned()
}

#[test]
fn lines_are_run_and_refusals_fail() -> Result {
    let console = Standing::start("a")?;
    let ran = console.opence(&["run", "game_speed", "2"])?;
    assert!(ran.status.success(), "{ran:?}");
    assert_eq!(text(&ran).trim(), "ran game_speed 2");
    let json = console.opence(&["run", "--json", "game_speed 1"])?;
    let reply: serde_json::Value = serde_json::from_str(text(&json).trim())?;
    assert_eq!(reply["text"], "ran game_speed 1");

    let refused = console.opence(&["run", "time", "99"])?;
    assert_eq!(refused.status.code(), Some(1));
    assert!(String::from_utf8_lossy(&refused.stderr).contains("99"));
    // (the game's console has no state of its own to report)
    assert_eq!(console.opence(&["state"])?.status.code(), Some(1));
    Ok(())
}

#[test]
fn wrong_usage_and_no_engine_exit_two() -> Result {
    let console = Standing::start("b")?;
    for wrong in [
        &[][..],
        &["run"][..],
        &["teleport_everything"][..],
        &["frame"][..],
        &["frame", "a.png", "--size", "big"][..],
        &["watch", "--backlog", "many"][..],
        &["script"][..],
    ] {
        assert_eq!(console.opence(wrong)?.status.code(), Some(2), "{wrong:?}");
    }
    let nowhere = Command::new(env!("CARGO_BIN_EXE_opence"))
        .args(["--socket", "/nonexistent/c.sock", "state"])
        .output()?;
    assert_eq!(nowhere.status.code(), Some(2));
    assert!(String::from_utf8_lossy(&nowhere.stderr).contains("--headless"));
    assert_eq!(parse_size("640x360")?, (640, 360));
    assert!(matches!(parse_size("640"), Err(Error::Usage(_))));
    Ok(())
}

#[test]
fn the_shell_runs_every_piped_line() -> Result {
    let console = Standing::start("c")?;
    let mut shell = Command::new(env!("CARGO_BIN_EXE_opence"))
        .arg("--socket")
        .arg(&console.socket)
        .arg("shell")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;
    shell
        .stdin
        .take()
        .ok_or("no stdin")?
        .write_all(b"game_speed 1\n# nothing\ntime 99\ngame_speed 2\n")?;
    let output = shell.wait_with_output()?;
    // (a refusal counted, the lines after it still run)
    assert_eq!(output.status.code(), Some(1));
    let out = text(&output);
    assert!(out.contains("error:"), "{out}");
    assert!(out.contains("ran game_speed 2"), "{out}");

    let mut connection = Connection::connect(&console.socket)?;
    let mut out = Vec::new();
    opence_cli::run(&mut connection, "cheat_all_weapons", false, &mut out)?;
    assert_eq!(String::from_utf8(out)?, "ran cheat_all_weapons\n");
    Ok(())
}
