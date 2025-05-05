use crate::types::{self, InputFile};
use std::{
    any::Any,
    fs,
    iter::{FilterMap, Map},
    os::unix::process,
    path::{Path, PathBuf},
    string,
};
use toml::{ser, Table};

pub struct Project {
    pub filegroups: Vec<FileGroup>,
    //pub settings: ProjectSettings,
    pub context: ProjectContext,
}

pub struct FileGroup {
    pub name: String,
    pub files: Vec<InputFile>,
    pub pre_insert: PathBuf,
    pub post_insert: PathBuf,
    pub process: bool,
}

// pub struct ProjectSettings {

// }

pub struct ProjectContext {
    pub input_folder: PathBuf,
    pub output_folder: PathBuf,
    pub global_pre_insert: PathBuf,
    pub global_post_insert: PathBuf,

    pub filemap: Vec<PathBuf>, // mapped to index

                               //variables later
}

macro_rules! get_table_bool_or_default {
    ($table:ident, $key:expr, $default:expr) => {
        $table
            .get($key)
            .unwrap_or(&toml::Value::try_from($default).unwrap())
            .as_bool()
            .unwrap_or($default)
    };
}

macro_rules! get_table_string_or_default {
    ($table:ident, $key:expr, $default:expr) => {
        // $table
        //     .get($key)
        //     .unwrap_or(&toml::Value::try_from($default).unwrap())
        //     .as_str()
        //     .unwrap_or($default)
        if $table.contains_key($key) {
            $table.get($key).unwrap().as_str().unwrap()
        } else {
            $default
        }
    };
}

pub fn parse_project(tomlpath: &Path) -> Project {
    let tomlfile = fs::read_to_string(tomlpath).expect("Project file unreadable or missing.");

    let mut project: Project = Project {
        filegroups: Vec::new(),
        context: ProjectContext {
            input_folder: PathBuf::new(),
            output_folder: PathBuf::new(),
            global_pre_insert: PathBuf::new(),
            global_post_insert: PathBuf::new(),
            filemap: Vec::new(),
        },
    };
    let config = tomlfile
        .parse::<Table>()
        .expect("Project file not in propper toml format");
    let settings_section = config["settings"]
        .as_table()
        .expect("Project file missing [settings] section");
    let filegroups_section = config["fileGroups"]
        .as_table()
        .expect("Project file contains no file groups ");

    let project_root = tomlpath
        .parent()
        .expect("Project file unreadable or missing.");

    project.context.input_folder = PathBuf::from(get_table_string_or_default!(
        settings_section,
        "inputFolder",
        "skid"
    ));

    project.context.output_folder = PathBuf::from(get_table_string_or_default!(
        settings_section,
        "outputFolder",
        "content"
    ));

    project.context.global_pre_insert = project_root.join(get_table_string_or_default!(
        settings_section,
        "preInsertGlobal",
        ""
    ));
    project.context.global_post_insert = project_root.join(get_table_string_or_default!(
        settings_section,
        "postInsertGlobal",
        ""
    ));

    for (k, v) in filegroups_section {
        if !v.is_table() {
            continue;
        }
        let filegroup_def: &toml::map::Map<String, toml::Value> = v.as_table().unwrap();
        let name = k.clone();
        let pre_insert = get_table_string_or_default!(filegroup_def, "preInsert", "");
        let post_insert = get_table_string_or_default!(filegroup_def, "postInsert", "");
        let process = get_table_bool_or_default!(filegroup_def, "process", false);

        let recurse_find = get_table_bool_or_default!(filegroup_def, "recursiveFind", false);

        let dir = get_table_string_or_default!(filegroup_def, "folder", "");

        let mut group = FileGroup {
            files: Vec::new(),
            name: k.clone(),
            pre_insert: pre_insert.into(),
            post_insert: post_insert.into(),
            process,
        };

        if filegroup_def.contains_key("files") {
            let file_array = filegroup_def["files"].as_array().unwrap_or_else(|| {
                panic!("'files' section of fileGroup.{} needs to be an array", k)
            });
            for file in file_array {
                let filename = file.as_str().unwrap_or_else(|| {
                    panic!(
                        "'files' section of fileGroup.{} needs to only contain strings",
                        k
                    )
                });
                let mut new_file = crate::types::InputFile::new();
                new_file.file_input = project.context.input_folder.clone();
                new_file.file_input.push(filename);

                new_file.file_htmlout = project.context.output_folder.clone();
                new_file.file_htmlout.push(filename);
                new_file.file_htmlout.set_extension("html");

                new_file.file_skidout = new_file.file_htmlout.clone();
                new_file.file_skidout.set_extension("sko");

                group.files.push(new_file);
            }
        }

        project.filegroups.push(group);
    }

    return project;
}

pub trait FileIndexing {
    fn index_of_file(&mut self, f: &PathBuf) -> usize;
    fn file_for_index(&self, i: usize) -> Option<&PathBuf>;
}

impl FileIndexing for ProjectContext {
    fn index_of_file(&mut self, f: &PathBuf) -> usize {
        let mut cannonical = f.canonicalize().unwrap();
        let mut index = 0;
        for p in &self.filemap {
            if cannonical == *p {
                return index;
            }
            index = index + 1;
        }
        self.filemap.push(cannonical);
        return self.filemap.len() - 1;
    }

    fn file_for_index(&self, i: usize) -> Option<&PathBuf> {
        if i >= self.filemap.len() {
            return None;
        }
        return Some(&self.filemap[i]);
    }
}
