//! The part of WASI the runtime supplies itself: the game's and the
//! reference harnesses' output (`fd_write`), read through checked ranges.

use opence_runtime::{Error, Runtime};

fn module(text: &str) -> Vec<u8> {
    wat::parse_str(text).unwrap_or_default()
}

/// fd_write(fd, iovs, count, written): the iovecs at 0, the bytes at 100
const WRITER: &str = r#"
(module
  (import "wasi_snapshot_preview1" "fd_write" (func $write (param i32 i32 i32 i32) (result i32)))
  (memory (export "memory") 1)
  (data (i32.const 0) "\64\00\00\00\06\00\00\00\6a\00\00\00\03\00\00\00")
  (data (i32.const 100) "hello\nbye")
  ;; a vector that reaches past the memory
  (data (i32.const 200) "\fa\ff\00\00\64\00\00\00")
  (func (export "both") (result i32) (call $write (i32.const 1) (i32.const 0) (i32.const 2) (i32.const 50)))
  (func (export "error") (result i32) (call $write (i32.const 2) (i32.const 8) (i32.const 1) (i32.const 50)))
  (func (export "outside") (result i32) (call $write (i32.const 1) (i32.const 200) (i32.const 1) (i32.const 50)))
  (func (export "written_outside") (result i32) (call $write (i32.const 1) (i32.const 0) (i32.const 1) (i32.const 65534))))
"#;

#[test]
fn fd_write_is_kept_per_descriptor_with_the_count_written() -> Result<(), Error> {
    let runtime = Runtime::new(None)?;
    let mut game = runtime.load(&module(WRITER))?;
    assert_eq!(game.call_i32("both")?, 0);
    assert_eq!(game.read(50, 4)?, 9u32.to_le_bytes());
    assert_eq!(game.call_i32("error")?, 0);
    assert_eq!(game.output(1), b"hello\nbye");
    assert_eq!(game.output(2), b"bye");
    assert_eq!(game.output(3), b"");
    assert_eq!(game.call_count("wasi_snapshot_preview1.fd_write"), 2);
    Ok(())
}

#[test]
fn fd_write_outside_the_memory_is_efault_and_writes_nothing() -> Result<(), Error> {
    let runtime = Runtime::new(None)?;
    let mut game = runtime.load(&module(WRITER))?;
    // WASI's errno 21, EFAULT
    assert_eq!(game.call_i32("outside")?, 21);
    assert_eq!(game.call_i32("written_outside")?, 21);
    assert_eq!(game.output(1), b"");
    Ok(())
}

#[test]
fn a_command_s_exit_code_is_its_result() -> Result<(), Error> {
    let runtime = Runtime::new(None)?;
    let mut game = runtime.load(&module(
        r#"(module
          (import "wasi_snapshot_preview1" "proc_exit" (func $exit (param i32)))
          (func (export "_start") (call $exit (i32.const 0))))"#,
    ))?;
    assert_eq!(game.run_command()?, 0);
    let mut returns = runtime.load(&module(r#"(module (func (export "_start")))"#))?;
    assert_eq!(returns.run_command()?, 0);
    let mut fails = runtime.load(&module(
        r#"(module
          (import "wasi_snapshot_preview1" "proc_exit" (func $exit (param i32)))
          (func (export "_start") (call $exit (i32.const 4))))"#,
    ))?;
    assert_eq!(fails.run_command()?, 4);
    Ok(())
}
