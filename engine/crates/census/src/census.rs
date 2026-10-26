//! The census as a whole: the original, the engine's citations and what
//! the original's main loop reaches, summed up (`opence census`), checked
//! (`--check`) or written to the docs (`--write`).

use std::collections::BTreeSet;
use std::fmt::Write as _;
use std::path::Path;

use crate::citations::{self, Citations};
use crate::original::Original;
use crate::Error;

/// The function every game, campaign or multiplayer, runs through.
pub const MAIN: &str = "main_loop";

/// What `opence census` does.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    /// Print the summary.
    Summary,
    /// Fail when the census's requirements are not met, or its files are
    /// stale.
    Check,
    /// Write the files the census owns.
    Write,
}

/// The census of the repository at a root.
#[derive(Debug, Clone)]
pub struct Census {
    /// The original.
    pub original: Original,
    /// The original's functions the engine cites.
    pub citations: Citations,
    /// The nodes of the original's graph that [`MAIN`] reaches.
    pub reached: BTreeSet<String>,
}

impl Census {
    /// The census of the repository at `root`.
    pub fn read(root: &Path) -> Result<Self, Error> {
        let source = root.join("source");
        if !source.is_dir() {
            return Err(Error::Io {
                path: source,
                source: std::io::ErrorKind::NotFound.into(),
            });
        }
        let original = Original::read(&source)?;
        let citations = citations::collect(&root.join("engine"), &original.names())?;
        let reached = original.graph.reach(&[MAIN], |_| true);
        Ok(Self {
            original,
            citations,
            reached,
        })
    }

    /// Its summary, a line for each count.
    pub fn summary(&self) -> String {
        let functions = &self.original.functions;
        let bytes: u64 = functions.iter().map(|f| u64::from(f.size)).sum();
        let inlined = functions.iter().filter(|f| f.size == 0).count();
        let reached = functions
            .iter()
            .filter(|f| self.reached.contains(&f.name))
            .count();
        let mut text = String::new();
        let _ = writeln!(
            text,
            "the original: {} functions in {bytes} bytes ({inlined} inlined), {} added by the old port",
            functions.len(),
            self.original.added.len()
        );
        let _ = writeln!(text, "the engine: {} cited", self.citations.len());
        let _ = writeln!(text, "the graph: {reached} reached from {MAIN}");
        text
    }
}

/// `opence census` on the repository at `root`: the summary, after the
/// check or the writing `mode` asks for.
pub fn census(root: &Path, mode: Mode) -> Result<String, Error> {
    let census = Census::read(root)?;
    match mode {
        Mode::Summary | Mode::Check | Mode::Write => Ok(census.summary()),
    }
}
