use crate::executor::{DataTypes, Executor};
use crate::lexer::Lexer;
use crate::parser::parse;
use std::collections::HashMap;
#[test]
fn primary_test() {
    let code = "
        pwoli_sadhanam i;
        i=0;
        seriyano_mwone i um 0 um same_aane {
            i = 10;
        } seri_allel {
            i = -1;
        }
        repeat_adi i um 0 um same_alle {
            i = i-1;
        }
        dhe_pidicho i;
    ";

    let mut lex = Lexer::new(&code, HashMap::new(), 0);
    let parsed = parse(&code, &mut lex);
    let mut exec = Executor::new(lex.literal_table, lex.symbol_lookup);

    assert!(exec.execute(&parsed.unwrap()).is_ok());
}

#[test]
fn arithmetic_test() {
    let code = "
        pwoli_sadhanam i;
        pwoli_sadhanam j;
        i = 55/5+22*7-3;
        j = i % 3;
    ";
    let exec = get_executor(code).unwrap();
    assert_eq!(
        exec.symbol_table[&exec.symbol_lookup_table["i"]],
        DataTypes::Integer(162)
    );
    assert_eq!(
        exec.symbol_table[&exec.symbol_lookup_table["j"]],
        DataTypes::Integer(0)
    );
}
#[test]
fn malayalam_test() {
    let code = "
        പൊളിസാധനം നമ്പർ;
        നമ്പർ = 1;
        നമ്പർ = നമ്പർ + 1;
        റിപീറ്റടി 10 നെകാൾ നമ്പർ ചെറുതാണെകിൽ {
            ശെരിയാണോ നമ്പർ%2 um 0 um സെയിം_ആണേ {
                നമ്പർ = നമ്പർ+2;
            } ശെരി_അല്ലേൽ {
                നമ്പർ = നമ്പർ+1;
            }
        }
    ";
    let exec = get_executor(code).unwrap();
    assert_eq!(
        exec.symbol_table[&exec.symbol_lookup_table["നമ്പർ"]],
        DataTypes::Integer(10)
    );
}

fn get_executor(code: &str) -> Result<Executor, ()> {
    let mut lex = Lexer::new(&code, HashMap::new(), 0);
    let parsed = parse(&code, &mut lex);
    let mut exec = Executor::new(lex.literal_table, lex.symbol_lookup);
    exec.execute(&parsed.unwrap()).unwrap();
    Ok(exec)
}
