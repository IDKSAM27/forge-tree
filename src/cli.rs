use crate::{Generator, Parser, Result};
use clap::{Arg, ArgMatches, Command};
use colored::*;
use std::collections::HashMap;

pub struct Cli;

impl Cli {
    pub fn new() -> Command {
        Command::new("forge-tree")
            .version(env!("CARGO_PKG_VERSION"))
            .about("Forge project structures from text representations")
            .author("Sampreet Patil <sampreetpatil270@gmail.com>")
            .subcommand_required(true)
            .arg_required_else_help(true)
            .subcommand(
                Command::new("forge")
                    .about("Forge a project structure from a text file")
                    .arg(
                        Arg::new("input")
                            .help("Input file containing the project structure")
                            .required(true)
                            .index(1)
                    )
                    .arg(
                        Arg::new("output")
                            .short('o')
                            .long("output")
                            .help("Output directory (default: current directory)")
                            .default_value(".")
                    )
                    .arg(
                        Arg::new("force")
                            .short('f')
                            .long("force")
                            .help("Force overwrite existing files")
                            .action(clap::ArgAction::SetTrue)
                    )
                    .arg(
                        Arg::new("verbose")
                            .short('v')
                            .long("verbose")
                            .help("Verbose output")
                            .action(clap::ArgAction::SetTrue)
                    )
                    .arg(
                        Arg::new("variable")
                            .long("var")
                            .help("Set template variables (format: key=value)")
                            .action(clap::ArgAction::Append)
                    )
            )
            .subcommand(
                Command::new("validate")
                    .about("Validate a structure file without forging it")
                    .arg(
                        Arg::new("input")
                            .help("Input file to validate")
                            .required(true)
                            .index(1)
                    )
            )
    }

    pub fn run(matches: ArgMatches) -> Result<()> {
        match matches.subcommand() {
            Some(("forge", sub_matches)) => Self::handle_forge(sub_matches),
            Some(("validate", sub_matches)) => Self::handle_validate(sub_matches),
            _ => unreachable!(),
        }
    }

    fn handle_forge(matches: &ArgMatches) -> Result<()> {
        let input_file = matches.get_one::<String>("input").unwrap();
        let output_dir = matches.get_one::<String>("output").unwrap();
        let force = matches.get_flag("force");
        let verbose = matches.get_flag("verbose");
        let variables = Self::parse_variables(matches);

        if verbose {
            println!("{} Parsing structure from: {}", "📖".cyan(), input_file);
        }

        let parser = Parser::new();
        let mut structure = parser.parse_file(input_file)?;

        // Add user-provided variables
        structure.variables.extend(variables);

        if verbose {
            println!("{} Root: {}", "🌳".green(), structure.root);
            println!("{} Items: {}", "📁".blue(), Self::count_total_items(&structure.items));
        }

        let generator = Generator::new()
            .with_verbose(verbose);

        generator.generate(&structure, output_dir)?;

        Ok(())
    }

    fn handle_validate(matches: &ArgMatches) -> Result<()> {
        let input_file = matches.get_one::<String>("input").unwrap();

        println!("{} Validating: {}", "🔍".cyan(), input_file);

        let parser = Parser::new();
        let structure = parser.parse_file(input_file)?;

        println!("{} Structure is valid!", "✅".green());
        println!("  {} Root: {}", "🌳".green(), structure.root);
        println!("  {} Total items: {}", "📊".blue(), Self::count_total_items(&structure.items));

        Ok(())
    }

    fn parse_variables(matches: &ArgMatches) -> HashMap<String, String> {
        let mut variables = HashMap::new();

        if let Some(vars) = matches.get_many::<String>("variable") {
            for var in vars {
                if let Some((key, value)) = var.split_once('=') {
                    variables.insert(key.to_string(), value.to_string());
                } else {
                    eprintln!("{} Invalid variable format: {} (expected key=value)", 
                             "⚠️".yellow(), var);
                }
            }
        }

        variables
    }

    fn count_total_items(items: &[crate::parser::StructureItem]) -> usize {
        let mut count = items.len();
        for item in items {
            count += Self::count_total_items(&item.children);
        }
        count
    }
}
