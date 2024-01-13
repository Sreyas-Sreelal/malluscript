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
            ["vachakam_vangikuga","vachakam_vangikuka","vachagam_vangikuga","vachagam_vangikuka","വാചകംവാങ്ങിക്കുക"] => TokenType::InputString,
            ["akam_vangikuga", "akkam_vangikuga","അക്കംവാങ്ങിക്കുക"] => TokenType::InputNumber,
            ["ezhuthuka","ezhuthuga","kanikuka","kanikuga","എഴുതുക","കാണിക്കുക"] => TokenType::Write,
            ["enkil","engil" ,"എങ്കിൽ"] => TokenType::If,
            ["adhallengil","adhallenkil", "അതല്ലെങ്കിൽ"] => TokenType::Else,
            ["aavarthikuga","avarthikuga","aavarthikuka","avarthikuka","ആവർത്തിക്കുക"] => TokenType::Loop,
            ["veluthan","വലുതാണ്"] => TokenType::GreaterThan,
            ["veluthanenkil","veluthanengil","വലുതാണെങ്കിൽ"] => TokenType::IfGreaterThan,
            ["thullyamalla","onnalla", "തുല്യമല്ല","ഒന്നല്ല"] => TokenType::NotEqual,
            ["thullyamallenkil","thullyamallengil","onnallenkil","onnallengil","തുല്യമല്ലെങ്കിൽ","ഒന്നല്ലെങ്കിൽ"] => TokenType::IfNotEqual,
            ["cheruthan", "ചെറുതാണ്", ] => TokenType::LessThan,
            ["cheruthanenkil","cheruthanengil", "ചെറുതാണെങ്കിൽ", ] => TokenType::IfLessThan,
            ["thullyaman","onnan","orupole","തുല്യമാണ്","ഒന്നാണ്","ഒരുപോലെ"] => TokenType::EqualTo,
            ["thullyamanenkil","thullyamanengil","onnanenkil","onnanengil", "orupoleyanenkil","orupoleyanengil","തുല്യമാണെങ്കിൽ","ഒന്നാണെങ്കിൽ","ഒരുപോലെയാണെങ്കിൽ"] => TokenType::IfEqualTo,
            ["um", "ഉം"] => TokenType::Um,
            ["ne_kal", "നെകാൾ"] => TokenType::Nekal,
            ["kodukuga","kodukuka","madakiayakuga","madakiayakuka","കൊടുക്കുക","മടക്കിഅയയ്ക്കുക"] => TokenType::Return
        );

        Self { list }
    }
}
