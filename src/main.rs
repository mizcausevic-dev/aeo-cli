//! Command-line tool for the AEO Protocol v0.1.
//!
//! Subcommands: validate, fetch, inspect, claim.
use std::fs;
use std::path::PathBuf;
use std::process::ExitCode;

use aeo_protocol::{fetch_well_known, well_known_url, AuditMode, Document};
use anyhow::{anyhow, Context, Result};
use clap::{Parser, Subcommand};
use colored::Colorize;

#[derive(Parser)]
#[command(name = "aeo")]
#[command(version, about = "AEO Protocol v0.1 command-line tool")]
#[command(
    long_about = "Validate, fetch, and inspect AEO Protocol declaration documents.\n\nSpec: https://github.com/mizcausevic-dev/aeo-protocol-spec"
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Validate a local AEO document file against the v0.1 schema.
    Validate {
        /// Path to the JSON file to validate.
        file: PathBuf,
    },
    /// Fetch and pretty-print the AEO declaration at an origin's well-known URL.
    Fetch {
        /// Origin URL (e.g. https://mizcausevic-dev.github.io).
        origin: String,
    },
    /// Show a structured summary of an AEO document (entity, claim count, audit mode).
    Inspect {
        /// Either a file path or an origin URL.
        target: String,
    },
    /// Extract and print a specific claim by ID.
    Claim {
        /// Either a file path or an origin URL.
        target: String,
        /// Claim ID to extract.
        id: String,
    },
}

fn main() -> ExitCode {
    let cli = Cli::parse();
    match run(cli) {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("{} {:#}", "error:".red().bold(), err);
            ExitCode::from(1)
        }
    }
}

fn run(cli: Cli) -> Result<()> {
    match cli.command {
        Command::Validate { file } => cmd_validate(file),
        Command::Fetch { origin } => cmd_fetch(&origin),
        Command::Inspect { target } => cmd_inspect(&target),
        Command::Claim { target, id } => cmd_claim(&target, &id),
    }
}

fn cmd_validate(path: PathBuf) -> Result<()> {
    let raw = fs::read_to_string(&path).with_context(|| format!("reading {}", path.display()))?;
    let doc = Document::from_json(&raw).context("parsing AEO document")?;
    println!(
        "{} {} — {} ({} claims)",
        "OK".green().bold(),
        path.display(),
        doc.entity.name,
        doc.claims.len()
    );
    Ok(())
}

fn cmd_fetch(origin: &str) -> Result<()> {
    let url = well_known_url(origin);
    eprintln!("{} {}", "fetching:".cyan().bold(), url.dimmed());
    let doc = fetch_well_known(origin).context("fetching well-known document")?;
    println!("{}", doc.to_json()?);
    Ok(())
}

fn cmd_inspect(target: &str) -> Result<()> {
    let doc = load(target)?;
    println!(
        "{}            {}",
        "Protocol:".bold(),
        doc.aeo_version.green()
    );
    println!(
        "{}              {}",
        "Entity:".bold(),
        format!("{:?}", doc.entity.entity_type).cyan()
    );
    println!("{}                {}", "Name:".bold(), doc.entity.name);
    println!(
        "{}                  {}",
        "ID:".bold(),
        doc.entity.id.dimmed()
    );
    println!(
        "{}       {}",
        "Canonical URL:".bold(),
        doc.entity.canonical_url.dimmed()
    );
    println!(
        "{}     {}",
        "Primary sources:".bold(),
        doc.authority.primary_sources.len()
    );
    if let Some(verifications) = &doc.authority.verifications {
        println!("{}      {}", "Verifications:".bold(), verifications.len());
    }
    println!("{}              {}", "Claims:".bold(), doc.claims.len());
    for c in &doc.claims {
        println!(
            "  - {} {} ({:?})",
            c.id.green(),
            c.predicate.dimmed(),
            c.confidence
        );
    }
    if let Some(audit) = &doc.audit {
        let mode = match audit.mode {
            AuditMode::None => "none".dimmed().to_string(),
            AuditMode::Signature => "signature".yellow().to_string(),
            AuditMode::Endpoint => "endpoint".magenta().to_string(),
        };
        println!("{}          {}", "Audit mode:".bold(), mode);
    }
    Ok(())
}

fn cmd_claim(target: &str, id: &str) -> Result<()> {
    let doc = load(target)?;
    let claim = doc
        .find_claim(id)
        .ok_or_else(|| anyhow!("claim '{}' not found in document", id))?;
    let json = serde_json::to_string_pretty(claim)?;
    println!("{}", json);
    Ok(())
}

fn load(target: &str) -> Result<Document> {
    if target.starts_with("http://") || target.starts_with("https://") {
        fetch_well_known(target).with_context(|| format!("fetching from {target}"))
    } else {
        let raw = fs::read_to_string(target).with_context(|| format!("reading {target}"))?;
        Document::from_json(&raw).with_context(|| format!("parsing {target}"))
    }
}
