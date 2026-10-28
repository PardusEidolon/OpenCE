//! The dev crate's control socket answered by the game's own console: each
//! command line a client sends is handed to the telnet console
//! ([`Console::ask`]) and its reply sent back as `{"ok": true, "text": ...}`,
//! so `opence run <line>` reaches the original's script compiler, as a line
//! typed at its telnet console does.
//! The console is one session, so the lines are asked one at a time, in the
//! order they arrive.

use std::io::{self, BufRead, BufReader, Read, Write};
use std::os::unix::net::{UnixListener, UnixStream};
use std::path::Path;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex, PoisonError};
use std::time::{Duration, Instant};

use opence_dev::control::{
    bind_private, parse_request, refusal, reply, RateLimit, Request, RequestError, MAX_CONNECTIONS,
    MAX_REQUEST,
};
use serde_json::{Map, Value};

use super::Console;

/// how long a line is given for its reply
const REPLY_LIMIT: Duration = Duration::from_secs(5);
/// how long a reply may take to write before the client is dropped
const WRITE_TIMEOUT: Duration = Duration::from_secs(5);

/// Binds the control socket at `path`, readable by this user alone.
pub fn bind(path: &Path) -> io::Result<UnixListener> {
    bind_private(path)
}

/// Serves `listener` on a thread of its own, each connection on another,
/// at most [`MAX_CONNECTIONS`] at once.
pub fn serve(listener: UnixListener, console: Console) {
    let asking = Arc::new(Mutex::new(console));
    let connections = Arc::new(AtomicUsize::new(0));
    let _ = std::thread::Builder::new()
        .name("control".into())
        .spawn(move || accept(&listener, &asking, &connections));
}

fn accept(listener: &UnixListener, asking: &Arc<Mutex<Console>>, connections: &Arc<AtomicUsize>) {
    for stream in listener.incoming() {
        let Ok(mut stream) = stream else { continue };
        if connections.fetch_add(1, Ordering::AcqRel) >= MAX_CONNECTIONS {
            connections.fetch_sub(1, Ordering::AcqRel);
            let _ = writeln!(
                stream,
                "{}",
                refusal(&format!("at most {MAX_CONNECTIONS} connections at once"))
            );
            continue;
        }
        let asking = Arc::clone(asking);
        let connections = Arc::clone(connections);
        let spawned = std::thread::Builder::new()
            .name("control connection".into())
            .spawn(move || {
                connection(&asking, stream);
                connections.fetch_sub(1, Ordering::AcqRel);
            });
        if spawned.is_err() {
            break;
        }
    }
}

fn connection(asking: &Mutex<Console>, stream: UnixStream) {
    let _ = stream.set_write_timeout(Some(WRITE_TIMEOUT));
    let Ok(reading) = stream.try_clone() else {
        return;
    };
    let mut reader = BufReader::new(reading);
    let mut writer = stream;
    let mut rate = RateLimit::default();
    loop {
        let mut line = String::new();
        let limit = (MAX_REQUEST + 2) as u64;
        match (&mut reader).take(limit).read_line(&mut line) {
            Ok(0) => return,
            Ok(_) => {}
            Err(_) => {
                let _ = writeln!(writer, "{}", refusal("the request is not UTF-8 text"));
                return;
            }
        }
        let line = line.trim_end_matches(['\n', '\r']);
        if line.len() > MAX_REQUEST {
            // (the rest of the line cannot be told from a request)
            let _ = writeln!(writer, "{}", refusal(&RequestError::TooLong.to_string()));
            return;
        }
        if line.trim().is_empty() {
            continue;
        }
        let answered = if rate.allow(Instant::now()) {
            let console = asking.lock().unwrap_or_else(PoisonError::into_inner);
            answer(&console, line)
        } else {
            refusal(&RequestError::TooFrequent.to_string())
        };
        if writeln!(writer, "{answered}").is_err() {
            return;
        }
    }
}

/// Answers one request line: a command line is run on the game's console
/// and its output returned as `text`; anything else is refused.
pub fn answer(console: &Console, line: &str) -> String {
    match parse_request(line) {
        Err(error) => refusal(&error.to_string()),
        Ok(Request::Structured(..)) => refusal("the game's console runs command lines only"),
        Ok(Request::Command(command)) => match console.ask(&command, REPLY_LIMIT) {
            None => refusal("the game's console is not listening yet"),
            Some(output) => {
                let mut fields = Map::new();
                fields.insert("text".into(), Value::String(text(&output)));
                reply(true, fields)
            }
        },
    }
}

/// The console's output as the shell shows it: its lines ended by `\n`,
/// with the blank lines the console writes around each print left out.
fn text(output: &str) -> String {
    output
        .split(['\r', '\n'])
        .filter(|line| !line.is_empty())
        .collect::<Vec<_>>()
        .join("\n")
}
