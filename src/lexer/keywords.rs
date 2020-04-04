use std::collections::HashMap;
use crate::lexer::tokens::TokenType;
#[derive(Clone)]
pub struct Keywords<'input>  {
    pub list:HashMap<&'static str,TokenType<'input> > 
}
impl<'input>  Keywords<'input>  {
    pub fn new() -> Self { 
        let mut list = HashMap::new();
        list.insert("pwoli_sadhanam", TokenType::Declaration);
        list.insert("address_thada", TokenType::InputString);
        list.insert("number_thada", TokenType::InputNumber);
        list.insert("dhe_pidicho", TokenType::Write);
        list.insert("seriyano_mwone", TokenType::If);
        list.insert("seri_allel", TokenType::Else);
        list.insert("repeat_adi", TokenType::Loop);
        list.insert("inekal_veluthane", TokenType::GreaterThan);
        list.insert("um_same_alle", TokenType::NotEqual);
        list.insert("inekal_cheruthane", TokenType::LessThan);
        list.insert("um_same_aane", TokenType::EqualTo);
        Self {
            list
        }
    }
}