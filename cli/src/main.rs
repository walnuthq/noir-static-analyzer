use nargo::package::{Package, PackageType};
use nargo::workspace::Workspace;
use noir_analyzer::ast::analyzer::Analyzer;
use noir_analyzer::ast::parser::Parser;
use noir_analyzer::diagnostics::reporter::Reporter;
use noir_analyzer::lints::lint_rule::LintRule;
use serde::Deserialize;
use std::collections::BTreeMap;
use std::fs;
use std::path::PathBuf;

/// CLI arguments for the Noir Analyzer.
#[derive(Debug, clap::Parser)]
#[command(
    name = "noir-analyzer",
    version = "0.1.0",
    about = "Static analysis for Noir programs"
)]
struct Cli {
    /// Path to the Nargo.toml file
    #[arg(long, value_name = "PATH", default_value = "Nargo.toml")]
    manifest_path: PathBuf,
}

#[derive(Debug, Deserialize)]
struct NargoToml {
    package: PackageConfig,
    _dependencies: Option<BTreeMap<String, DependencyConfig>>,
}

#[derive(Debug, Deserialize)]
struct PackageConfig {
    name: String,
    version: Option<String>,
    #[serde(rename = "type")]
    package_type: String,
    entry: Option<String>,
    compiler_version: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum DependencyConfig {
    Path { _path: String },
    Git { _git: String, _tag: String },
}

fn main() {
    let args = <Cli as clap::Parser>::parse();
    println!("Using manifest path: {:?}", args.manifest_path);

    match parse_workspace(&args.manifest_path) {
        Ok(workspace) => {
            println!("Workspace root: {:?}", workspace.root_dir);
            for package in &workspace.members {
                println!("Package: {}", package.name);
                println!("Entry point: {:?}", package.entry_path);

                // Run linters on the entrypoint
                if let Err(e) = run_linters(&package.entry_path) {
                    eprintln!("Error running linters: {:?}", e);
                }
            }
        }
        Err(e) => eprintln!("Error parsing Nargo.toml: {:?}", e),
    }
}

/// Parses `Nargo.toml` and constructs a `Workspace`
fn parse_workspace(manifest_path: &PathBuf) -> Result<Workspace, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(manifest_path)?;
    let parsed: NargoToml = toml::from_str(&content)?;

    let package_type = match parsed.package.package_type.as_str() {
        "bin" => PackageType::Binary,
        "lib" => PackageType::Library,
        "contract" => PackageType::Contract,
        _ => return Err("Invalid package type in Nargo.toml".into()),
    };

    let package = Package {
        name: parsed
            .package
            .name
            .parse()
            .map_err(|_| "Invalid package name")?,
        version: parsed.package.version,
        compiler_required_version: parsed.package.compiler_version,
        root_dir: manifest_path.parent().unwrap().to_path_buf(),
        entry_path: manifest_path
            .parent()
            .unwrap()
            .join(parsed.package.entry.unwrap_or_else(|| "src/main.nr".into())),
        package_type,
        dependencies: BTreeMap::new(),
        expression_width: None,
    };

    let workspace = Workspace {
        root_dir: manifest_path.parent().unwrap().to_path_buf(),
        target_dir: None,
        members: vec![package],
        selected_package_index: None,
        is_assumed: false,
    };

    Ok(workspace)
}

/// Runs lint rules on the given entry point
/// Runs lint rules on the given entry point
fn run_linters(entry_path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    // Read the source file
    let source = fs::read_to_string(entry_path)?;

    let parsed_module = Parser::parse_program_with_dummy_file(&source)
        .map_err(|_| "Failed to parse entry point")?;

    // Collect all registered lints
    let lints: Vec<Box<dyn LintRule>> = vec![Box::new(
        noir_analyzer::lints::unused_function::UnusedFunction,
    )];

    let mut analyzer = Analyzer::new(&lints);
    match analyzer.analyze(&parsed_module) {
        Ok(lints) => {
            // Pass entry_path to pretty_report instead of FileManager
            println!("{}", Reporter::pretty_report(&lints, entry_path));
        }
        Err(_) => println!("Ignore errors in PoC"),
    }

    Ok(())
}
