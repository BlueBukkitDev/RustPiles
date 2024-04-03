struct Word {
    content:String,
    kind:WordType
}

impl Word{
    pub fn new(&self, text:&str) -> Word {
        Word {
            content: String::from(&text),
            kind: getKind()
        }
    }
    ///Need to add functionality to this.
    fn get_words(line:String) -> Vec<Word> {
        let chars = line.chars().collect();
        let words = Vec::new();
        let builder = "";
        let whitespace = false;

        while i < chars.len() {
            if chars[i] == " " {
                whitespace = true;
            }else{
                whitespace = false;
                builder = builder+chars[i];
            }
            if get_kind(builder) == WordType::Symbol {
                if get_kind(builder+chars[i+1]) == WordType::Symbol {
                    words.add(builder);
                    builder = "";
                    i+=2;
                    continue;
                }
                words.add(builder);
                i+=1;
                continue;
            }else if get_kind(builder) == WordType::Keyword {

            }
            i+=1;
        }
    }
}

pub fn get_kind(text:&str) -> WordType {
    match &str {
        "value" | "function" | "target" | "if" | "else" | "for" | "while" | "print" | "call" | "go" => WordType::Keyword,
        "("|")"|"["|"]"|"{"|"}"|"<="|">="|"!="|"=="|"<"|">"|"=" | "" => WordType::Symbol,
    }
    WordType::UserDefined
}

enum WordType{
    Keyword, Symbol, UserDefined
}