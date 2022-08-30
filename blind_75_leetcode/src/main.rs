use std::error::Error;
use std::fs;
use std::io::Write;
use std::path::Path;

macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<dyn Error>::from(format!($($tt)*))) }
}
type Result<T> = ::std::result::Result<T, Box<dyn Error>>;

use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Problem id
    #[clap(value_parser)]
    id: String,

    /// Problem name
    #[clap(value_parser = parse_name)]
    name: String,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let path_string = format!("./src/solution/s{:0>4}_{}.rs", args.id, args.name);
    let path = Path::new(&path_string);
    if path.exists() {
        return err!("File {} exists, do nothing.", path_string);
    }
    println!(
        "gen problem id: {}, name: {} at: {}",
        args.id, args.name, path_string
    );
    write_template_code(
        &gen_template_code(&read_template().unwrap(), &args.id),
        &path,
    )?;
    insert_mod(&format!("s{:0>4}_{}", args.id, args.name))
}

fn read_template() -> Result<String> {
    match fs::read_to_string("./template.rs") {
        Ok(s) => Ok(s),
        Err(e) => err!("Problem reading the template file: {:?}", e),
    }
}

fn gen_template_code(template: &str, problem_id: &str) -> String {
    template.replace("__PROBLEM_ID__", problem_id)
}

fn write_template_code(code: &str, path: &Path) -> Result<()> {
    match fs::write(path, code) {
        Ok(s) => Ok(s),
        Err(e) => err!("Problem writing the code template file: {:?}", e),
    }
}

fn insert_mod(mod_name: &str) -> Result<()> {
    let mod_name = format!("mod {};", mod_name);
    let mut mod_file = match fs::OpenOptions::new()
        .write(true)
        .append(true)
        .open("./src/solution/mod.rs")
    {
        Ok(f) => f,
        Err(e) => return err!("Problem opening file ./src/solution/mod.rs, Error: {:?}", e),
    };

    match writeln!(mod_file, "{}", &mod_name) {
        Ok(s) => Ok(s),
        Err(e) => err!(
            "Problem writing the mod {} to the ./src/solution/mod.rs, Error: {}",
            mod_name,
            e
        ),
    }
}

fn parse_name(name: &str) -> std::result::Result<String, String> {
    // replace - with _ with name input
    Ok(name.replace("-", "_"))

    // ToDo: More Validated
}
