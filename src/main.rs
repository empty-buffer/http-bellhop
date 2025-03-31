use clap::{Arg, Command};
use std::string::ToString;

mod config;
mod error;
mod files;
mod parser;
mod request;

pub use self::error::{Error, Result};
const DEFAULT_ENV: &str = "default";

/// `Opt` is a structure that handles command-line arguments for the HTTP Bellhop CLI tool.
///
/// It includes the following options:
/// - `env`: defines which environment setup should be used. It can be set using `-e` or `--env` command-line flags.
/// - `file`: specifies the location of a JSON file to run. It can be set with `-f` or `--file` flags.
/// - `dir`: indicates the location of a directory that contains JSON files to run. Use `-d` or `--dir` to set it.
///
/// All the options are not required and have an Option<String> type.
///

// fn run(opt: Opt) -> Result<(), Box<dyn Error>> {
//     let env: String;
//     if opt.env.is_some() {
//         env = opt.env.unwrap_or("default".to_owned());
//     } else {
//         env = DEFAULT_ENV.to_owned();
//     }

//     if opt.file.is_some() {
//         let path = get_cfg(&opt.file.unwrap_or("undefined".to_string()))?;
//         let cfg = config::deserialize_data(path.as_path())?;

//         match cfg.to_request(&env)?.do_request() {
//             Ok(()) => (),
//             Err(e) => println!("Request filed: {:?}", e),
//         }
//     } else if opt.dir.is_some() {
//         let files = &mut vec![];
//         let path = get_cfg(&opt.dir.unwrap_or_else(|| "undefined".to_string()))?;
//         let res = get_files(&path, files);
//         match res {
//             Ok(_) => {
//                 for file in files {
//                     let cfg = config::deserialize_data(file.as_path())?;
//                     match cfg.to_request(&env)?.do_request() {
//                         Ok(()) => (),
//                         Err(e) => println!("Request filed: {:?}", e),
//                     }
//                 }
//             }
//             Err(e) => println!("{:#?}", e),
//         }
//     }
//     println!("-----\nDone");
//     Ok(())
// }

fn main() -> Result<()> {
    let cli = Command::new("bellhop")
        .version("v0.0.1") // change in future to a env or something
        .args([
            Arg::new("run")
                .short('r')
                .long("run")
                .value_parser(["silent", "verbose"])
                .default_value("silent"),
            Arg::new("file")
                .short('f')
                .long("file")
                .default_value("bellhop.json"),
        ])
        .get_matches();

    let run_arg = match cli.get_one::<String>("run") {
        Some(arg) if !arg.is_empty() => arg,
        Some(_) => return Err(Error::BadArguments("can't be empty".to_string())),
        None => return Err(Error::BadArguments("can't be empty".to_string())),
    };

    let file_arg = match cli.get_one::<String>("file") {
        Some(arg) if !arg.is_empty() => arg,
        Some(_) => return Err(Error::BadArguments("can't be empty".to_string())),
        None => return Err(Error::BadArguments("can't be empty".to_string())),
    };

    let path = files::get_config(file_arg.as_str())?;

    let request_conf = config::deserialize_data(path.as_path())?;

    // println!("{:?}", request_conf);

    let requests =
        request_conf.to_request(&config::fields::environment::env::Env::Default.into_inner())?;

    for req in requests.iter() {
        if let Err(e) = req.clone().do_request() {
            println!("{:?}", e)
        }
    }

    Ok(())
}
