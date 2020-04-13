use crate::lexer::tokens::TokenType;
use crate::encoding::to_ascii;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct Keywords<'input> {
    pub list: HashMap<String, TokenType<'input>>,
}

impl<'input> Keywords<'input> {
    pub fn new() -> Self {
        let mut list = HashMap::new();
        list.insert("pwoli_sadhanam".to_string(), TokenType::Declaration);
        list.insert("address_thada".to_string(), TokenType::InputString);
        list.insert("number_thada".to_string(), TokenType::InputNumber);
        list.insert("dhe_pidicho".to_string(), TokenType::Write);
        list.insert("seriyano_mwone".to_string(), TokenType::If);
        list.insert("seri_allel".to_string(), TokenType::Else);
        list.insert("repeat_adi".to_string(), TokenType::Loop);
        list.insert("veluthane".to_string(), TokenType::GreaterThan);
        list.insert("same_alle".to_string(), TokenType::NotEqual);
        list.insert("cheruthane".to_string(), TokenType::LessThan);
        list.insert("same_aane".to_string(), TokenType::EqualTo);
        list.insert("um".to_string(), TokenType::Um);
        list.insert("ne_kal".to_string(), TokenType::Nekal);
        list.insert("pwoli_sanam".to_string(), TokenType::Declaration);
        list.insert("pwoli_saanam".to_string(), TokenType::Declaration);
        
        list.insert("poli_sadhanam".to_string(), TokenType::Declaration);
        list.insert("poli_sanam".to_string(), TokenType::Declaration);
        list.insert("poli_saanam".to_string(), TokenType::Declaration);

        list.insert("address_thada".to_string(), TokenType::InputString);
        list.insert("number_thada".to_string(), TokenType::InputNumber);
        list.insert("dhe_pidicho".to_string(), TokenType::Write);
        
        list.insert("seriyano_mwone".to_string(), TokenType::If);
        list.insert("seriyano".to_string(), TokenType::If);
        
        list.insert("seri_allel".to_string(), TokenType::Else);
        list.insert("repeat_adi".to_string(), TokenType::Loop);
        list.insert("veluthane".to_string(), TokenType::GreaterThan);
        
        list.insert("same_alle".to_string(), TokenType::NotEqual);
        list.insert("same_allel".to_string(), TokenType::NotEqual);
        
        list.insert("cheruthane".to_string(), TokenType::LessThan);
        list.insert("same_aane".to_string(), TokenType::EqualTo);
        list.insert("um".to_string(), TokenType::Um);
        list.insert("ne_kal".to_string(), TokenType::Nekal);
        
        //Malayalam counterparts
        list.insert(to_ascii("പൊളിസാധനം"), TokenType::Declaration);
        list.insert(to_ascii("അഡ്രസ്_താടാ"), TokenType::InputString);
        list.insert(to_ascii("നമ്പർ_താടാ"), TokenType::InputNumber);
        list.insert(to_ascii("ദേ_പിടിച്ചോ"), TokenType::Write);
        list.insert(to_ascii("ശെരിയാണോ_മോനെ"), TokenType::If);
        list.insert(to_ascii("ശെരി_അല്ലേൽ"), TokenType::Else);
        list.insert(to_ascii("റിപീറ്റടി"), TokenType::Loop);
        list.insert(to_ascii("വലുതാണെ"), TokenType::GreaterThan);
        list.insert(to_ascii("സെയിം_അല്ല"), TokenType::NotEqual);
        list.insert(to_ascii("ചെറുതാണെ"), TokenType::LessThan);
        list.insert(to_ascii("സെയിം_ആണേ"), TokenType::EqualTo);
        list.insert(to_ascii("ഉം"), TokenType::Um);
        list.insert(to_ascii("നെകാൾ"), TokenType::Nekal);
        
        Self { list }
    }
}
