# The compilers, tools and libraries the builds and the shell share
{ inputs, cell }:
let
  pkgs = cell.pkgs;
  # the repository as a path. std copies each cell into the store alone,
  # so a path such as ../.. would leave the repository; and a path, not
  # "${inputs.self}/...", copies only the file named (a Cargo.lock's
  # vendored crates then build again only when it changes)
  root = /. + builtins.unsafeDiscardStringContext inputs.self.outPath;
  # clang 22: the guest's compiler (its arm64_32 target), and the
  # version the profiles in pgo/ were recorded with
  llvm = pkgs.llvmPackages_22;
  # wasm32-wasip1's C library, for the reference harnesses, and the same
  # library built for wasm32-wasip1-threads (atomics, a shared memory and
  # wasi-libc's pthreads), for game.wasm
  wasi = pkgs.pkgsCross.wasi32;
  wasilibcThreads = wasi.wasilibc.overrideAttrs (old: {
    pname = "wasilibc-threads";
    cmakeFlags = old.cmakeFlags ++ [ "-DTARGET_TRIPLE=wasm32-wasip1-threads" ];
  });
  rust = pkgs.rust-bin.fromRustupToolchainFile (root + /rust-toolchain.toml);
  python = pkgs.python3.withPackages (ps: [
    ps.pytest
    ps.pyflakes
  ]);
in
{
  inherit
    root
    llvm
    rust
    python
    ;

  version = inputs.self.shortRev or inputs.self.dirtyShortRev or "dirty";

  # what tools/macos_build.py takes from the environment. The guest's
  # compiler is the unwrapped clang: the wrapper's macOS flags would
  # reach the arm64_32 guest's compiles.
  buildEnvironment = {
    HALO_MACOS_GUEST_CC = "${llvm.clang-unwrapped}/bin/clang";
    HALO_MACOS_LLD = "${llvm.lld}/bin/ld.lld";
    HALO_MACOS_AR = "${llvm.llvm}/bin/llvm-ar";
    # (GLES2, GLES3 and KHR: the guest's OpenGL ES declarations)
    HALO_MACOS_GLES_INCLUDE = "${pkgs.libglvnd.dev}/include";
    # (the SDL3 the host links and whose headers the guest compiles with:
    # 3.4.16, as tools/android_build.py's SDL_TAG)
    HALO_MACOS_SDL_INCLUDE = "${pkgs.sdl3.dev}/include";
    HALO_MACOS_SDL_LIB = "${pkgs.sdl3}/lib";
    HALO_MACOS_CODESIGN = "${pkgs.darwin.sigtool}/bin/codesign";
    # game.wasm (tools/engine_wasm.py): the unwrapped clang, for its wasm32
    # target, wasi-libc for wasm32-wasip1-threads (the game) and for
    # wasm32-wasip1 (the reference harnesses)
    HALO_WASM_CC = "${llvm.clang-unwrapped}/bin/clang";
    HALO_WASI_LIBC_THREADS = "${wasilibcThreads}";
    HALO_WASI_LIBC_THREADS_DEV = "${wasilibcThreads.dev}";
    HALO_WASI_LIBC = "${wasi.wasilibc}";
    HALO_WASI_LIBC_DEV = "${wasi.wasilibc.dev}";
    # (compiler-rt's builtins for wasm32, which the unwrapped clang's
    # resource directory does not have)
    HALO_WASM_BUILTINS = "${wasi.llvmPackages_22.compiler-rt}/lib/wasip1/libclang_rt.builtins-wasm32.a";
  };

  tools = [
    python
    pkgs.ninja
    llvm.lld
    llvm.llvm
    rust
    pkgs.cargo-llvm-cov
    pkgs.cargo-nextest
    pkgs.cargo-mutants
    pkgs.cargo-insta
    pkgs.rust-cbindgen
    pkgs.cargo-deny
    pkgs.darwin.sigtool
    # (`nom build`, `nom develop`: nix's builds, as a tree)
    pkgs.nix-output-monitor
  ];
  libraries = [
    pkgs.sdl3
    pkgs.apple-sdk_26
  ];

  # the guest's C library, where tools/macos_build.py would download it
  # (the sandbox has no network)
  musl = pkgs.fetchurl {
    url = "https://musl.libc.org/releases/musl-1.2.5.tar.gz";
    hash = "sha256-qaEYu+hNh2TaDqDSizqz+uhHf8fkCF2QECuFlvx8deQ=";
  };

  # crane with the repository's Rust (the engine's packages and checks)
  craneLib = (inputs.crane.mkLib pkgs).overrideToolchain (_: rust);
}
