//! `opence-wasm module.wasm`: runs a WASI command module in the game's
//! runtime and exits with its code, its descriptors 1 and 2 copied to this
//! process's. The reference traces built as wasm32 run through it
//! (tools/engine_wasm_cc.py), so a trace compares the wasm build with the
//! native one through the same runtime the game uses.
//!
//! `OPENCE_WASM_CACHE` names a folder for Wasmtime's cache of compiled
//! modules; without it each module compiles afresh.

use std::io::Write;
use std::path::PathBuf;
use std::process::ExitCode;

use opence_runtime::Runtime;

fn run(module: &PathBuf) -> Result<i32, String> {
    let wasm = std::fs::read(module).map_err(|error| format!("{}: {error}", module.display()))?;
    let cache = std::env::var_os("OPENCE_WASM_CACHE").map(PathBuf::from);
    let runtime = Runtime::new(cache.as_deref()).map_err(|error| error.to_string())?;
    let mut game = runtime.load(&wasm).map_err(|error| error.to_string())?;
    let result = game.run_command();
    // (what it wrote before a trap is kept, to show where it stopped)
    let copied = std::io::stdout()
        .write_all(game.output(1))
        .and_then(|()| std::io::stdout().flush())
        .and_then(|()| std::io::stderr().write_all(game.output(2)));
    copied.map_err(|error| format!("writing the output: {error}"))?;
    result.map_err(|error| error.to_string())
}

fn main() -> ExitCode {
    let mut arguments = std::env::args_os().skip(1);
    let (Some(module), None) = (arguments.next(), arguments.next()) else {
        eprintln!("usage: opence-wasm module.wasm");
        return ExitCode::from(2);
    };
    match run(&PathBuf::from(module)) {
        // (an exit code is a byte, as a process's is)
        Ok(code) => ExitCode::from(code.to_le_bytes()[0]),
        Err(error) => {
            eprintln!("opence-wasm: {error}");
            ExitCode::from(70)
        }
    }
}
