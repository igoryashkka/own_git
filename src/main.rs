use clap::{Parser, Subcommand};
use git_starter_rust::git;
use std::fs;

#[derive(Parser)]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[command(name = "init")]
    Init,
    #[command(name = "cat-file")]
    ReadObject {
        #[clap(short = 'p')]
        file_sha: String,
    },
    #[command(name = "hash-object")]
    WriteObject {
        #[clap(short = 'w')]
        content_file: String,
    },
    #[command(name = "ls-tree")]
    ReadTree {
        #[clap(long = "name-only")]
        tree_sha: String,
    },
    #[command(name = "write-tree")]
    WriteTree,
    #[command(name = "commit-tree")]
    CommitTree {
        tree_sha: String,
        #[clap(short)]
        parent: String,
        #[clap(short)]
        message: String,
    },
    #[command(name = "clone")]
    Clone { repo_url: String, dir: String },
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Init => git::do_git_init()?,
        Commands::ReadObject { file_sha } => git::read_git_object(file_sha)?,
        Commands::WriteObject { content_file } => {
            let content_file = fs::read(content_file).unwrap();
            println!("{}", git::write_git_object(content_file, "blob", "./")?);
        }
        Commands::ReadTree { tree_sha } => git::read_tree_object(tree_sha.to_string())?,
        Commands::WriteTree => {
            let res = git::write_tree_object(&".".to_string())?;
            println!("{res}");
        }
        Commands::CommitTree {
            tree_sha,
            parent,
            message,
        } => {
            println!(
                "{}",
                git::do_commit(
                    tree_sha.to_string(),
                    parent.to_string(),
                    message.to_string(),
                )?
            );
        }
        Commands::Clone { repo_url, dir } => {
            println!("repo : {} , dir {} : ", repo_url, dir);
            let res = git::clone_repo(repo_url.to_string(), dir.to_string());
            if let Err(err) = res {
                eprintln!("Error: {}", err);
            }
        }
    }
    Ok(())
}
