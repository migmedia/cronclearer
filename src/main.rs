// (c) 2025 migmedia
use std::{
    fmt::Display,
    fs::{self, File},
    io::Result as IoResult,
    path::PathBuf,
    process::{self, Command, Output},
};
use tempfile::TempDir;

/// An executable command with its parameters.
struct Exec {
    program: String,
    params: Vec<String>,
}

impl Display for Exec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.program, self.params.join(" "))
    }
}

fn main() -> IoResult<()> {
    // Get command line arguments (skipping the program name)
    let mut check_stderr = true;
    let args: Vec<String> = {
        let args: Vec<String> = std::env::args().skip(1).collect();
        if let Some((prog_idx, _)) = args.iter().enumerate().find(|(_, p)| !p.starts_with("-")) {
            for arg in &args[0..prog_idx] {
                match arg.as_str() {
                    "-h" | "--help" => print_usage(),
                    "-i" | "--ignore-text" => check_stderr = false,
                    p => {
                        eprintln!("Unknown parameter: {p}");
                        process::exit(1);
                    }
                }
            }
            args[prog_idx..].to_vec()
        } else {
            args
        }
    };

    if args.is_empty() {
        print_usage();
    }
    let exec = Exec {
        program: args[0].clone(),
        params: args[1..].to_vec(),
    };

    // Create temporary directory
    let tmp_dir = TempDir::new()?;
    let out_path = tmp_dir.path().join("croncls.out");
    let trace_path = tmp_dir.path().join("croncls.trace");

    // Run the command and capture output
    let output = execute(&exec, &out_path, &trace_path)?;

    // Process the trace output
    let ps4 = std::env::var("PS4").unwrap_or_else(|_| "+ ".to_string());
    let trace_content = fs::read_to_string(&trace_path)?;
    let std_err = trace_content
        .lines()
        .filter(|line| !line.starts_with(&ps4))
        .collect::<Vec<_>>()
        .join("\n");

    let status = output.status.code().unwrap_or(-1);

    // Check if there was an error or non-empty error output
    if status != 0 || (check_stderr && !std_err.is_empty()) {
        println!("# Failure or error output for the command:");
        println!("`{exec}`");
        println!("\n## Resultcode: {status}");
        println!("\n## Err output:");
        println!("```\n{std_err}\n```");
        println!("\n## Std output:");

        let std_out = fs::read_to_string(&out_path)?;
        println!("```\n{std_out}\n```");

        if std_err.trim() != trace_content.trim() {
            println!("\n## Trace output:");
            println!("```\n{trace_content}\n```");
        }
    }

    process::exit(status);
}

fn print_usage() {
    eprintln!("Usage: cronclearer [-ih] <command> [args...]");
    eprintln!("\nOptions:");
    eprintln!("    -h, --help        Show this usage information.");
    eprintln!("    -i, --ignore-text React only on exit-code, not on text on stderr.");
    process::exit(1);
}

/// Execute the given command and capture its output.
fn execute(exec: &Exec, stdout: &PathBuf, stderr: &PathBuf) -> IoResult<Output> {
    let mut command = Command::new(&exec.program);
    command.args(&exec.params);

    // Redirect stdout and stderr to files
    let stdout_file = File::create(stdout)?;
    let stderr_file = File::create(stderr)?;

    command.stdout(stdout_file).stderr(stderr_file).output()
}
