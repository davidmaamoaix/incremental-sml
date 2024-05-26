use string_interner::symbol::SymbolU32;

// Source map for a single file (treated as a compilation unit).
struct SourceMap {
    path: SymbolU32,
    source: String,
    line_offsets: Vec<u32>
}

impl SourceMap {

    fn new(path: SymbolU32, source: String) -> SourceMap {
        SourceMap {
            path,
            source,
            line_offsets: Vec::new()
        }
    }
}
