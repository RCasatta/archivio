use regex::Regex;
use std::collections::HashSet;
use std::convert::TryFrom;
use std::fs::DirEntry;
use std::iter::FromIterator;
use std::path::PathBuf;
use std::{env, fs};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

/// A Element contains file name and the set of its tags
#[derive(Debug, Clone)]
struct Element {
    /// File name
    pub name: String,

    /// Tags according to the naming convention
    tags: HashSet<String>,
}

/// Elements is a list of Element
#[derive(Debug, Clone)]
struct Elements(pub Vec<Element>);

impl Elements {
    /// Create Elements struct by listing "Files" directory
    fn new() -> Result<Self> {
        let files_dir = Elements::files_base_dir();
        if !files_dir.exists() || files_dir.is_file() {
            return Err("Must be launched in a dir with `Files` subdirectory".into());
        }
        let data: Result<Vec<Element>> = fs::read_dir(files_dir)?
            .filter_map(|e| e.ok())
            .filter(|e| {
                // exclude hidden files
                e.file_name()
                    .to_str()
                    .map(|f| !f.starts_with("."))
                    .unwrap_or(true)
            })
            .map(Element::try_from)
            .collect();
        let elements = Elements(data?);
        let tags_base_dir = Elements::tags_base_dir();
        if !tags_base_dir.exists() {
            fs::create_dir(tags_base_dir)?;
        }
        Ok(elements)
    }

    /// return all unique tags in self
    fn tags(&self) -> HashSet<String> {
        self.0.iter().map(|e| &e.tags).flatten().cloned().collect()
    }

    /// return another Elements struct filtering self by keeping elements that have all the tags
    /// in `filter` and removes these tags from remaining elements
    fn filter(&self, filter: &[&str]) -> Elements {
        if filter.is_empty() {
            self.clone()
        } else {
            let filter: HashSet<String> =
                HashSet::from_iter(filter.iter().cloned().map(String::from));
            let mut elements = vec![];
            for el in self.0.iter() {
                if el.tags.intersection(&filter).count() == filter.len() {
                    let mut new_el = el.clone();
                    new_el.tags = new_el.tags.difference(&filter).cloned().collect();
                    elements.push(new_el);
                }
            }
            Elements(elements)
        }
    }

    fn tags_base_dir() -> PathBuf {
        let mut tags_dir = env::current_dir().expect("cannot get current_dir");
        tags_dir.push("Tags");
        tags_dir
    }

    fn files_base_dir() -> PathBuf {
        let mut files_dir = env::current_dir().expect("cannot get current_dir");
        files_dir.push("Files");
        files_dir
    }

    /// returns all file names in self
    fn names(&self) -> HashSet<String> {
        self.0.iter().map(|e| String::from(&e.name)).collect()
    }

    /// Creates a directory tree of tags and symbolic reference to files under "Tags" directory
    fn creates_dirs_and_refs(&self, path: &[&str]) -> Result<()> {
        let new_elements = self.filter(&path);
        let first_level = path.is_empty();
        for element in new_elements.0.iter() {
            // not showing files in the first level (would be all)
            if !first_level {
                let mut src = Elements::files_base_dir();
                src.push(&element.name);
                let mut dst = Elements::tags_base_dir();
                dst.extend(path);
                dst.push(&element.name);
                if !dst.exists() {
                    std::os::unix::fs::symlink(src, dst)?;
                }
            }
        }
        // we are not going deeper than 3 levels
        if path.len() > 2 {
            return Ok(());
        } else {
            for tag in new_elements.tags() {
                // we are not showing folders if they are not filtering more than current view
                if first_level || new_elements.names() != new_elements.filter(&[&tag]).names() {
                    let mut new_dir = Elements::tags_base_dir();
                    new_dir.extend(path);
                    new_dir.push(&tag);
                    if !new_dir.exists() {
                        fs::create_dir(new_dir)?;
                    }
                    let mut new_path = path.to_vec();
                    new_path.push(tag.as_str());
                    self.creates_dirs_and_refs(&new_path)?;
                }
            }
        }
        Ok(())
    }
}

impl TryFrom<DirEntry> for Element {
    type Error = Box<dyn std::error::Error>;

    fn try_from(value: DirEntry) -> std::result::Result<Self, Self::Error> {
        let re = Regex::new(r"\d{4}-\d{2}-\d{2}[_[a-zA-Z0-9\-]*]*.[a-z]*").unwrap();
        let name = value
            .file_name()
            .to_str()
            .expect("Name is None")
            .to_string();
        if !re.is_match(&name) {
            return Err(format!(
                "File name {:?} is invalid must be in format YYYY-MM-DD[_Tag]+.ext, exiting",
                &name
            )
            .into());
        }
        let stem = value
            .path()
            .file_stem()
            .expect("Stem is None")
            .to_str()
            .expect("Stem is None")
            .to_string();
        let mut tags: HashSet<String> = stem.split('_').skip(1).map(String::from).collect();
        tags.insert(String::from(&stem[..4]));
        Ok(Element { name, tags })
    }
}

fn main() -> Result<()> {
    let elements = Elements::new()?;
    elements.creates_dirs_and_refs(&[])
}
