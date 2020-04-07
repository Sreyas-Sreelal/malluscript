use crate::executor::ast::Expression::*;
use crate::executor::ast::SourceUnitPart::Statement;
use crate::executor::ast::Statement::*;
use crate::executor::ast::*;
use crate::lexer::tokens::TokenType::Number;
use crate::lexer::Lexer;
use crate::parser::parse;

#[test]
fn parser_test() {
    let code = "
        pwoli_sadhanam i;
        i=0;
        seriyano_mwone i 0 um_same_alle {
            i = 10;
        } seri_allel {
            i = -1;
        }
        repeat_adi i 0 um_same_alle {
            i = i-1;
        }
        dhe_pidicho i;
    ";
    let lex = Lexer::new(&code);
    let parsed = parse(&code, lex);
    let expected = SourceUnit(
        [
            Statement(Declaration((9, 26), Symbol((24, 24), "i"))),
            Statement(Assignment(
                (35, 39),
                Symbol((35, 35), "i"),
                Integer((37, 38), Number(0)),
            )),
            Statement(Conditional(
                (48, 154),
                NotEquals(
                    (67, 78),
                    Box::new(Symbol((63, 63), "i")),
                    Box::new(Integer((65, 66), Number(0))),
                ),
                SourceUnit(
                    [Statement(Assignment(
                        (94, 101),
                        Symbol((94, 94), "i"),
                        Integer((98, 100), Number(10)),
                    ))]
                    .to_vec(),
                ),
                Some(SourceUnit(
                    [Statement(Assignment(
                        (137, 144),
                        Symbol((137, 137), "i"),
                        UnaryMinus((141, 142), Box::new(Integer((142, 143), Number(1)))),
                    ))]
                    .to_vec(),
                )),
            )),
            Statement(Loop(
                (163, 223),
                NotEquals(
                    (178, 189),
                    Box::new(Symbol((174, 174), "i")),
                    Box::new(Integer((176, 177), Number(0))),
                ),
                SourceUnit(
                    [Statement(Assignment(
                        (205, 213),
                        Symbol((205, 205), "i"),
                        Subtract(
                            (210, 211),
                            Box::new(Symbol((209, 209), "i")),
                            Box::new(Integer((211, 212), Number(1))),
                        ),
                    ))]
                    .to_vec(),
                ),
            )),
            Statement(Write((232, 246), Symbol((244, 244), "i"))),
        ]
        .to_vec(),
    );
    assert_eq!(expected, parsed);
}
