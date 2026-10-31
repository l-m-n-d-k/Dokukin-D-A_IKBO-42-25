//! Виртуальная файловая система в памяти, загружаемая из CSV-файла.
//!
//! Формат CSV: заголовок `path,type,content`, далее по строке на элемент.
//! `path` — полный путь от корня (`/docs/a.txt`), вложенность задаётся
//! разделителем `/`. `type` — `dir` или `file`. `content` — содержимое
//! файла в base64 (для каталогов пусто). Недостающие родительские каталоги
//! создаются автоматически. Файл на диске не изменяется.

use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

use base64::Engine;
use base64::engine::general_purpose::STANDARD;
use sha2::{Digest, Sha256};

use crate::csv;

/// Заголовок CSV-файла VFS.
const HEADER: [&str; 3] = ["path", "type", "content"];

/// Содержимое каталога: имя элемента и сам элемент.
type Children = BTreeMap<String, Node>;

/// Элемент VFS.
// Содержимое элементов читают команды следующих этапов.
#[allow(dead_code)]
#[derive(Debug, PartialEq, Eq)]
pub enum Node {
    /// Каталог.
    Dir(Children),
    /// Файл с двоичным содержимым.
    File(Vec<u8>),
}

/// Загруженная в память виртуальная файловая система.
#[derive(Debug)]
pub struct Vfs {
    /// Имя VFS (имя CSV-файла).
    pub name: String,
    /// Хеш SHA-256 исходных данных в шестнадцатеричном виде.
    pub sha256: String,
    /// Корневой каталог.
    pub root: Children,
}

impl Vfs {
    /// Загружает VFS из CSV-файла, не изменяя его.
    pub fn load(path: &str) -> Result<Vfs, String> {
        let bytes = fs::read(path).map_err(|err| format!("{path}: {err}"))?;
        let name = Path::new(path)
            .file_name()
            .map_or_else(|| path.to_string(), |n| n.to_string_lossy().into());
        Vfs::from_bytes(name, &bytes).map_err(|err| format!("{path}: {err}"))
    }

    /// Строит VFS из байтов CSV-файла.
    pub fn from_bytes(name: String, bytes: &[u8]) -> Result<Vfs, String> {
        let text = std::str::from_utf8(bytes).map_err(|err| err.to_string())?;
        let mut vfs = Vfs {
            name,
            sha256: hex(&Sha256::digest(bytes)),
            root: Children::new(),
        };
        let mut rows = text.lines().enumerate().filter(|(_, l)| !l.is_empty());
        match rows.next() {
            Some((_, header)) if csv::fields(header) == HEADER => {}
            _ => return Err(format!("expected header {}", HEADER.join(","))),
        }
        for (index, line) in rows {
            vfs.add_row(&csv::fields(line))
                .map_err(|err| format!("line {}: {err}", index + 1))?;
        }
        Ok(vfs)
    }

    /// Добавляет в дерево элемент из одной строки CSV.
    fn add_row(&mut self, row: &[String]) -> Result<(), String> {
        let [path, kind, content] = row else {
            return Err(format!("expected {} fields", HEADER.len()));
        };
        match kind.as_str() {
            "dir" => self.insert(path, None),
            "file" => {
                let data = STANDARD
                    .decode(content)
                    .map_err(|err| format!("{path}: bad base64: {err}"))?;
                self.insert(path, Some(data))
            }
            other => Err(format!("{path}: unknown type '{other}'")),
        }
    }

    /// Вставляет каталог (`None`) или файл (`Some`) по полному пути.
    fn insert(&mut self, path: &str, data: Option<Vec<u8>>) -> Result<(), String> {
        let parts: Vec<&str> = path.split('/').filter(|p| !p.is_empty()).collect();
        let Some((last, parents)) = parts.split_last() else {
            return Err(format!("'{path}': empty path"));
        };
        let mut dir = &mut self.root;
        for part in parents {
            let node = dir
                .entry(part.to_string())
                .or_insert_with(|| Node::Dir(Children::new()));
            dir = match node {
                Node::Dir(children) => children,
                Node::File(_) => return Err(format!("{path}: parent is a file")),
            };
        }
        match (dir.get(*last), data) {
            (None, None) => dir.insert(last.to_string(), Node::Dir(Children::new())),
            (None, Some(bytes)) => dir.insert(last.to_string(), Node::File(bytes)),
            (Some(Node::Dir(_)), None) => return Ok(()),
            (Some(_), _) => return Err(format!("{path}: already exists")),
        };
        Ok(())
    }
}

/// Переводит байты в строку из шестнадцатеричных цифр.
fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn load(csv: &str) -> Result<Vfs, String> {
        Vfs::from_bytes("t.csv".to_string(), csv.as_bytes())
    }

    #[test]
    fn nested_paths_create_parents() {
        let vfs = load("path,type,content\n/a/b/c.txt,file,aGk=\n").unwrap();
        let Node::Dir(a) = &vfs.root["a"] else { panic!() };
        let Node::Dir(b) = &a["b"] else { panic!() };
        assert_eq!(b["c.txt"], Node::File(b"hi".to_vec()));
    }

    #[test]
    fn hash_covers_raw_data() {
        let vfs = load("path,type,content\n").unwrap();
        assert_eq!(vfs.sha256, hex(&Sha256::digest(b"path,type,content\n")));
    }

    #[test]
    fn empty_input_without_header_is_error() {
        assert!(load("").is_err());
    }

    #[test]
    fn bad_base64_is_error() {
        assert!(load("path,type,content\n/a,file,***\n").is_err());
    }

    #[test]
    fn unknown_type_is_error() {
        assert!(load("path,type,content\n/a,link,\n").is_err());
    }

    #[test]
    fn duplicate_file_is_error() {
        let csv = "path,type,content\n/a,file,\n/a,file,\n";
        assert!(load(csv).is_err());
    }

    #[test]
    fn file_as_parent_is_error() {
        let csv = "path,type,content\n/a,file,\n/a/b,file,\n";
        assert!(load(csv).is_err());
    }
}
