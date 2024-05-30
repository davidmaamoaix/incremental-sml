use std::fs::File;
use std::io::{ Read, Result };

#[derive(Debug)]
pub struct FileRepr {
    pub path: String,
    pub content: String
}

pub fn load_from_path_list(paths: &Vec<String>) -> Result<Vec<FileRepr>> {
    let mut reprs = Vec::new();

    for path in paths {
        let mut file = File::open(path)?;
        let mut text = String::new();
        file.read_to_string(&mut text)?;

        let repr = FileRepr {
            path: path.clone(),
            content: text
        };

        reprs.push(repr);
    }

    Ok(reprs)
}

pub fn load_from_cm(cm_path: &str) -> Result<Vec<FileRepr>> {
    todo!();
}
