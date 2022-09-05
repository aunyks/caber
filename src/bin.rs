use caber::{Binary, CaberConfig, CaberError, ExportMode};
use clap::Parser;
use std::fs::File;
use std::io::{self, prelude::*};
use std::path::Path;
use std::str::FromStr;

#[derive(Parser, Debug)]
enum OutputLanguage {
    EcmaScript,
    TypeScript,
}

impl FromStr for OutputLanguage {
    type Err = CaberError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "ecmascript" | "javascript" | "js" => Ok(Self::EcmaScript),
            "typescript" | "ts" => Ok(Self::TypeScript),
            _ => Err(Self::Err::UnrecognizedLanguage),
        }
    }
}

#[derive(Parser, Debug)]
#[clap(
    author = "Gerald Nash (aunyks)",
    version,
    about = "Embed binary data into JavaScript and TypeScript projects."
)]
struct CliArgs {
    /// The binary to be embedded
    pub input_file: String,
    /// The output language of the binary (javascript / ecmascript, typescript, etc)
    #[clap(short = 'l', long)]
    pub output_lang: Option<OutputLanguage>,
    /// The export mode of the typed array (default, object, or none)
    #[clap(short = 'm', long)]
    pub export_mode: Option<ExportMode>,
    /// The output file containing the embedded file
    #[clap(short = 'o', long)]
    pub output_file: Option<String>,
}

fn main() -> Result<(), io::Error> {
    let args = CliArgs::parse();
    // dbg!(&args);
    let input_filepath = Path::new(&args.input_file);
    let displayable_input_filepath = input_filepath.display();
    let mut input_file = match File::open(&input_filepath) {
        Err(why) => {
            eprintln!("Couldn't open {}: {}", displayable_input_filepath, why);
            std::process::exit(1);
        }
        Ok(input_file) => input_file,
    };
    let mut input_file_bytes = Vec::new();
    match input_file.read_to_end(&mut input_file_bytes) {
        Err(why) => {
            eprintln!("Couldn't read {}: {}", displayable_input_filepath, why);
            std::process::exit(1);
        }
        _ => {}
    };

    let binary = Binary::new(&input_file_bytes).with_config(CaberConfig {
        export_mode: args.export_mode.unwrap_or(ExportMode::Default),
    });

    let output_string = match args.output_lang.unwrap_or(OutputLanguage::EcmaScript) {
        OutputLanguage::EcmaScript => binary.to_ecmascript(),
        OutputLanguage::TypeScript => binary.to_typescript(),
    };

    if args.output_file == Some(String::from("-")) || args.output_file.is_none() {
        println!("{}", output_string);
        std::process::exit(0);
    }

    let output_filepath = args.output_file.unwrap();
    let output_filepath = Path::new(&output_filepath);

    let mut output_file = File::create(output_filepath)?;
    output_file.write_all(output_string.as_bytes())?;

    Ok(())
}
