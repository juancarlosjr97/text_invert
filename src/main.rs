use clap::Parser;
use std::thread;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(
        short,
        long,
        help = "Word(s) to be reversed or checked for palindrome. It can be specified multiple times."
    )]
    word: Vec<String>,

    #[arg(
        short = 'p',
        long = "palindrome",
        help = "Check if the word(s) are palindromes instead of reversing."
    )]
    palindrome: bool,
}

fn reverse_word(word: &str) -> String {
    word.chars().rev().collect()
}

fn is_palindrome(word: &str) -> bool {
    let lower = word.to_lowercase();
    lower.chars().eq(lower.chars().rev())
}

fn run(word: &str, palindrome: bool) -> Result<String, String> {
    if word.is_empty() {
        return Err("Error: The word cannot be empty.".to_string());
    }
    if palindrome {
        if is_palindrome(word) {
            Ok(format!("{} is a palindrome", word))
        } else {
            Ok(format!("{} is not a palindrome", word))
        }
    } else {
        Ok(reverse_word(word))
    }
}

fn main() {
    let args = Args::parse();

    let handles: Vec<_> = args
        .word
        .iter()
        .map(|word| {
            let word = word.clone();
            let palindrome = args.palindrome;
            thread::spawn(move || match run(&word, palindrome) {
                Ok(output) => println!("{}", output),
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
    fn test_is_palindrome() {
        assert!(is_palindrome("racecar"));
        assert!(is_palindrome("RaceCar"));
        assert!(is_palindrome("a"));
        assert!(is_palindrome(""));
        assert!(!is_palindrome("hello"));
        assert!(!is_palindrome("rust"));
    }

    #[test]
    fn test_run() {
        assert_eq!(run("hello", false).unwrap(), "olleh");
        assert_eq!(run("rust", false).unwrap(), "tsur");
        assert_eq!(run("", false).unwrap_err(), "Error: The word cannot be empty.");
        assert_eq!(run("a", false).unwrap(), "a");
        assert_eq!(run("racecar", true).unwrap(), "racecar is a palindrome");
        assert_eq!(run("hello", true).unwrap(), "hello is not a palindrome");
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

    #[test]
    fn test_main_palindrome_flag() {
        let output = Command::new("cargo")
            .arg("run")
            .arg("--")
            .arg("--word")
            .arg("racecar")
            .arg("--palindrome")
            .output()
            .expect("Check palindrome");
        assert!(output.status.success());
        let output_str = String::from_utf8_lossy(&output.stdout);
        assert!(output_str.contains("racecar is a palindrome"));

        let output = Command::new("cargo")
            .arg("run")
            .arg("--")
            .arg("--word")
            .arg("hello")
            .arg("-p")
            .output()
            .expect("Check not palindrome");
        assert!(output.status.success());
        let output_str = String::from_utf8_lossy(&output.stdout);
        assert!(output_str.contains("hello is not a palindrome"));
    }
}
