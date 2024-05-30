use string_interner::symbol::SymbolU32;

// Source map for a single file (treated as a compilation unit).
pub struct SourceMap {
    path: SymbolU32,
    source: String,
    line_offsets: Vec<usize> // i-th element is the offset of the i-th line.
}

impl SourceMap {

    pub fn new(path: SymbolU32, source: String) -> SourceMap {
        let mut line_mapping = vec![0];
        let bytes = source.as_bytes();

        for i in 0..source.len() {
            if bytes[i] == b'\n' {
                line_mapping.push(i + 1);
            }
        }

        SourceMap {
            path,
            source,
            line_offsets: line_mapping
        }
    }

    // `lineno` is 0-based; offset by one during rendering (same with column).
    fn lineno_from_offset(&self, offset: usize) -> usize {
        let mut lineno = 0;
        for i in 0..self.line_offsets.len() {
            if self.line_offsets[i] > offset {
                break;
            }
            lineno = i;
        }
        lineno
    }
}
