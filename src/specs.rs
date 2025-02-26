use crate::{dbug, info, verb, LogLevel};
use regex::Regex;
use serde_json::{json, Map, Value};
use serde_yml;
use std::path::{Path, PathBuf};
use std::str::FromStr;
use std::{env, fmt, fs};
use walkdir::WalkDir;

#[derive(Debug)]
pub struct Specification {
    pub partitional: String,
    pub regional: String,
    pub common: String,
    pub zonal: String,
    pub device: String,
    pub compiled: Value,
}

impl Specification {
    pub fn build(
        partitional: &str,
        regional: &str,
        common: &str,
        zonal: &str,
        device: &str,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        // Read all YAML files
        let yaml_contents: Vec<String> = [partitional, regional, common, zonal, device]
            .iter()
            .filter_map(|path| fs::read_to_string(path).ok())
            .collect();

        // Convert each YAML to JSON Value and collect them
        let json_values: Vec<Value> = yaml_contents
            .iter()
            .map(|content| serde_yml::from_str(content))
            .collect::<Result<Vec<Value>, _>>()?;

        // Merge all objects under a single key
        let mut merged_map = Map::new();
        for value in json_values {
            if let Some(obj) = value.as_object() {
                for (_key, value) in obj {
                    if let Some(inner_obj) = value.as_object() {
                        for (k, v) in inner_obj {
                            merged_map.insert(k.clone(), v.clone());
                        }
                    }
                }
            }
        }

        // Create the final merged object
        let compiled = json!({"data": merged_map});

        Ok(Self {
            partitional: String::from(partitional),
            regional: String::from(regional),
            common: String::from(common),
            zonal: String::from(zonal),
            device: String::from(device),
            compiled,
        })
    }

    pub fn get_layer(&self) -> String {
        PathBuf::from(&self.device)
            .parent()
            .unwrap()
            .file_name()
            .unwrap()
            .to_string_lossy()
            .chars()
            .filter(|c| !c.is_numeric())
            .collect::<String>()
    }

    pub fn get_hostname(&self) -> String {
        PathBuf::from(&self.device)
            .file_name()
            .unwrap()
            .to_string_lossy()
            .into_owned()
    }
}

impl fmt::Display for Specification {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Specification:\n\
             Partitional: {}\n\
             Regional: {}\n\
             Common: {}\n\
             Zonal: {}\n\
             Device: {}\n\
             Compiled:\n{}",
            self.partitional, self.regional, self.common, self.zonal, self.device, self.compiled
        )
    }
}

pub fn compile(pattern: &Regex, spec_path: &String, dbg: LogLevel) -> Vec<Specification> {
    let spec_list: Vec<String> = filter_specs(pattern, get_specs(spec_path, dbg));
    info!(
        dbg,
        "Matched {} devices against '{}'",
        &spec_list.len(),
        &pattern
    );
    for spec in &spec_list {
        verb!(dbg, " | {}", spec);
    }
    let mut specifications: Vec<Specification> = Vec::new();
    for spec in spec_list {
        let zonal: String = get_zonal(&spec);
        let common: String = get_common(&spec);
        let regional: String = get_regional(&spec);
        let partitional: String = get_partional(&common, &regional);
        specifications.push(
            match Specification::build(&partitional, &regional, &common, &zonal, &spec) {
                Ok(compiled_spec) => {
                    dbug!(
                        dbg,
                        "Compiled Spec for '{}'\n | {}\n | {}\n | {}\n | {}\n | {}",
                        compiled_spec.get_hostname(),
                        compiled_spec.partitional,
                        compiled_spec.regional,
                        compiled_spec.common,
                        compiled_spec.zonal,
                        compiled_spec.device
                    );
                    compiled_spec
                }
                Err(e) => panic!("failed to build Specification: {}", e),
            },
        )
    }
    specifications
}

fn get_partional(common: &str, regional: &str) -> String {
    let re = Regex::new(r"partition: ([^\n\r$]+)").expect("failed");

    [common, regional]
        .iter()
        .find_map(|&path| {
            fs::read_to_string(path).ok().and_then(|contents| {
                re.captures(&contents).and_then(|captures| {
                    captures
                        .get(1)
                        .map(|matched| format!("./spec/common/{}.yaml", matched.as_str().trim()))
                })
            })
        })
        .unwrap_or_else(|| format!("./spec/common/default.yaml"))
}

fn get_regional(path: &str) -> String {
    let dir_name = PathBuf::from(path)
        .parent()
        .unwrap()
        .file_name()
        .unwrap()
        .to_string_lossy()
        .into_owned();

    let new_path = PathBuf::from(path)
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .join("common")
        .join(format!("{}.yaml", &dir_name[0..2]));

    String::from(new_path.to_str().unwrap())
}

fn get_common(spec: &str) -> String {
    let path: PathBuf = PathBuf::from_str(&spec).expect("Path was invalid");
    String::from(path.with_file_name("common.yaml").to_str().unwrap())
}

fn get_zonal(spec: &str) -> String {
    let path: PathBuf = PathBuf::from_str(&spec).expect("Path was invalid");

    String::from(format!(
        "{}.common.yaml",
        path.file_name()
            .and_then(|filename| filename.to_str())
            .and_then(|filename| filename.split_once('-'))
            .map(|(zone, _)| path.with_file_name(zone))
            .unwrap_or_else(|| path.with_file_name("common.yaml"))
            .to_str()
            .unwrap()
            .to_string(),
    ))
}

fn filter_specs(pattern: &Regex, spec_list: Vec<String>) -> Vec<String> {
    spec_list
        .into_iter()
        .filter(|spec| {
            Path::new(spec)
                .file_name()
                .and_then(|name| name.to_str())
                .map_or(false, |name| pattern.is_match(name))
        })
        .collect()
}

fn get_specs(spec_path: &String, dbg: LogLevel) -> Vec<String> {
    let mut result = Vec::new();
    let root = Path::new(&spec_path);
    for entry in WalkDir::new(root).into_iter().filter_map(|e| e.ok()) {
        if let Ok(path) = entry.path().strip_prefix(root) {
            let path_str = path.to_string_lossy().to_string();

            if !path_str.contains("common") && path_str.ends_with("yaml") {
                result.push(format!("{}/{}", spec_path, path_str));
            }
        }
    }
    info!(
        dbg,
        "Skyforge found {} renderable devices in {}",
        result.len(),
        env::current_dir().unwrap().display()
    );
    result
}
