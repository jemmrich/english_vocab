use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashSet;
use regex::Regex;

// How to run:
// cargo install
// cargo run -- "I am a sentence with words" ./english-words/1000-top-words.txt
// or
// ./english_vocab "I am a sentence with words" ./english-words/1000-top-words.txt

struct VocabResults {
  total_words: i32,
  total_unknown_words: usize,
  unknown_word_list: String,
  words_found: i32,
  understanding: f32,
  understood_paragraph: String,
}

fn main() {
  let args: Vec<String> = env::args().collect();
  let paragraph: String = String::from(&args[1]);
  let file_path: String = String::from(&args[2]);

  // Normalize the paragraph for easier processing
  let normalized_paragraph: String = remove_punctuation(&paragraph);

  let results: VocabResults = analyze_paragraph(&normalized_paragraph, &file_path);

  // Show the results
  println!("\nResults");
  println!("Original paragraph: \n{0}\n", paragraph);
  println!("Normalized paragraph: \n{0}\n", normalized_paragraph);
  println!("Understood paragraph: \n{0}\n", results.understood_paragraph);

  println!("Unique words: {0}", results.total_words);
  println!("Found words: {0}", results.words_found);
  println!("Total unknown words: {0}", results.total_unknown_words);
  println!("Unknown word list: {0}", results.unknown_word_list);
  println!("Understanding: {0:.2}%", results.understanding);
}

/**
 * Analyze a paragraph and return a tuple of results.
 * TODO: I have a feeling this could be waaay done better if I knew more about Rust.
 * Specifically, I would like to be able to read a file line by line instead of
 * reading the whole thing into memory and iterating over it. I'm just imagining 50,000 words
 * loaded into memory, or worse, a multi GB file. Also, who's to say what is even in this file?
 */
fn analyze_paragraph(paragraph: &str, file_path: &str) -> VocabResults {
  let Ok(lines) = read_lines(file_path) else {
    panic!("Could not read file");
  };

  // Split the paragraph into words, create a unique word hash set
  let words: Vec<&str> = paragraph.split_whitespace().collect();
  let unique_words: HashSet<&str> = words.iter().cloned().collect();

  // Lets make it easy and start with all the words and remove them as we find them
  let mut unknown_words: HashSet<&str> = unique_words.clone();

  let total_words: i32 = unique_words.len().try_into().unwrap();
  let mut words_found: i32 = 0;

  // Read the lines from the file
  for line in lines {
    if let Ok(word) = line {

      // Have we found all the words in the hash?
      if words_found == total_words {
        break;
      }

      // Does the word exist in the hash set?
      if unique_words.get(word.as_str()) == None {
        continue;
      }

      // If we found the word, lets remove it from the unknown hash if it exists
      unknown_words.remove(word.as_str());

      words_found += 1;
    }
  }

  // Debug party
  let _all_unique: String = unique_words.clone().into_iter().collect::<Vec<&str>>().join(", ");
  let _all_unknown: String = unknown_words.clone().into_iter().collect::<Vec<&str>>().join(", ");

  // An understanding level based on percentage of words found
  let understanding: f32 = (words_found as f32 / total_words as f32) * 100.0;

  // Create a new paragraph with unknown words replaced with underscores
  let mut understood_paragraph: String = paragraph.clone().to_string();

  unknown_words.clone().into_iter().for_each(|word| {
    // Replace only full words not portions
    let pattern: String = format!(r"\b{}\b", word);
    let reg = Regex::new(pattern.as_str()).unwrap();
    understood_paragraph = reg.replace_all(understood_paragraph.as_str(), "_").to_string();
  });

  let total_unknown_words: usize = unknown_words.len();

  let results: VocabResults = VocabResults {
    total_words,
    total_unknown_words,
    unknown_word_list: unknown_words.into_iter().collect::<Vec<&str>>().join(", "),
    words_found,
    understanding,
    understood_paragraph,
  };

  return results;
}

/**
 * Remove punctuation and non-alphabetic characters from a string
 */
fn remove_punctuation(paragraph: &String) -> String {
  // Remove any whitespace and lowercase
  let mut normalized: String = paragraph.trim().to_lowercase();

  // Remove any non-alphabetic characters except whitespace
  normalized = normalized.replace(|c: char| !c.is_alphabetic() && !c.is_whitespace(), "");
  return normalized;
}

/**
 * Read a file and do things with fancy results and stuff. I'll need to read up on this because as
 * it stands, all I see is <.::> everywhere lol.
 */
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path> + Copy, P: std::fmt::Display {
  let file: Result<File, io::Error> = File::open(filename);
  match file {
    Ok(file) => Ok(io::BufReader::new(file).lines()),
    Err(error) => panic!("Problem opening the file `{0}` with error {1}", filename.to_string(), error),
  }
}
