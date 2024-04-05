
struct Word {
    content:String,
    kind:WordType
}

impl Word {
    pub fn new(text: &str) -> Word {
        Word {
            content: String::from(text),
            kind: Word::get_kind(text)
        }
    }
    ///Gets a Vec of Words from a line of text.
    pub fn get_words(line: String) -> Vec<Word> {
        let chars:Vec<char> = line.chars().collect();
        let mut words:Vec<Word> = Vec::new();
        let mut builder = String::new();
        let mut whitespace = false;
        let mut i = 0;

        while i < chars.len() {
            if chars[i] == ' ' {
                whitespace = true;
            } else {
                whitespace = false;
                builder.push(chars[i]);
            }
            if Word::get_kind(&builder.clone()) == WordType::Symbol {
                let mut test = builder.clone();
                test.push(chars[i+1]);
                if Word::get_kind(&test) == WordType::Symbol {
                    words.push(Word::new(&builder.clone()));
                    builder = String::new();
                    i += 2;
                    continue;
                }
                words.push(Word::new(&builder.clone()));
                i += 1;
                continue;
            } else if Word::get_kind(&builder.clone()) == WordType::Keyword {

            }
            i += 1;
        }
        return words;
    }

    pub fn get_kind(text:&str) -> WordType {
        match text {
            "value" | "function" | "target" | "if" | "else" | "for" | "while" | "print" | "call" | "go" => WordType::Keyword,
            "("|")"|"["|"]"|"{"|"}"|"<="|">="|"!="|"=="|"<"|">"|"=" | "" => WordType::Symbol,
            _ => WordType::UserDefined
        }
    }
}

#[derive(PartialEq)]
enum WordType{
    Keyword, Symbol, UserDefined
}