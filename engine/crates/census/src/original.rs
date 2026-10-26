//! The original as the census reads it: its functions (from the symbol
//! headers, and the inlined ones from the C) and its call graph.

use std::collections::BTreeSet;
use std::path::Path;

use crate::c::{definitions, Kind};
use crate::graph::Graph;
use crate::symbols::{self, Function};
use crate::{read_text, walk, Error};

/// The original.
#[derive(Debug, Clone)]
pub struct Original {
    /// Every function: the symbol headers' code, in address order, then
    /// the functions the C defines that have no symbol of their own
    /// (inlined by the compiler, with an address and a size of 0), by file.
    pub functions: Vec<Function>,
    /// The functions the old hosted port added to `source/`, which are not
    /// the original's (see [`crate::c::Definition::port`]).
    pub added: Vec<Function>,
    /// The call graph of its `.c` and `.h` files.
    pub graph: Graph,
}

impl Original {
    /// The original at `source`.
    pub fn read(source: &Path) -> Result<Self, Error> {
        let mut functions = symbols::functions(source)?;
        let mut files = Vec::new();
        for path in walk(source, &["c", "h"])? {
            let file = path
                .strip_prefix(source)
                .map_err(|_| Error::Outside(path.clone()))?
                .to_string_lossy()
                .replace('\\', "/");
            files.push((file, definitions(&read_text(&path)?)));
        }
        let named: BTreeSet<String> = functions.iter().map(|f| f.name.clone()).collect();
        let mut seen = BTreeSet::new();
        let mut added = Vec::new();
        for (file, found) in &files {
            for definition in found {
                if definition.kind == Kind::Function
                    && !named.contains(&definition.name)
                    && seen.insert(definition.name.clone())
                {
                    let function = Function {
                        address: 0,
                        size: 0,
                        name: definition.name.clone(),
                        file: file.clone(),
                    };
                    if definition.port {
                        added.push(function);
                    } else {
                        functions.push(function);
                    }
                }
            }
        }
        Ok(Self {
            functions,
            added,
            graph: Graph::new(&files),
        })
    }

    /// The names of its functions.
    pub fn names(&self) -> BTreeSet<String> {
        self.functions.iter().map(|f| f.name.clone()).collect()
    }
}
