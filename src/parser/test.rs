use crate::executor::ast::Expression::*;
use crate::executor::ast::SourceUnitPart::Statement as Stmt;
use crate::executor::ast::Statement::*;
use crate::executor::ast::*;
use crate::lexer::tokens::TokenType;

use crate::lexer::Lexer;
use crate::parser::parse;
use std::collections::HashMap;

#[test]
fn parser_test() {
    let code = "
        i=0;
        i um 0 um thullyamalla enkil {
            i = 10;
        } adhallengil {
            i = -1;
        }
        i um 0 um thullyamalla enkil avarthikuga {
            i = i-1;
        }
        i kanikuga;
    ";
    let mut lex = Lexer::new(&code, HashMap::new(), 0);
    let parsed = parse(&code, &mut lex);

    println!("{:?}", parsed);
    let expected = SourceUnit(
        [
            Stmt(Assignment(
                (9, 13),
                Symbol((9, 9), TokenType::Symbol(1)),
                Integer((11, 12), TokenType::Integer(0)),
            )),
            Stmt(Conditional(
                (22, 126),
                NotEquals(
                    (32, 43),
                    Box::new(Symbol((22, 22), TokenType::Symbol(1))),
                    Box::new(Integer((27, 28), TokenType::Integer(0))),
                ),
                SourceUnit(
                    [Stmt(Assignment(
                        (65, 72),
                        Symbol((65, 65), TokenType::Symbol(1)),
                        Integer((69, 71), TokenType::Integer(10)),
                    ))]
                    .to_vec(),
                ),
                Some(SourceUnit(
                    [Stmt(Assignment(
                        (109, 116),
                        Symbol((109, 109), TokenType::Symbol(1)),
                        UnaryMinus(
                            (113, 114),
                            Box::new(Integer((114, 115), TokenType::Integer(1))),
                        ),
                    ))]
                    .to_vec(),
                )),
            )),
            Stmt(Loop(
                (135, 208),
                NotEquals(
                    (145, 156),
                    Box::new(Symbol((135, 135), TokenType::Symbol(1))),
                    Box::new(Integer((140, 141), TokenType::Integer(0))),
                ),
                SourceUnit(
                    [Stmt(Assignment(
                        (190, 198),
                        Symbol((190, 190), TokenType::Symbol(1)),
                        Subtract(
                            (195, 196),
                            Box::new(Symbol((194, 194), TokenType::Symbol(1))),
                            Box::new(Integer((196, 197), TokenType::Integer(1))),
                        ),
                    ))]
                    .to_vec(),
                ),
            )),
            Stmt(Write((217, 228), Symbol((217, 217), TokenType::Symbol(1)))),
        ]
        .to_vec(),
    );
    assert_eq!(expected, parsed.unwrap());
}

#[test]
fn parser_import_test() {
    let code = "
        math:operations m ennu ulppeduthuka;
        math:utils ulppeduthuka;
    ";
    let mut lex = Lexer::new(&code, HashMap::new(), 0);
    let parsed = parse(&code, &mut lex);
    assert!(parsed.is_ok());

    let unit = parsed.unwrap();
    assert_eq!(unit.0.len(), 2);

    if let SourceUnitPart::Statement(Statement::Import(_, path, alias)) = &unit.0[0] {
        assert_eq!(path.len(), 2);
        assert!(alias.is_some());
    } else {
        panic!("Expected import statement with alias");
    }

    if let SourceUnitPart::Statement(Statement::Import(_, path, alias)) = &unit.0[1] {
        assert_eq!(path.len(), 2);
        assert!(alias.is_none());
    } else {
        panic!("Expected import statement without alias");
    }
}
