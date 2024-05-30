use std::collections::HashMap;
use string_interner::symbol::SymbolU32;
use string_interner::DefaultStringInterner;

use super::source_map::SourceMap;
use super::file_loader::{ FileRepr };

pub struct LoaderSession {
    path_interner: DefaultStringInterner,
    source_maps: HashMap<SymbolU32, SourceMap>
}

impl LoaderSession {

    pub fn new() -> Self {
        LoaderSession {
            path_interner: DefaultStringInterner::default(),
            source_maps: HashMap::new()
        }
    }

    pub fn load(&mut self, deps: Vec<FileRepr>) -> Result<(), String> {
        for dep in deps {
            let path_sym = self.path_interner.get_or_intern(dep.path.as_str());
            let smap = SourceMap::new(path_sym, dep.content);
            self.source_maps.insert(path_sym, smap);
        }

        Ok(())
    }
}
