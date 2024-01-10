use crate::executor::ast::Expression::*;
use crate::executor::ast::SourceUnitPart::Statement;
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
        kanikuga i;
    ";
    let mut lex = Lexer::new(&code, HashMap::new(), 0);
    let parsed = parse(&code, &mut lex);

    println!("{:?}", parsed);
    let expected = SourceUnit(
        [
            Statement(Assignment(
                (9, 13),
                Symbol((9, 9), TokenType::Symbol(1)),
                Integer((11, 12), TokenType::Integer(0)),
            )),
            Statement(Conditional(
                (22, 126),
                NotEquals(
                    (32, 43),
                    Box::new(Symbol((22, 22), TokenType::Symbol(1))),
                    Box::new(Integer((27, 28), TokenType::Integer(0))),
                ),
                SourceUnit(
                    [Statement(Assignment(
                        (65, 72),
                        Symbol((65, 65), TokenType::Symbol(1)),
                        Integer((69, 71), TokenType::Integer(10)),
                    ))]
                    .to_vec(),
                ),
                Some(SourceUnit(
                    [Statement(Assignment(
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
            Statement(Loop(
                (135, 208),
                NotEquals(
                    (145, 156),
                    Box::new(Symbol((135, 135), TokenType::Symbol(1))),
                    Box::new(Integer((140, 141), TokenType::Integer(0))),
                ),
                SourceUnit(
                    [Statement(Assignment(
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
            Statement(Write((217, 228), Symbol((226, 226), TokenType::Symbol(1)))),
        ]
        .to_vec(),
    );
    assert_eq!(expected, parsed.unwrap());
}
