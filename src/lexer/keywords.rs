use crate::lexer::tokens::TokenType;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct Keywords {
    pub list: HashMap<String, TokenType>,
}

impl Keywords {
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
        list.insert("പൊളിസാധനം".to_string(), TokenType::Declaration);
        list.insert("അഡ്രസ്_താടാ".to_string(), TokenType::InputString);
        list.insert("നമ്പർ_താടാ".to_string(), TokenType::InputNumber);
        list.insert("ദേ_പിടിച്ചോ".to_string(), TokenType::Write);
        list.insert("ശെരിയാണോ_മോനെ".to_string(), TokenType::If);
        list.insert("ശെരി_അല്ലേൽ".to_string(), TokenType::Else);
        list.insert("റിപീറ്റടി".to_string(), TokenType::Loop);
        list.insert("വലുതാണെ".to_string(), TokenType::GreaterThan);
        list.insert("സെയിം_അല്ല".to_string(), TokenType::NotEqual);
        list.insert("ചെറുതാണെ".to_string(), TokenType::LessThan);
        list.insert("സെയിം_ആണേ".to_string(), TokenType::EqualTo);
        list.insert("ഉം".to_string(), TokenType::Um);
        list.insert("നെകാൾ".to_string(), TokenType::Nekal);
        
        Self { list }
    }
}
