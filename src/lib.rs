peg::parser! {
    /// Grammar for a simple English parser
    ///
    /// `english_parser` module contains rules to parse words, numbers,
    /// punctuation, sentences and whole tetx in English.
    pub grammar english_parser() for str {
        /// `word` matches a word, which is a sequence of alphabetic characters with optinal symbols - and '
        pub rule word() -> &'input str
            = $(['a'..='z' | 'A'..='Z']+ (['-' | '\'']['a'..='z' | 'A'..='Z']+)* (['\''])?)

        /// `capital_word` matches a word that starts with a capital letter.
        pub rule capital_word() -> &'input str
            = $(['A'..='Z']['a'..='z' | 'A'..='Z']+ (['-' | '\'']['a'..='z' | 'A'..='Z']+)* (['\''])?)

        /// `number` matches a sequence of numeric characters.
        pub rule number() -> &'input str
            = $(['0'..='9']+ ("." ['0'..='9']+)?)

        /// `date` matches dates in the format dd/mm/yyyy.
        pub rule date() -> &'input str
            = $(day:['0'..='3']? ['0'..='9'] "/" month:['0'..='1']? ['0'..='9'] "/" year:['1'..='9']['0'..='9']['0'..='9']['0'..='9'])

        /// `hour` matches times in the format hh:mm (am|pm).
        pub rule hour() -> &'input str
            = $(hour:['0'..='1']? ['0'..='9'] ":" minute:['0'..='5']['0'..='9'] " " period:("am" / "pm"))

        /// `end_punctuation` matches sentence-ending punctuation.
        pub rule end_punctuation() -> &'input str
            = $("..." / ['.' | '?' | '!'])

        // `other_punctuation` matches non sentence-ending punctuation.
        pub rule other_punctuation() -> &'input str
            = $([',' | ';' | ':' | '-'])

        /// `whitespace` matches any whitespace character.
        pub rule whitespace() -> &'input str
            = $([' ' | '\t' | '\n' | '\r'])

        /// `sentence` matches a sequence of words, numbers and other punctuation ending with sentence-ending punctuation.
        pub rule sentence() -> Vec<&'input str>
            = capital_w:capital_word() sequence:((word() / date() / hour() / number() / whitespace() / other_punctuation())*) end_punct:end_punctuation() {
                let mut sequence_vec = sequence.to_vec();
                sequence_vec.insert(0, capital_w);
                sequence_vec.push(end_punct);
                sequence_vec
            }

        /// `text` matches a series of sentences, separated by whitespace.
        ///
        /// This rule can be used to parse entire paragraphs or documents.
        pub rule text() -> Vec<Vec<&'input str>>
            = sentences:(sentence() ** whitespace()) {
                sentences
            }
    }
}
