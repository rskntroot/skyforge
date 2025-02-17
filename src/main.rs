mod cli;
mod log;
mod specs;
mod tmpls;

use log::LogLevel;

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
            serde_json::to_string_pretty(&spec.compiled).unwrap()
        );
        info!(dbg, "Rendered Config:");
        for line in result {
            if line != "\n" {
                print!("{}", line)
            }
        }
    }
}
