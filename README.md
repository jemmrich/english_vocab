# Rustacean English Vocab
Well this was fun, my first Rust project! The main goal was that I needed a small project touching on a bunch of little things that I can use to slowly iterate and improve on as I learn more about the Rust language.

This project came about because I recently rediscovered my interest in Morse Code. It got me thinking about what makes someone fluent in a language, or at least vocab fluent. The idea was, if I was to provide a vocabulary list of the top 1000 most used words in the English language, how far would that take you in an average everyday conversation, a news article, or even a book? Could you approach learning using the 80/20 rule? What if you wanted to experiment and develop your own word lists, maybe context specific words, or maybe going the next level to popular phrases?

On my quest to learn Morse Code, I might fork this into something more specialized, or expand this into other languages and finally get back to learning German again. But if all fails, at least I'm learning Rust, eh?

## Installing Rust
```
brew install rustup
rustup-init
```

## Running the Project
```
cargo install
cargo run -- "I am a sentence with words" ./words.txt
```

or

```
./english_vocab "I am a sentence with words" ./words.txt
```

## Current Output Example
```
cargo run -- "If you're looking for random paragraphs, you've come to the right place. When a random word or a random sentence isn't quite enough, the next logical step is to find a random paragraph. We created the Random Paragraph Generator with you in mind. The process is quite simple. Choose the number of random paragraphs you'd like to see and click the button. Your chosen number of paragraphs will instantly appear." ./1000words.txt

Results
Original Paragraph:
If you're looking for random paragraphs, you've come to the right place. When a random word or a random sentence isn't quite enough, the next logical step is to find a random paragraph. We created the Random Paragraph Generator with you in mind. The process is quite simple. Choose the number of random paragraphs you'd like to see and click the button. Your chosen number of paragraphs will instantly appear.

Normalized Paragraph:
if youre looking for random paragraphs youve come to the right place when a random word or a random sentence isnt quite enough the next logical step is to find a random paragraph we created the random paragraph generator with you in mind the process is quite simple choose the number of random paragraphs youd like to see and click the button your chosen number of paragraphs will instantly appear

Understood Paragraph:
if _ _ for _ _ _ come to the right place when a _ word or a _ _ _ quite enough the next _ step _ to find a _ _ we _ the _ _ _ with you in mind the process _ quite simple choose the number of _ _ _ like to see and _ the _ your _ number of _ will _ appear

Unique words: 49
Unknown words: 17
Found words: 39
Understanding: 79.59%
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