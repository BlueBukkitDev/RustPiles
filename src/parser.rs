
struct Word {
    content:String,
    kind:WordType
}

impl Word {
    pub fn new(&self, text: &str) -> Word {
        Word {
            content: String::from(&text),
            kind: Word::getKind(text)
        }
    }
    ///Gets a Vec of Words from a line of text.
    pub fn get_words(line: String) -> Vec<Word> {
        let chars = line.chars().collect();
        let mut words = Vec::new();
        let mut builder = "";
        let mut whitespace = false;
        let mut i = 0;

        while i < chars.len() {
            if chars[i] == " " {
                whitespace = true;
            } else {
                whitespace = false;
                builder = builder + chars[i];
            }
            if Word::get_kind(builder) == WordType::Symbol {
                if Word::get_kind(builder + chars[i + 1]) == WordType::Symbol {
                    words.add(builder);
                    builder = "";
                    i += 2;
                    continue;
                }
                words.add(builder);
                i += 1;
                continue;
            } else if Word::get_kind(builder) == WordType::Keyword {}
            i += 1;
        }
    }

    pub fn get_kind(text:&str) -> WordType {
        match text {
            "value" | "function" | "target" | "if" | "else" | "for" | "while" | "print" | "call" | "go" => WordType::Keyword,
            "("|")"|"["|"]"|"{"|"}"|"<="|">="|"!="|"=="|"<"|">"|"=" | "" => WordType::Symbol,
            _ => WordType::UserDefined
        }
    }
}

enum WordType{
    Keyword, Symbol, UserDefined
}