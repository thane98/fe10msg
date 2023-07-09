use std::path::PathBuf;
use indexmap::IndexMap;
use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    Extract {
        #[arg(short, long)]
        output: Option<PathBuf>,
        archive: PathBuf,
    },
    Import {
        #[arg(short, long)]
        output: Option<PathBuf>,
        script: PathBuf,
    },
}

fn main() {
    match Args::parse().command {
        Command::Extract { output, archive } => {
            let raw = std::fs::read(&archive).expect("Failed to read archive");
            let message_archive = mila::TextArchive::from_bytes(
                &raw,
                mila::TextArchiveFormat::ShiftJIS,
                mila::Endian::Big,
            )
            .expect("Failed to deserialize archive");
            let escaped_archive: IndexMap<String, String> = message_archive.get_entries()
                .iter()
                .map(|(k, v)| (k.to_string(), v.to_string()))
                .collect();
            let raw = serde_yaml::to_string(&escaped_archive).expect("Failed to serialize archive");
            let path = output.unwrap_or_else(|| archive.with_extension("yml"));
            std::fs::write(path, raw).expect("Failed to write output to disk");
        }
        Command::Import {
            output,
            script,
        } => {
            let raw = std::fs::read_to_string(&script).expect("Failed to read script");
            let mut message_archive = mila::TextArchive::new(mila::TextArchiveFormat::ShiftJIS, mila::Endian::Big);
            let entries: IndexMap<String, String> = serde_yaml::from_str(&raw).expect("Failed to parse message YAML");
            for (k, v) in entries {
                message_archive.set_message(&k, &v);
            }
            let raw = message_archive.serialize().expect("Failed to serialize message archive");
            let path = output.unwrap_or_else(|| script.with_extension("m"));
            std::fs::write(path, raw).expect("Failed to write output to disk");
        }
    }
}
