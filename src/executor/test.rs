use crate::executor::{DataTypes, Executor};
use crate::lexer::Lexer;
use crate::parser::parse;
use std::collections::HashMap;
#[test]
fn primary_test() {
    let code = "
        i=0;
        i um 0 um onnan enkil {
            i = 10;
        } adhallengil {
            i = -1;
        }
        i um 0 um onnalla enkil avarthikuga {
            i = i-1;
        }
        ezhuthuga i;
    ";

    let mut lex = Lexer::new(&code, HashMap::new(), 0);
    let parsed = parse(&code, &mut lex);
    let mut exec = Executor::new(lex.literal_table, lex.symbol_lookup);

    assert!(exec.execute(&parsed.unwrap()).is_ok());
}

#[test]
fn arithmetic_test() {
    let code = "
        i = 55/5+22*7-3;
        j = i % 3;
    ";
    let exec = get_executor(code).unwrap();
    assert_eq!(
        exec.symbol_table[&(exec.frame_level, exec.symbol_lookup_table["i"])],
        DataTypes::Integer(162)
    );
    assert_eq!(
        exec.symbol_table[&(exec.frame_level, exec.symbol_lookup_table["j"])],
        DataTypes::Integer(0)
    );
}
#[test]
fn malayalam_test() {
    let code = "
        നമ്പർ = 1;
        നമ്പർ = നമ്പർ + 1;
        10 നെകാൾ നമ്പർ cheruthan enkil avarthikuga {
            നമ്പർ%2 um 0 um thullyaman enkil {
                നമ്പർ = നമ്പർ+2;
            } adhallengil {
                നമ്പർ = നമ്പർ+1;
            }
        }
    ";
    let exec = get_executor(code).unwrap();
    assert_eq!(
        exec.symbol_table[&(exec.frame_level, exec.symbol_lookup_table["നമ്പർ"])],
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
