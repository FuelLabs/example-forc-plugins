use clap::Parser;
use colored::*;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use std::process;

#[derive(Parser, Debug)]
#[command(version = "1.0.0")]
#[command(about = "Forc.toml metadata parser that reads and validates ABI schema")]
struct Args {
    #[arg(short, long, help = "Path to the Forc.toml file")]
    path: String,
}

#[derive(Debug, Deserialize)]
struct ForcConfig {
    project: Project,
}

#[derive(Debug, Deserialize)]
struct Project {
    metadata: Option<Metadata>,
}

#[derive(Debug, Deserialize)]
struct Metadata {
    indexing: Option<Indexing>,
}

#[derive(Debug, Deserialize)]
struct Indexing {
    schema_path: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
struct AbiLoggedType {
    #[serde(rename = "logId")]
    log_id: String,
    #[serde(rename = "concreteTypeId")]
    concrete_type_id: String,
}

#[derive(Debug, Deserialize)]
struct AbiSchema {
    #[serde(rename = "loggedTypes")]
    logged_types: Option<Vec<AbiLoggedType>>,
}

fn main() {
    let args = Args::parse();

    // Read and validate Forc.toml
    println!("{}", "\n📖 Reading Forc.toml configuration...".blue().bold());
    let content = match fs::read_to_string(&args.path) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("{} {}", "❌ Error:".red().bold(), e.to_string().red());
            process::exit(1);
        }
    };

    let forc_config: ForcConfig = match toml::from_str(&content) {
        Ok(config) => config,
        Err(e) => {
            eprintln!("{} {}", "❌ Error:".red().bold(), e.to_string().red());
            process::exit(1);
        }
    };

    // Validate the required structure exists
    let schema_path = match forc_config
        .project
        .metadata
        .as_ref()
        .and_then(|m| m.indexing.as_ref())
        .and_then(|i| i.schema_path.as_ref())
    {
        Some(path) => path,
        None => {
            eprintln!(
                "{} {}",
                "❌ Error:".red().bold(),
                "Missing required project.metadata.indexing.schema_path in Forc.toml".red()
            );
            process::exit(1);
        }
    };

    // Get the ABI schema path and read it
    let project_path = Path::new(&args.path).parent().unwrap_or(Path::new(""));
    let abi_path = project_path.join(schema_path);

    println!("{}", "✓ Forc.toml validated successfully".green());
    println!("{}", "\n📑 Reading ABI schema...".blue().bold());
    println!("{}", format!("Path: {}", abi_path.display()).dimmed());

    let abi_content = match fs::read_to_string(abi_path) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("{} {}", "❌ Error:".red().bold(), e.to_string().red());
            process::exit(1);
        }
    };

    let abi_schema: AbiSchema = match serde_json::from_str(&abi_content) {
        Ok(schema) => schema,
        Err(e) => {
            eprintln!("{} {}", "❌ Error:".red().bold(), e.to_string().red());
            process::exit(1);
        }
    };

    let logged_types = match abi_schema.logged_types {
        Some(types) => types,
        None => {
            eprintln!(
                "{} {}",
                "❌ Error:".red().bold(),
                "No logged types field in ABI".red()
            );
            process::exit(1);
        }
    };

    if logged_types.is_empty() {
        println!("{}", "⚠️  No logged types found in ABI".yellow());
        process::exit(1);
    }

    println!("{}", "✓ ABI schema loaded successfully".green());
    println!("{}", "\n📋 Logged Types:".magenta().bold());

    for (index, logged_type) in logged_types.iter().enumerate() {
        println!("\n{}", format!("[Entry {}]", index + 1).cyan());
        println!(
            "{}",
            format!(
                "  🔍 Log ID: {}",
                logged_type.log_id.to_string().white()
            )
            .yellow()
        );
        println!(
            "{}",
            format!(
                "  🔑 Concrete Type ID: {}",
                logged_type.concrete_type_id.to_string().white()
            )
            .yellow()
        );
    }

    println!(); // Add extra newline for cleaner output
}