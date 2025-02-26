mod cli;
mod log;
mod specs;
mod tmpls;

use log::LogLevel;
use serde_yml;
use std::fs::{self, create_dir_all, write, OpenOptions};
use std::io::Write;
use std::path::Path;
use tmpls::RenderedConfig;

fn main() {
    let args: cli::Args = cli::parse_args();
    let dbg: LogLevel = args.loglevel;

    dbug!(dbg, "{}", &args);

    let specifications: Vec<specs::Specification> =
        specs::compile(&args.devices, &args.env.spec_path, dbg);

    for spec in specifications {
        let result = tmpls::process_templates(&spec, dbg).ok().unwrap();
        verb!(
            dbg,
            "Compiled Spec:\n{}",
            serde_json::to_string_pretty(&spec.compiled["data"]).unwrap()
        );
        output_rendered_configs(result, dbg)
    }
}

fn output_rendered_configs(rendered_config: RenderedConfig, dbg: LogLevel) {
    info!(dbg, "Writing Output");
    let path: String = format!("out/{}", rendered_config.hostname);
    if Path::new(&path).exists() {
        fs::remove_dir_all(&path).ok();
    }
    create_dir_all(&path).ok();
    let rendered_config_path = format!("{}/all.conf", path);
    for rendered_template in rendered_config.configs {
        let outpath = format!("{}/{}.tmpl", path, rendered_template.0);
        verb!(dbg, " | {}", &outpath);
        write(&outpath, &rendered_template.1).ok();
        append_to_file(&rendered_config_path, rendered_template.1).ok();
    }
    let outpath = format!("{}/compiled.spec", path);
    let spec: String = match serde_yml::to_string(&rendered_config.spec) {
        Ok(yaml) => yaml,
        Err(e) => {
            eprintln!("Failed to convert to YAML: {}", e);
            String::new()
        }
    };
    verb!(dbg, " | {}", &outpath);
    write(&outpath, spec).ok();
    info!(dbg, " | {} ", &rendered_config_path);
}

fn append_to_file(path: &str, content: String) -> std::io::Result<()> {
    let mut file = OpenOptions::new().create(true).append(true).open(path)?;

    write!(file, "{}", content)?;
    Ok(())
}
