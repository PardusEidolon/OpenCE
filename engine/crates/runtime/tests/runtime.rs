//! The runtime on small modules written here, and on game.wasm when it has
//! been built (tools/engine_wasm.py; the test skips itself without it).

use std::path::PathBuf;

use opence_runtime::{Error, Runtime, Value};

fn module(text: &str) -> Vec<u8> {
    wat::parse_str(text).unwrap_or_default()
}

/// a cache folder of the test's own, empty
fn cache(name: &str) -> PathBuf {
    let folder = std::env::temp_dir().join(format!("opence_runtime_{name}_{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&folder);
    folder
}

const CALLER: &str = r#"
(module
  (import "env" "halo_platform_tick" (func $tick (param i32 i64) (result i32)))
  (import "env" "platform_log" (func $log (param f32 f64)))
  (memory (export "memory") 1)
  (func (export "run") (result i32)
    (call $log (f32.const 1.5) (f64.const -2.25))
    (drop (call $tick (i32.const 7) (i64.const 9)))
    (call $tick (i32.const -1) (i64.const 0x100000000))))
"#;

#[test]
fn the_stubs_record_each_call_with_its_arguments_and_return_zero() -> Result<(), Error> {
    let runtime = Runtime::new(None)?;
    let mut game = runtime.load(&module(CALLER))?;
    assert_eq!(game.call_i32("run")?, 0);
    let calls: Vec<_> = game
        .calls()
        .iter()
        .map(|call| (game.import_name(call.import), call.args.clone()))
        .collect();
    assert_eq!(
        calls,
        vec![
            (
                "env.platform_log".to_string(),
                vec![
                    Value::F32(1.5f32.to_bits()),
                    Value::F64((-2.25f64).to_bits())
                ]
            ),
            (
                "env.halo_platform_tick".to_string(),
                vec![Value::I32(7), Value::I64(9)]
            ),
            (
                "env.halo_platform_tick".to_string(),
                vec![Value::I32(-1), Value::I64(0x1_0000_0000)]
            ),
        ]
    );
    assert_eq!(game.call_count("env.halo_platform_tick"), 2);
    assert_eq!(game.call_count("env.platform_log"), 1);
    assert_eq!(game.call_count("env.missing"), 0);
    Ok(())
}

#[test]
fn the_record_keeps_a_bounded_number_of_calls_but_counts_them_all() -> Result<(), Error> {
    let runtime = Runtime::new(None)?;
    let mut game = runtime.load(&module(
        r#"(module
          (import "env" "f" (func $f))
          (func (export "run") (local $i i32)
            (loop $again
              (call $f)
              (local.set $i (i32.add (local.get $i) (i32.const 1)))
              (br_if $again (i32.lt_u (local.get $i) (i32.const 5000))))))"#,
    ))?;
    game.set_record_limit(100);
    game.call("run")?;
    assert_eq!(game.calls().len(), 100);
    assert_eq!(game.call_count("env.f"), 5000);
    Ok(())
}

#[test]
fn proc_exit_ends_the_call_with_its_code() -> Result<(), Error> {
    let runtime = Runtime::new(None)?;
    let mut game = runtime.load(&module(
        r#"(module
          (import "wasi_snapshot_preview1" "proc_exit" (func $exit (param i32)))
          (func (export "run") (call $exit (i32.const 3)) (unreachable)))"#,
    ))?;
    match game.call("run") {
        Err(Error::Exit(3)) => Ok(()),
        other => panic!("expected an exit with code 3, got {other:?}"),
    }
}

#[test]
fn a_trap_in_the_game_is_an_error_not_a_panic() -> Result<(), Error> {
    let runtime = Runtime::new(None)?;
    let mut game = runtime.load(&module(r#"(module (func (export "run") (unreachable)))"#))?;
    assert!(matches!(game.call("run"), Err(Error::Trap(_))));
    Ok(())
}

#[test]
fn a_missing_export_and_a_malformed_module_are_errors() -> Result<(), Error> {
    let runtime = Runtime::new(None)?;
    let mut game = runtime.load(&module("(module)"))?;
    assert!(matches!(game.call("run"), Err(Error::NoExport(_))));
    assert!(matches!(
        runtime.load(b"\0asm\x01\0\0\0\x01\x10"),
        Err(Error::Compile(_))
    ));
    Ok(())
}

#[test]
fn imports_other_than_functions_are_refused() -> Result<(), Error> {
    let runtime = Runtime::new(None)?;
    let result = runtime.load(&module(r#"(module (import "env" "memory" (memory 1)))"#));
    assert!(matches!(result, Err(Error::Import(_))));
    Ok(())
}

#[test]
fn the_memory_is_read_and_written_with_bounds_checks() -> Result<(), Error> {
    let runtime = Runtime::new(None)?;
    let mut game = runtime.load(&module(
        r#"(module (memory (export "memory") 1)
             (func (export "run") (i32.store (i32.const 16) (i32.const 0x04030201))))"#,
    ))?;
    game.call("run")?;
    assert_eq!(game.memory_size(), 65536);
    assert_eq!(game.read(16, 4)?, vec![1, 2, 3, 4]);
    game.write(65532, &[9, 9, 9, 9])?;
    assert!(matches!(
        game.read(65533, 4),
        Err(Error::OutOfBounds { .. })
    ));
    assert!(matches!(
        game.write(u64::MAX, &[0]),
        Err(Error::OutOfBounds { .. })
    ));
    Ok(())
}

#[test]
fn the_compiled_module_is_cached_and_used_again() -> Result<(), Error> {
    let folder = cache("cached");
    let wasm = module(CALLER);
    let first = Runtime::new(Some(&folder))?;
    first.load(&wasm)?;
    let files = |folder: &PathBuf| -> usize {
        fn count(path: &std::path::Path) -> usize {
            std::fs::read_dir(path).map_or(0, |entries| {
                entries
                    .flatten()
                    .map(|entry| {
                        if entry.path().is_dir() {
                            count(&entry.path())
                        } else {
                            1
                        }
                    })
                    .sum()
            })
        }
        count(folder)
    };
    assert!(
        files(&folder) > 0,
        "nothing was cached in {}",
        folder.display()
    );
    let second = Runtime::new(Some(&folder))?;
    let mut game = second.load(&wasm)?;
    assert_eq!(game.call_i32("run")?, 0);
    assert!(second.cache_hits() >= 1, "the second load compiled again");
    let _ = std::fs::remove_dir_all(&folder);
    Ok(())
}

/// game.wasm, where tools/engine_wasm.py builds it
fn game_wasm() -> Option<Vec<u8>> {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..");
    std::fs::read(root.join("build/engine/wasm/game.wasm")).ok()
}

#[test]
fn game_wasm_loads_with_a_stub_for_every_import() -> Result<(), Error> {
    let Some(wasm) = game_wasm() else {
        eprintln!("skipped: no build/engine/wasm/game.wasm (python tools/engine_wasm.py)");
        return Ok(());
    };
    let runtime = Runtime::new(None)?;
    let game = runtime.load(&wasm)?;
    // the Xbox's window, 0x80000000 to 0x88000000, is inside the memory
    assert!(game.memory_size() >= 0x8800_0000);
    assert!(game.import_count() > 400);
    for export in ["halo_game_main", "malloc", "free", "_initialize"] {
        assert!(game.has_export(export), "game.wasm has no {export}");
    }
    Ok(())
}
