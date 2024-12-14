use clap::Parser;
use std::thread;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(
        short,
        long,
        help = "Word(s) to be reversed. It can be specified multiple times."
    )]
    word: Vec<String>,
}

fn reverse_word(word: &str) -> String {
    word.chars().rev().collect()
}

fn run(word: &str) -> Result<String, String> {
    if word.is_empty() {
        return Err("Error: The word cannot be empty.".to_string());
    }

    Ok(reverse_word(word))
}

fn main() {
    let args = Args::parse();

    let handles: Vec<_> = args
        .word
        .iter()
        .map(|word| {
            let word = word.clone();
            thread::spawn(move || match run(&word) {
                Ok(reversed_word) => println!("{}", reversed_word),
                Err(err) => {
                    eprintln!("{}", err);
                    std::process::exit(1);
                }
            })
        })
        .collect();

    for handle in handles {
        handle.join().expect("Thread panicked");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::process::Command;

    #[test]
    fn test_reverse_word() {
        assert_eq!(reverse_word("hello"), "olleh");
        assert_eq!(reverse_word("rust"), "tsur");
        assert_eq!(reverse_word(""), "");
        assert_eq!(reverse_word("a"), "a");
    }

    #[test]
    fn test_run() {
        assert_eq!(run("hello").unwrap(), "olleh");
        assert_eq!(run("rust").unwrap(), "tsur");
        assert_eq!(run("").unwrap_err(), "Error: The word cannot be empty.");
        assert_eq!(run("a").unwrap(), "a");
    }

    #[test]
    fn test_main() {
        let output = Command::new("cargo")
            .arg("run")
            .arg("--")
            .arg("--word")
            .arg("jcbd")
            .arg("--word")
            .arg("1234")
            .output()
            .expect("Reverse the word successfully");

        assert!(output.status.success());
        let output_str = String::from_utf8_lossy(&output.stdout);
        assert!(output_str.contains("dbcj"));
        assert!(output_str.contains("4321"));
    }

    #[test]
    fn test_main_empty_word() {
        let output = Command::new("cargo")
            .arg("run")
            .arg("--")
            .arg("--word")
            .arg("")
            .output()
            .expect("Failed to execute the cli command");

        assert!(!output.status.success());
        assert!(
            String::from_utf8_lossy(&output.stderr).contains("Error: The word cannot be empty.")
        );
    }
}
