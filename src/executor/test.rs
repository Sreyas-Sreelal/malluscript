use crate::executor::Executor;
use crate::lexer::Lexer;
use crate::parser::parse;

#[test]
fn executor_test() {
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
    let lex = Lexer::new(&code);
    let parsed = parse(&code, lex);
    assert!(Executor::new().execute(&parsed.unwrap()).is_ok());
}
