# Rustacean English Vocab
Well, this was fun—my first Rust project! The main goal was that I needed a small project touching on a bunch of little things that I could slowly iterate and improve on as I learn more about the Rust language.

This project came about because I recently rediscovered my interest in Morse Code. It got me thinking about what makes someone fluent in a language, or at least fluent in vocabulary. The idea was, if I were to provide a vocabulary list of the top 1000 most used words in the English language, how far would that take you in an average everyday conversation, a news article, or even a book? Could you approach learning using the 80/20 rule? What if you wanted to experiment and develop your own word lists, maybe context specific words, or maybe go the next level with popular phrases?

On my quest to learn Morse Code, I might fork this into something more specialized or, expand this into other languages, and finally get back to learning German again. But if all fails, at least I'm learning Rust, eh?

## Installing Rust
```
brew install rustup
rustup-init
```

## Running the Project
```
cargo install
cargo run -- "I am a sentence with words" ./english-words/5000-top-words.txt
```

or

```
./english_vocab "I am a sentence with words" ./english-words/5000-top-words.txt
```

## Current Output Example
Using the supplied 2500 most popular English words and using a random paragraph generator, here is what it looks like.

```
cargo run -- "If you're looking for random paragraphs, you've come to the right place. When a random word or a random sentence isn't quite enough, the next logical step is to find a random paragraph. We created the Random Paragraph Generator with you in mind. The process is quite simple. Choose the number of random paragraphs you'd like to see and click the button. Your chosen number of paragraphs will instantly appear." ./english-words/2500-top-words.txt

Results
Original paragraph:
If you're looking for random paragraphs, you've come to the right place. When a random word or a random sentence isn't quite enough, the next logical step is to find a random paragraph. We created the Random Paragraph Generator with you in mind. The process is quite simple. Choose the number of random paragraphs you'd like to see and click the button. Your chosen number of paragraphs will instantly appear.

Normalized paragraph:
if youre looking for random paragraphs youve come to the right place when a random word or a random sentence isnt quite enough the next logical step is to find a random paragraph we created the random paragraph generator with you in mind the process is quite simple choose the number of random paragraphs youd like to see and click the button your chosen number of paragraphs will instantly appear

Understood paragraph:
if _ looking for random _ _ come to the right place when a random word or a random _ _ quite enough the next _ step is to find a random _ we created the random _ _ with you in mind the process is quite simple choose the number of random _ _ like to see and click the button your _ number of _ will _ appear

Unique words: 49
Found words: 38
Total unknown words: 11
Unknown word list: isnt, paragraph, chosen, logical, youd, paragraphs, generator, youre, instantly, sentence, youve
Understanding: 77.55%
```

## Creating Your Own English Word Lists
In the `english-words` folder, I have supplied a great resource of the 10,000 most common English words or order of frequency, which I have borrowed from https://github.com/first20hours/google-10000-english/.

To create your own variation from this list, for example, you want to see the impact of knowing just 500 of the most frequently used words, or maybe 5000, you can run this on Linux or MacOs:
```
head -500 english-words/google-10000-english/google-10000-english-usa-no-swears.txt > english-words/500-top-words.txt
```


## Todo's
A few things I would like to improve on:
- Read the vocab file line by line instead of reading the whole thing into memory
- Maybe offer support for CSV files vs single word lines, or autodetect
- I would also like to pre-bundle a few vocab files with the project for various learning methodologies/strategies
- 20 mins in, I started to wonder about naming conventions, utility files, etc
- Probably handle errors better. I wonder what I can do to crash this thing?
- Ideally I would like to have smaller more functional style functions
- Introduce some testing. Testing things like unicode characters, utility functions around normalizing, and calculating results, etc
- Provide some better command line support in the form of flags, options and arg error handling
- Answer the question: will I ever be a die hard Rustacean? I hope so!