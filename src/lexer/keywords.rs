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
            ["pwoli_sadhanam", "pwoli_sanam", "pwoli_saanam", "poli_sadhanam", 
            "poli_sanam", "poli_saanam", "പൊളിസാധനം", ] => TokenType::Declaration,
            ["address_thada", "അഡ്രസ്_താടാ"] => TokenType::InputString,
            ["number_thada", "നമ്പർ_താടാ"] => TokenType::InputNumber,
            ["dhe_pidicho", "ദേ_പിടിച്ചോ"] => TokenType::Write,
            ["seriyano_mwone", "seriyano", "ശെരിയാണോ_മോനെ","ശെരിയാണോ"] => TokenType::If,
            ["seri_allel", "ശെരി_അല്ലേൽ"] => TokenType::Else,
            ["repeat_adi", "റിപീറ്റടി"] => TokenType::Loop,
            ["veluthane", "veluthanenkil", "veluthanekil", "valuthanenkil",
            "valuthanekil", "valuthane", "വലുതാണെ", "വലുതാണെങ്കിൽ", "വലുതാണെകിൽ", ] => TokenType::GreaterThan,
            ["same_alle", "same_allel", "സെയിം_അല്ല"] => TokenType::NotEqual,
            ["cheruthane", "cheruthanenkil", "cheruthanekil", "charuthane",
            "charuthanenkil", "charuthanekil", "ചെറുതാണെകിൽ", "ചെറുതാണെങ്കിൽ", ] => TokenType::LessThan,
            ["same_aane", "സെയിം_ആണേ"] => TokenType::EqualTo,
            ["um", "ഉം"] => TokenType::Um,
            ["ne_kal", "നെകാൾ"] => TokenType::Nekal
        );

        Self { list }
    }
}
