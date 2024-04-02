struct Word {
    content:String,
    kind:WordType
}

impl Word{
    fn new(&self, text:&str) -> Word {
        Word {
            content: String::from(&text),
            kind: getKind()
        }
    }

    fn get_kind(&self, text:&str) -> WordType {
        match &str {
            "value" | "function" | "target" | "if" | "else" | "for" | "while" | "print" | "call" | "go" => WordType::Keyword,
            "("|")"|"["|"]"|"{"|"}"|"<="|">="|"!="|"=="|"<"|">"|"=" | "" => WordType::Symbol,
        }
        WordType::UserDefined
    }

    ///Need to add functionality to this.
    fn get_words(line:String) -> Vec<Word> {
        Vec::new()
    }
}

enum WordType{
    Keyword, Symbol, UserDefined
}