use crate::lexer::tokens::TokenType;
use std::collections::HashMap;
#[derive(Clone, Debug)]
pub struct Keywords<'input> {
    pub list: HashMap<&'static str, TokenType<'input>>,
}
impl<'input> Keywords<'input> {
    pub fn new() -> Self {
        let mut list = HashMap::new();
        list.insert("pwoli_sadhanam", TokenType::Declaration);
        list.insert("pwoli_sanam", TokenType::Declaration);
        list.insert("pwoli_saanam", TokenType::Declaration);
        
        list.insert("poli_sadhanam", TokenType::Declaration);
        list.insert("poli_sanam", TokenType::Declaration);
        list.insert("poli_saanam", TokenType::Declaration);

        list.insert("address_thada", TokenType::InputString);
        list.insert("number_thada", TokenType::InputNumber);
        list.insert("dhe_pidicho", TokenType::Write);
        
        list.insert("seriyano_mwone", TokenType::If);
        list.insert("seriyano", TokenType::If);
        
        list.insert("seri_allel", TokenType::Else);
        list.insert("repeat_adi", TokenType::Loop);
        list.insert("veluthane", TokenType::GreaterThan);
        
        list.insert("same_alle", TokenType::NotEqual);
        list.insert("same_allel", TokenType::NotEqual);
        
        list.insert("cheruthane", TokenType::LessThan);
        list.insert("same_aane", TokenType::EqualTo);
        list.insert("um", TokenType::Um);
        list.insert("ne_kal", TokenType::Nekal);
        Self { list }
    }
}
