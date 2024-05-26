use string_interner::DefaultStringInterner;

struct LoaderSession {
    intern: DefaultStringInterner
}

impl LoaderSession {

    pub(crate) fn new() -> LoaderSession {
        LoaderSession {
            intern: DefaultStringInterner::default()
        }
    }
}
