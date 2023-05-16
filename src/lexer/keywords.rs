use crate::lexer::tokens::TokenType;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct Keywords {
    pub list: HashMap<String, TokenType>,
}

// Fancy macro for ease of adding new keywords
macro_rules! keywordize {
    ($( $words:expr => $func:expr ), *) => {{
        let mut list = HashMap::new();
        $( for &word in &($words) { list.insert(word.to_string(), $func); } )*
        list
    }};
}

impl Keywords {
    pub fn new() -> Self {
        // The following list contain keywords that are named meme_style
        // Last word will be in മലയാളലിപി(Malayalalipi)
        let list = keywordize!(
            ["vachaakam_vangikuga","വാചകംവാങ്ങിക്കുക"] => TokenType::InputString,
            ["akam_vangikuga", "അക്കംവാങ്ങിക്കുക"] => TokenType::InputNumber,
            ["ezhuthuka","ezhuthuga","kanikuka","kanikuga","എഴുതുക","കാണിക്കുക"] => TokenType::Write,
            ["enkil", "എങ്കിൽ"] => TokenType::If,
            ["adhallengil","adhallangil", "അതല്ലെങ്കിൽ"] => TokenType::Else,
            ["aavarthikuga","avarthikuga","ആവർത്തിക്കുക",""] => TokenType::Loop,
            ["veluthan","വലുതാണ്"] => TokenType::GreaterThan,
            ["thullyamalla","onnalla", "തുല്യമല്ല","ഒന്നല്ല"] => TokenType::NotEqual,
            ["cheruthan", "ചെറുതാണ്", ] => TokenType::LessThan,
            ["thullyaman","onnan", "തുല്യമാണ്","ഒന്നാണ്"] => TokenType::EqualTo,
            ["um", "ഉം"] => TokenType::Um,
            ["ne_kal", "നെകാൾ"] => TokenType::Nekal,
            ["kodukuga","kodukuka","madakiayakuga","കൊടുക്കുക","മടക്കിഅയയ്ക്കുക"] => TokenType::Return
        );

        Self { list }
    }
}
