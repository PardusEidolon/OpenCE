//! The census as a whole: the original, the engine's citations and what
//! the original's main loop reaches, summed up (`opence census`), checked
//! (`--check`) or written to the docs (`--write`).

use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write as _;
use std::path::Path;

use crate::citations::{self, Citations};
use crate::classes::{lacking, Class, Engine, From, Manifest, Traced};
use crate::original::Original;
use crate::{docs, layout, read_text, traced, Error};

/// The classes.
pub const MANIFEST: &str = "census/census.toml";

/// The functions the reference traces compile (`--write` writes it).
pub const TRACED: &str = "census/traced.tsv";

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
    /// The classes.
    pub manifest: Manifest,
    /// The traced functions.
    pub traced: Traced,
    /// Every function, classed.
    pub functions: Vec<Function>,
    /// What does not meet the requirements, a line each.
    problems: Vec<String>,
}

/// One function of the original, classed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Function {
    /// Its name.
    pub name: String,
    /// Its file of `source/`.
    pub file: String,
    /// Its size in bytes (0 when inlined).
    pub size: u32,
    /// Its class, if it has one.
    pub class: Option<Class>,
    /// Where the class comes from.
    pub from: From,
    /// The places that cite it.
    pub cited: Vec<String>,
    /// The script whose trace compiles it.
    pub traced_by: Option<String>,
    /// Whether [`MAIN`] reaches it.
    pub reached: bool,
    /// The Replaced caller it is reached only through, when it is Replaced
    /// through it.
    pub through_caller: Option<String>,
    /// The evidence it owes.
    pub owed: Option<String>,
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
        let engine = root.join("engine");
        let manifest = match engine.join(MANIFEST) {
            path if path.is_file() => Manifest::parse(MANIFEST, &read_text(&path)?)?,
            _ => Manifest::default(),
        };
        let traced = match engine.join(TRACED) {
            path if path.is_file() => read_traced(&read_text(&path)?),
            _ => Traced::new(),
        };
        let mut census = Self {
            original,
            citations,
            reached,
            manifest,
            traced,
            functions: Vec::new(),
            problems: Vec::new(),
        };
        census.classify(&Engine::read(&engine)?);
        Ok(census)
    }

    /// Classes every function, and finds the problems.
    fn classify(&mut self, engine: &Engine) {
        let none = Vec::new();
        for function in &self.original.functions {
            let (entry, mut from) = self.manifest.lookup(&function.name, &function.file);
            let traced_by = self.traced.get(&function.name).cloned();
            // (a trace makes a function Ported, unless its own entry says
            // otherwise)
            let class = match (from.clone(), &traced_by) {
                (From::Entry, _) | (_, None) => entry.and_then(|e| e.class),
                (_, Some(_)) => {
                    from = From::Trace;
                    Some(Class::Ported)
                }
            };
            self.functions.push(Function {
                name: function.name.clone(),
                file: function.file.clone(),
                size: function.size,
                class,
                from,
                cited: self.citations.get(&function.name).unwrap_or(&none).clone(),
                traced_by,
                reached: self.reached.contains(&function.name),
                through_caller: None,
                owed: entry.and_then(|e| e.owed.clone()),
            });
        }
        self.replace_through_callers();
        self.find_problems(engine);
        let scripts: BTreeSet<&String> = self.traced.values().collect();
        for script in scripts {
            let file = format!("{}.txt", script.strip_prefix("engine_").unwrap_or(script));
            let found = std::fs::read_dir(engine.root.join("crates"))
                .into_iter()
                .flatten()
                .flatten()
                .any(|folder| folder.path().join("tests").join(&file).is_file());
            if !found {
                self.problems.push(format!(
                    "{script}'s trace ({file}) is not in any crate's tests"
                ));
            }
        }
    }

    /// Classes as Replaced, through its caller, each function that would
    /// be Missing by a rule and that only Replaced code reaches.
    fn replace_through_callers(&mut self) {
        let classes: BTreeMap<&str, Option<Class>> = self
            .functions
            .iter()
            .map(|f| (f.name.as_str(), f.class))
            .collect();
        let live = self.original.graph.reach(&[MAIN], |name| {
            classes.get(name).copied().flatten() != Some(Class::Replaced)
        });
        let mut callers: BTreeMap<&str, &str> = BTreeMap::new();
        for (name, node) in &self.original.graph.nodes {
            let dead = classes.get(name.as_str()).copied().flatten() == Some(Class::Replaced)
                || !live.contains(name);
            if dead && self.reached.contains(name) {
                for other in &node.refers_to {
                    callers.entry(other.as_str()).or_insert(name.as_str());
                }
            }
        }
        let mut replaced = Vec::new();
        for (index, function) in self.functions.iter().enumerate() {
            let kept = match function.from {
                From::Rule(index) => {
                    self.manifest
                        .rule
                        .get(index)
                        .and_then(|rule| rule.through_callers)
                        == Some(false)
                }
                _ => true,
            };
            if function.class == Some(Class::Missing)
                && !kept
                && function.reached
                && !live.contains(&function.name)
            {
                if let Some(caller) = callers.get(function.name.as_str()) {
                    replaced.push((index, (*caller).to_owned()));
                }
            }
        }
        for (index, caller) in replaced {
            if let Some(function) = self.functions.get_mut(index) {
                function.class = Some(Class::Replaced);
                function.through_caller = Some(caller);
            }
        }
    }

    /// What does not meet the requirements. A rule's problem is told once,
    /// for the first function it classes, with how many more share it.
    fn find_problems(&mut self, engine: &Engine) {
        let mut found: Vec<(String, Option<usize>, usize)> = Vec::new();
        let mut by_rule: BTreeMap<(usize, String), usize> = BTreeMap::new();
        let known: BTreeSet<&str> = self.functions.iter().map(|f| f.name.as_str()).collect();
        for name in self.manifest.function.keys() {
            if !known.contains(name.as_str()) {
                found.push((format!("{name} is not a function of the original"), None, 0));
            }
        }
        for function in &self.functions {
            let name = &function.name;
            let rule = match function.from {
                From::Rule(index) => Some(index),
                _ => None,
            };
            let mut problems = Vec::new();
            match function.class {
                None => problems.push(format!("{name} has no class")),
                Some(Class::Missing) => {
                    if let Some(place) = function.cited.first() {
                        problems.push(format!("{name} is cited ({place}) but Missing"));
                    }
                }
                Some(_) if function.through_caller.is_some() => {}
                Some(class) => {
                    let default = crate::classes::Entry::default();
                    let entry = self
                        .manifest
                        .lookup(name, &function.file)
                        .0
                        .filter(|_| function.from != From::Trace)
                        .unwrap_or(&default);
                    let lacks = lacking(name, class, entry, &self.traced, &function.cited, engine);
                    problems.extend(lacks.wrong.iter().map(|why| format!("{name}: {why}")));
                    match (&function.owed, lacks.owed.is_empty()) {
                        (Some(_), true) => {
                            problems.push(format!("{name} owes nothing: its owed is stale"));
                        }
                        (None, false) => {
                            problems.extend(lacks.owed.iter().map(|why| format!("{name}: {why}")));
                        }
                        _ => {}
                    }
                }
            }
            for problem in problems {
                match rule {
                    Some(index) => {
                        let shared = problem.replacen(name.as_str(), "", 1);
                        match by_rule.get(&(index, shared.clone())) {
                            Some(&at) => {
                                if let Some(entry) = found.get_mut(at) {
                                    entry.2 += 1;
                                }
                            }
                            None => {
                                by_rule.insert((index, shared), found.len());
                                found.push((problem, Some(index), 0));
                            }
                        }
                    }
                    None => found.push((problem, None, 0)),
                }
            }
        }
        self.problems = found
            .into_iter()
            .map(|(problem, rule, more)| match (rule, more) {
                (Some(index), more) if more > 0 => {
                    format!("{problem} (and {more} more of rule {})", index + 1)
                }
                _ => problem,
            })
            .collect();
    }

    /// What does not meet the requirements, a line each.
    pub fn problems(&self) -> Vec<String> {
        self.problems.clone()
    }

    /// The classed function named `name`.
    pub fn function(&self, name: &str) -> Option<&Function> {
        self.functions.iter().find(|f| f.name == name)
    }

    /// The functions that owe evidence.
    pub fn owed(&self) -> impl Iterator<Item = &Function> {
        self.functions.iter().filter(|f| f.owed.is_some())
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
        let classes: Vec<String> = Class::ALL
            .iter()
            .map(|class| {
                let (count, bytes) = self
                    .functions
                    .iter()
                    .filter(|f| f.class == Some(*class))
                    .fold((0, 0_u64), |(count, bytes), f| {
                        (count + 1, bytes + u64::from(f.size))
                    });
                format!("{count} {} ({bytes} bytes)", class.name())
            })
            .collect();
        let _ = writeln!(
            text,
            "the classes: {}; {} owe evidence",
            classes.join(", "),
            self.owed().count()
        );
        text
    }
}

/// `opence census` on the repository at `root`: the summary, after the
/// check or the writing `mode` asks for.
pub fn census(root: &Path, mode: Mode) -> Result<String, Error> {
    let census = Census::read(root)?;
    match mode {
        Mode::Check => {
            let mut problems = census.problems();
            problems.extend(docs::stale(root, &census));
            if problems.is_empty() {
                Ok(census.summary())
            } else {
                Err(Error::Problems(problems))
            }
        }
        Mode::Write => {
            let path = root.join("engine").join(TRACED);
            if let Some(folder) = path.parent() {
                std::fs::create_dir_all(folder).map_err(|source| Error::Io {
                    path: folder.to_owned(),
                    source,
                })?;
            }
            std::fs::write(&path, traced::write_traced(&traced::generate(root)?))
                .map_err(|source| Error::Io { path, source })?;
            let path = root.join("engine").join(layout::LAYOUT);
            std::fs::write(&path, layout::write_layout(&layout::generate(root)?))
                .map_err(|source| Error::Io { path, source })?;
            let census = Census::read(root)?;
            docs::write(root, &census)?;
            Ok(census.summary())
        }
        Mode::Summary => Ok(census.summary()),
    }
}

/// What `opence census --check` finds beyond the crate's own check: a
/// `traced.tsv` that is not what the reference scripts compile, the
/// scripts whose trace is stale (both need Python and clang), and a
/// `tag_layout.tsv` that is not clang's layout of the tag structures.
pub fn check_traces(root: &Path) -> Result<Vec<String>, Error> {
    let mut found = Vec::new();
    let written = read_text(&root.join("engine").join(TRACED)).unwrap_or_default();
    if written != traced::write_traced(&traced::generate(root)?) {
        found.push(format!(
            "engine/{TRACED} is stale: run opence census --write"
        ));
    }
    found.extend(traced::stale(root)?);
    let written = read_text(&root.join("engine").join(layout::LAYOUT)).unwrap_or_default();
    if written != layout::write_layout(&layout::generate(root)?) {
        found.push(format!(
            "engine/{} is stale: run opence census --write",
            layout::LAYOUT
        ));
    }
    Ok(found)
}

/// `traced.tsv`'s functions: a function and its script a line.
pub fn read_traced(text: &str) -> Traced {
    text.lines()
        .filter_map(|line| line.split_once('\t'))
        .map(|(name, script)| (name.to_owned(), script.to_owned()))
        .collect()
}
