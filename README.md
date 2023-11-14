## Description

Simple parser of English sentences created for KMA Rust course. Parser can identify single words, numbers, punctuation symbols, whitespaces, sentences and whole text. 
[crates.io](https://crates.io/crates/english-language-parser)

## Usage
```
make run ARGS="-f test_files/test1.txt"
```
Output:
```
["Hello", ",", " ", "world", "!"]
```

Or to get help information:
```
make
```

## Techical
Parser uses `peg` library. Rules:

- `word()` matches a word, which is a sequence of alphabetic characters with optinal symbols - and '
- `capital_word()`  matches a word that starts with a capital letter.
- `number()` rule is used to parse numbers.
- `date()` matches dates in the format dd/mm/yyyy.
- `hour()` matches times in the format hh:mm (am|pm).
- `end_punctuation()` rule is used to parse punctuation marks that can end a sentence: `... | . | ! | ?`
- `other_punctuation()` rule is used to parse punctuation marks that can be inside a sentence: `, | ; | : | -`
- `whitespace()` rule is used to parse spaces or other identation symbols like `'\t' | '\n' | '\r'`
- `sentence()` rule is used to parse the whole sentence. It uses all three previous rules to parse the input string. Sentence must start with a capital word and end in an `end_punctuation`
- `text()` rule can parse multiple sentences