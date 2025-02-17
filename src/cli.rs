use crate::log::LogLevel;
use clap::{Arg, ArgAction, ArgGroup, Command};
use regex::Regex;
use std::fmt;

#[derive(Debug)]
pub struct Args {
    pub devices: Regex,
    pub loglevel: LogLevel,
    pub env: EnvVars,
}

impl fmt::Display for Args {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "devices: {}, loglevel: {}, env: {}",
            self.devices, self.loglevel, self.env
        )
    }
}

#[derive(Debug)]
pub struct EnvVars {
    pub spec_path: String,
    pub tmpl_path: String,
    pub out_path: String,
    pub log_path: String,
}

impl fmt::Display for EnvVars {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "spec_path: {}, tmpl_path: {}, out_path: {}, log_path: {}",
            self.spec_path, self.tmpl_path, self.out_path, self.log_path
        )
    }
}

/// loads `command line arguments` and `environment variables` using custom `clap`
pub fn parse_args() -> Args {
    let matches: clap::ArgMatches = build().get_matches();

    let raw_devices = matches.get_one::<String>("devices").unwrap().to_string();
    let devices: Regex =
        Regex::new(&raw_devices).expect("Invalid regex pattern provided for `devices`");

    let loglevel: LogLevel = match matches.get_flag("debug") {
        true => LogLevel::Debug,
        false => match matches.get_flag("verbose") {
            true => LogLevel::Verbose,
            false => LogLevel::Info,
        },
    };

    let env: EnvVars = parse_env();

    Args {
        devices,
        loglevel,
        env,
    }
}

/// loads only environment variables
pub fn parse_env() -> EnvVars {
    EnvVars {
        spec_path: match std::env::var("SF_SPEC_PATH") {
            Ok(path) => path.trim_end_matches('/').to_string(),
            Err(_) => String::from("./spec"),
        },
        tmpl_path: match std::env::var("SF_TMPL_PATH") {
            Ok(path) => path.trim_end_matches('/').to_string(),
            Err(_) => String::from("./tmpl"),
        },
        out_path: match std::env::var("SF_OUT_PATH") {
            Ok(path) => path.trim_end_matches('/').to_string(),
            Err(_) => String::from("./out"),
        },
        log_path: match std::env::var("SF_LOG_PATH") {
            Ok(path) => path.trim_end_matches('/').to_string(),
            Err(_) => String::from("./log"),
        },
    }
}

const ABOUT_MSG: &str = r#"Skyforge Config Generation Engine"#;
const ENV_MSG: &str = r#"Environment:
    SF_SPEC_PATH    Path to the directory containing templates.        Defaults to "./spec".
    SF_TMPL_PATH    Path to the directory containing specifications.   Defaults to "./tmpl".
    SF_OUT_PATH     Path to the directory for command output.          Defaults to "./out".
    SF_LOG_PATH     Path to the directory for log output.              Defaults to "./log".
"#;

/// builds a custom command line argument parser
fn build() -> Command {
    Command::new("app")
        .about(ABOUT_MSG)
        .version(env!("CARGO_PKG_VERSION"))
        .author("rskntroot")
        .arg(
            Arg::new("devices")
                .long("devices")
                .short('d')
                .help("A regular expression pattern")
                .required(true),
        )
        .arg(
            Arg::new("debug")
                .long("debug")
                .help("Print debug information")
                .action(ArgAction::SetTrue)
                .required(false),
        )
        .arg(
            Arg::new("verbose")
                .short('v')
                .long("verbose")
                .help("Print verbose information")
                .action(ArgAction::SetTrue)
                .required(false),
        )
        .group(
            ArgGroup::new("loglevel")
                .args(&["debug", "verbose"])
                .required(false),
        )
        .after_help(ENV_MSG)
}
