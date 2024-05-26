// A `FileDepResolver` is a trait that defines a strategy for resolving all
// necessary file to run a compilation task (e.g., from a `.cm` file).

trait FileDepResolver {
    fn get_files_to_load(&self) -> Vec<String>;
}

// Simply loads a list of file paths to compile. Mainly used for testing or
// manual compilation (without a `.cm` dep file) through the CLI.
struct FileListDepResolver {
    paths: Vec<String>
}

impl FileListDepResolver {
    fn new(paths: Vec<String>) -> FileListDepResolver {
        FileListDepResolver { paths }
    }
}

impl FileDepResolver for FileListDepResolver {
    fn get_files_to_load(&self) -> Vec<String> {
        self.paths.clone()
    }
}
