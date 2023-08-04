// auto-generated: "lalrpop 0.20.0"
// sha3: 00c7262088a6187b68f7d6faf80a0eff3cd7ab29984ad20975eb41c02cec2322
use crate::lexer::tokens::*;
use crate::lexer::LexicalError;
use crate::executor::ast::*;
#[allow(unused_extern_crates)]
extern crate lalrpop_util as __lalrpop_util;
#[allow(unused_imports)]
use self::__lalrpop_util::state_machine as __state_machine;
extern crate core;
extern crate alloc;

#[rustfmt::skip]
#[allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports, unused_parens, clippy::all)]
mod __parse__SourceUnit {

    use crate::lexer::tokens::*;
    use crate::lexer::LexicalError;
    use crate::executor::ast::*;
    #[allow(unused_extern_crates)]
    extern crate lalrpop_util as __lalrpop_util;
    #[allow(unused_imports)]
    use self::__lalrpop_util::state_machine as __state_machine;
    extern crate core;
    extern crate alloc;
    use super::__ToTriple;
    #[allow(dead_code)]
    pub(crate) enum __Symbol<>
     {
        Variant0(TokenType),
        Variant1(Expression),
        Variant2(alloc::vec::Vec<Expression>),
        Variant3(usize),
        Variant4(Vec<Expression>),
        Variant5(core::option::Option<Expression>),
        Variant6(SourceUnit),
        Variant7(SourceUnitPart),
        Variant8(alloc::vec::Vec<SourceUnitPart>),
        Variant9(Statement),
    }
    const __ACTION: &[i8] = &[
        // State 0
        0, 0, 3, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 35, 36, 0, 0, 0, 0, 0, 37, 38, 39, 0, 5, 40, 6,
        // State 1
        0, 0, 3, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -33, 0, 35, 36, 0, 0, 0, 0, 0, 37, 38, 39, 0, 5, 40, 6,
        // State 2
        0, 0, 3, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 35, 36, 0, 0, 0, 0, 0, 37, 38, 39, 0, 0, 40, 0,
        // State 3
        0, 0, 3, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 35, 36, 0, 0, 0, 0, 0, 37, 38, 39, 0, 0, 40, 0,
        // State 4
        0, 0, 3, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 35, 36, 0, 0, 0, 0, 0, 37, 38, 39, 0, 0, 40, 0,
        // State 5
        0, 0, 3, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 35, 36, 0, 0, 0, 0, 0, 37, 38, 39, 0, 0, 40, 0,
        // State 6
        0, 0, 3, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 35, 36, 0, 0, 0, 0, 0, 37, 38, 39, 0, 0, 40, 0,
        // State 7
        0, 0, 3, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 35, 36, 0, 0, 0, 0, 0, 37, 38, 39, 0, 0, 40, 0,
        // State 8
        0, 0, 3, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 35, 36, 0, 0, 0, 0, 0, 37, 38, 39, 0, 5, 40, 6,
        // State 9
        0, 0, 3, -12, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 35, 36, 0, 0, 0, 0, 0, 37, 38, 39, 0, 0, 40, 0,
        // State 10
        0, 0, 3, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 35, 36, 0, 0, 0, 0, 0, 37, 38, 39, 0, 0, 40, 0,
        // State 11
        0, 0, 3, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 35, 36, 0, 0, 0, 0, 0, 37, 38, 39, 0, 0, 40, 0,
        // State 12
        0, 0, 3, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 35, 36, 0, 0, 0, 0, 0, 37, 38, 39, 0, 0, 40, 0,
        // State 13
        0, 0, 3, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 35, 36, 0, 0, 0, 0, 0, 37, 38, 39, 0, 0, 40, 0,
        // State 14
        0, 0, 3, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 35, 36, 0, 0, 0, 0, 0, 37, 38, 39, 0, 0, 40, 0,
        // State 15
        0, 0, 3, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 35, 36, 0, 0, 0, 0, 0, 37, 38, 39, 0, 0, 40, 0,
        // State 16
        0, 0, 3, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, -12, 0, 0, 0, 0, 0, 0, 35, 36, 0, 0, 0, 0, 0, 37, 38, 39, 0, 0, 40, 0,
        // State 17
        0, 0, 3, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 35, 36, 0, 0, 0, 0, 0, 37, 38, 39, 0, 5, 40, 6,
        // State 18
        0, 0, 3, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 35, 36, 0, 0, 0, 0, 0, 37, 38, 39, 0, 5, 40, 6,
        // State 19
        0, 0, 3, -14, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, -14, 0, 0, 0, 0, 0, 0, 35, 36, 0, 0, 0, 0, 0, 37, 38, 39, 0, 0, 40, 0,
        // State 20
        0, 0, 3, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 35, 36, 0, 0, 0, 0, 0, 37, 38, 39, 0, 5, 40, 6,
        // State 21
        0, 0, 3, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 35, 36, 0, 0, 0, 0, 0, 37, 38, 39, 0, 5, 40, 6,
        // State 22
        0, 0, 3, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 35, 36, 0, 0, 0, 0, 0, 37, 38, 39, 0, 5, 40, 6,
        // State 23
        0, 0, 3, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 35, 36, 0, 0, 0, 0, 0, 37, 38, 39, 0, 5, 40, 6,
        // State 24
        0, 0, -25, -25, 0, 7, -25, 8, 0, -25, -25, -25, 0, -25, -25, 0, -25, -25, 0, 0, 0, 0, 0, 0, 0, -25, -25, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 25
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -24, 0, 0, 0, 0, 41, 0, 0, 0, 0, 0, 0, 0, -24, 0, 0, 0,
        // State 26
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 42, 0, 0, 0,
        // State 27
        0, 0, 10, 0, 0, 0, 0, 0, 0, 43, 0, 11, 0, 0, 0, 0, 12, 13, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 28
        0, 14, -10, -10, 15, -10, -10, -10, 16, -10, -10, -10, 0, -10, -10, 0, -10, -10, 0, 0, 0, 0, 0, 0, 0, -10, -10, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 29
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 30
        0, 0, -35, 0, 0, 0, 0, -35, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -35, 0, -35, -35, 0, 0, 0, 0, 0, -35, -35, -35, 0, -35, -35, -35,
        // State 31
        0, 0, -34, 0, 0, 0, 0, -34, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -34, 0, -34, -34, 0, 0, 0, 0, 0, -34, -34, -34, 0, -34, -34, -34,
        // State 32
        0, -57, -57, 0, -57, -57, 0, -57, -57, -57, 0, -57, 0, 0, 0, 17, -57, -57, -19, 0, 0, 0, 0, -19, 0, 0, 0, 0, 0, 0, 0, -19, 0, 0, 0,
        // State 33
        0, -32, -32, -32, -32, -32, -32, -32, -32, -32, -32, -32, 0, -32, -32, 0, -32, -32, 0, 0, 0, 0, 0, 0, 0, -32, -32, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 34
        0, -49, -49, -49, -49, -49, -49, -49, -49, -49, -49, -49, 0, -49, -49, -49, -49, -49, -49, 0, 0, 0, 0, -49, 0, -49, -49, 0, 0, 0, 0, -49, 0, 0, 0,
        // State 35
        0, -50, -50, -50, -50, -50, -50, -50, -50, -50, -50, -50, 0, -50, -50, -50, -50, -50, -50, 0, 0, 0, 0, -50, 0, -50, -50, 0, 0, 0, 0, -50, 0, 0, 0,
        // State 36
        0, -52, -52, -52, -52, -52, -52, -52, -52, -52, -52, -52, 0, -52, -52, -52, -52, -52, -52, 0, 0, 0, 0, -52, 0, -52, -52, 0, 0, 0, 0, -52, 0, 0, 0,
        // State 37
        0, -53, -53, -53, -53, -53, -53, -53, -53, -53, -53, -53, 0, -53, -53, -53, -53, -53, -53, 0, 0, 0, 0, -53, 0, -53, -53, 0, 0, 0, 0, -53, 0, 0, 0,
        // State 38
        0, -48, -48, -48, -48, -48, -48, -48, -48, -48, -48, -48, 0, -48, -48, -48, -48, -48, -48, 0, 0, 0, 0, -48, 0, -48, -48, 0, 0, 0, 0, -48, 0, 0, 0,
        // State 39
        0, -51, -51, -51, -51, -51, -51, -51, -51, -51, -51, -51, 0, -51, -51, -51, -51, -51, -51, 0, 0, 0, 0, -51, 0, -51, -51, 0, 0, 0, 0, -51, 0, 0, 0,
        // State 40
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 18, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 52, 0, 0, 0,
        // State 41
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 19, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 42
        0, 0, -45, 0, 0, 0, 0, -45, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -45, 0, -45, -45, 0, 0, 0, 0, 0, -45, -45, -45, 0, -45, -45, -45,
        // State 43
        0, 0, -36, 0, 0, 0, 0, -36, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -36, 0, -36, -36, 0, 0, 0, 0, 0, -36, -36, -36, 0, -36, -36, -36,
        // State 44
        0, 0, 0, 64, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 45
        0, -57, -57, -57, -57, -57, -57, -57, -57, -57, -57, -57, 0, -57, -57, 17, -57, -57, 0, 0, 0, 0, 0, 0, 0, -57, -57, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 46
        0, -56, -56, -56, -56, -56, -56, -56, -56, -56, -56, -56, 0, -56, -56, 0, -56, -56, 0, 0, 0, 0, 0, 0, 0, -56, -56, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 47
        0, 0, 0, 0, 0, 0, 0, 0, 0, 65, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 48
        0, 0, 0, 0, 0, 0, 0, 0, 0, 66, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 49
        0, 14, -8, -8, 15, -8, -8, -8, 16, -8, -8, -8, 0, -8, -8, 0, -8, -8, 0, 0, 0, 0, 0, 0, 0, -8, -8, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 50
        0, 14, -9, -9, 15, -9, -9, -9, 16, -9, -9, -9, 0, -9, -9, 0, -9, -9, 0, 0, 0, 0, 0, 0, 0, -9, -9, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 51
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 21, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 52
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 68, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 53
        0, 0, 0, -28, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -28, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 54
        0, 0, 0, -11, 0, 0, 71, 0, 0, 0, 0, 0, 0, 0, -11, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 55
        0, 0, 0, 72, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 56
        0, 0, 0, 0, 0, 0, 0, 0, 0, 73, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 57
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 74, 0, 0, 75, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 76, 77, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 58
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 78, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 59
        0, -31, -31, -31, -31, -31, -31, -31, -31, -31, -31, -31, 0, -31, -31, 0, -31, -31, 0, 0, 0, 0, 0, 0, 0, -31, -31, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 60
        0, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, 0, -29, -29, 0, -29, -29, 0, 0, 0, 0, 0, 0, 0, -29, -29, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 61
        0, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, 0, -30, -30, 0, -30, -30, 0, 0, 0, 0, 0, 0, 0, -30, -30, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 62
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 79, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 63
        0, -55, -55, -55, -55, -55, -55, -55, -55, -55, -55, -55, 0, -55, -55, -55, -55, -55, -55, 0, 0, 0, 0, -55, 0, -55, -55, 0, 0, 0, 0, -55, 0, 0, 0,
        // State 64
        0, 0, -44, 0, 0, 0, 0, -44, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -44, 0, -44, -44, 0, 0, 0, 0, 0, -44, -44, -44, 0, -44, -44, -44,
        // State 65
        0, 0, -38, 0, 0, 0, 0, -38, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -38, 0, -38, -38, 0, 0, 0, 0, 0, -38, -38, -38, 0, -38, -38, -38,
        // State 66
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 80, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 67
        0, 0, -41, 0, 0, 0, 0, -41, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -41, 82, -41, -41, 0, 0, 0, 0, 0, -41, -41, -41, 0, -41, -41, -41,
        // State 68
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 83, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 69
        0, 0, 0, -13, 0, 0, 84, 0, 0, 0, 0, 0, 0, 0, -13, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 70
        0, 0, -4, -4, 0, 0, 0, -4, 0, 0, 0, 0, 0, 0, -4, 0, 0, 0, 0, 0, 0, -4, -4, 0, 0, 0, 0, 0, -4, -4, -4, 0, 0, -4, 0,
        // State 71
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 22, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 72
        0, 0, -37, 0, 0, 0, 0, -37, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -37, 0, -37, -37, 0, 0, 0, 0, 0, -37, -37, -37, 0, -37, -37, -37,
        // State 73
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -18, 0, 0, 0, 0, -18, 0, 0, 0, 0, 0, 0, 0, -18, 0, 0, 0,
        // State 74
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -17, 0, 0, 0, 0, -17, 0, 0, 0, 0, 0, 0, 0, -17, 0, 0, 0,
        // State 75
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -22, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -22, 0, 0, 0,
        // State 76
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -23, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -23, 0, 0, 0,
        // State 77
        85, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 86, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 87, 0, 0, 88, 0, 0, 0, 0, 0, 0, 0,
        // State 78
        0, -54, -54, -54, -54, -54, -54, -54, -54, -54, -54, -54, 0, -54, -54, -54, -54, -54, -54, 0, 0, 0, 0, -54, 0, -54, -54, 0, 0, 0, 0, -54, 0, 0, 0,
        // State 79
        0, 0, -39, 0, 0, 0, 0, -39, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -39, 89, -39, -39, 0, 0, 0, 0, 0, -39, -39, -39, 0, -39, -39, -39,
        // State 80
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 90, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 81
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 23, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 82
        0, 0, -47, 0, 0, 0, 0, -47, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -47, 0, -47, -47, 0, 0, 0, 0, 0, -47, -47, -47, 0, -47, -47, -47,
        // State 83
        0, 0, -5, -5, 0, 0, 0, -5, 0, 0, 0, 0, 0, 0, -5, 0, 0, 0, 0, 0, 0, -5, -5, 0, 0, 0, 0, 0, -5, -5, -5, 0, 0, -5, 0,
        // State 84
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -16, 0, 0, 0, 0, -16, 0, 0, 0, 0, 0, 0, 0, -16, 0, 0, 0,
        // State 85
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -15, 0, 0, 0, 0, -15, 0, 0, 0, 0, 0, 0, 0, -15, 0, 0, 0,
        // State 86
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -20, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -20, 0, 0, 0,
        // State 87
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -21, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -21, 0, 0, 0,
        // State 88
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 24, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 89
        0, 0, -46, 0, 0, 0, 0, -46, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -46, 0, -46, -46, 0, 0, 0, 0, 0, -46, -46, -46, 0, -46, -46, -46,
        // State 90
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 93, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 91
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 95, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 92
        0, 0, -43, 0, 0, 0, 0, -43, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -43, 0, -43, -43, 0, 0, 0, 0, 0, -43, -43, -43, 0, -43, -43, -43,
        // State 93
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 96, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 94
        0, 0, -42, 0, 0, 0, 0, -42, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -42, 0, -42, -42, 0, 0, 0, 0, 0, -42, -42, -42, 0, -42, -42, -42,
        // State 95
        0, 0, -40, 0, 0, 0, 0, -40, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -40, 0, -40, -40, 0, 0, 0, 0, 0, -40, -40, -40, 0, -40, -40, -40,
    ];
    fn __action(state: i8, integer: usize) -> i8 {
        __ACTION[(state as usize) * 35 + integer]
    }
    const __EOF_ACTION: &[i8] = &[
        // State 0
        0,
        // State 1
        -33,
        // State 2
        0,
        // State 3
        0,
        // State 4
        0,
        // State 5
        0,
        // State 6
        0,
        // State 7
        0,
        // State 8
        0,
        // State 9
        0,
        // State 10
        0,
        // State 11
        0,
        // State 12
        0,
        // State 13
        0,
        // State 14
        0,
        // State 15
        0,
        // State 16
        0,
        // State 17
        0,
        // State 18
        0,
        // State 19
        0,
        // State 20
        0,
        // State 21
        0,
        // State 22
        0,
        // State 23
        0,
        // State 24
        0,
        // State 25
        0,
        // State 26
        0,
        // State 27
        0,
        // State 28
        0,
        // State 29
        -58,
        // State 30
        -35,
        // State 31
        -34,
        // State 32
        0,
        // State 33
        0,
        // State 34
        0,
        // State 35
        0,
        // State 36
        0,
        // State 37
        0,
        // State 38
        0,
        // State 39
        0,
        // State 40
        0,
        // State 41
        0,
        // State 42
        -45,
        // State 43
        -36,
        // State 44
        0,
        // State 45
        0,
        // State 46
        0,
        // State 47
        0,
        // State 48
        0,
        // State 49
        0,
        // State 50
        0,
        // State 51
        0,
        // State 52
        0,
        // State 53
        0,
        // State 54
        0,
        // State 55
        0,
        // State 56
        0,
        // State 57
        0,
        // State 58
        0,
        // State 59
        0,
        // State 60
        0,
        // State 61
        0,
        // State 62
        0,
        // State 63
        0,
        // State 64
        -44,
        // State 65
        -38,
        // State 66
        0,
        // State 67
        -41,
        // State 68
        0,
        // State 69
        0,
        // State 70
        0,
        // State 71
        0,
        // State 72
        -37,
        // State 73
        0,
        // State 74
        0,
        // State 75
        0,
        // State 76
        0,
        // State 77
        0,
        // State 78
        0,
        // State 79
        -39,
        // State 80
        0,
        // State 81
        0,
        // State 82
        -47,
        // State 83
        0,
        // State 84
        0,
        // State 85
        0,
        // State 86
        0,
        // State 87
        0,
        // State 88
        0,
        // State 89
        -46,
        // State 90
        0,
        // State 91
        0,
        // State 92
        -43,
        // State 93
        0,
        // State 94
        -42,
        // State 95
        -40,
    ];
    fn __goto(state: i8, nt: usize) -> i8 {
        match nt {
            2 => 19,
            5 => 24,
            6 => 53,
            7 => 25,
            8 => 26,
            9 => match state {
                2 => 44,
                4 => 47,
                5 => 48,
                9 | 16 => 54,
                10 => 56,
                11 => 57,
                12 => 58,
                19 => 69,
                _ => 27,
            },
            11 => match state {
                16 => 62,
                _ => 55,
            },
            12 => match state {
                6 => 49,
                7 => 50,
                _ => 28,
            },
            13 => match state {
                8 => 52,
                17 => 66,
                18 => 68,
                20 => 80,
                21 => 90,
                22 => 91,
                23 => 93,
                _ => 29,
            },
            14 => match state {
                1 => 43,
                _ => 30,
            },
            15 => 1,
            16 => 31,
            17 => match state {
                2..=7 | 9..=16 | 19 => 45,
                _ => 32,
            },
            18 => match state {
                3 => 46,
                13 => 59,
                14 => 60,
                15 => 61,
                _ => 33,
            },
            _ => 0,
        }
    }
    const __TERMINAL: &[&str] = &[
        r###""!=""###,
        r###""%""###,
        r###""(""###,
        r###"")""###,
        r###""*""###,
        r###""+""###,
        r###"",""###,
        r###""-""###,
        r###""/""###,
        r###"";""###,
        r###""<""###,
        r###""=""###,
        r###""==""###,
        r###"">""###,
        r###""Close""###,
        r###""Open""###,
        r###""ne_kal""###,
        r###""um""###,
        r###""{""###,
        r###""}""###,
        r###"Else"###,
        r###"Float"###,
        r###"Identifier"###,
        r###"If"###,
        r###"IfEqualTo"###,
        r###"IfGreaterThan"###,
        r###"IfLessThan"###,
        r###"IfNotEqual"###,
        r###"InputNumber"###,
        r###"InputString"###,
        r###"Integer"###,
        r###"Loop"###,
        r###"Return"###,
        r###"StringLiteral"###,
        r###"Write"###,
    ];
    fn __expected_tokens(__state: i8) -> alloc::vec::Vec<alloc::string::String> {
        __TERMINAL.iter().enumerate().filter_map(|(index, terminal)| {
            let next_state = __action(__state, index);
            if next_state == 0 {
                None
            } else {
                Some(alloc::string::ToString::to_string(terminal))
            }
        }).collect()
    }
    fn __expected_tokens_from_states<
        '__0,
    >(
        __states: &[i8],
        _: core::marker::PhantomData<()>,
    ) -> alloc::vec::Vec<alloc::string::String>
    {
        __TERMINAL.iter().enumerate().filter_map(|(index, terminal)| {
            if __accepts(None, __states, Some(index), core::marker::PhantomData::<()>) {
                Some(alloc::string::ToString::to_string(terminal))
            } else {
                None
            }
        }).collect()
    }
    pub(crate) struct __StateMachine<'__0>
    where 
    {
        input: &'__0 str,
        __phantom: core::marker::PhantomData<()>,
    }
    impl<'__0> __state_machine::ParserDefinition for __StateMachine<'__0>
    where 
    {
        type Location = usize;
        type Error = LexicalError;
        type Token = TokenType;
        type TokenIndex = usize;
        type Symbol = __Symbol<>;
        type Success = SourceUnit;
        type StateIndex = i8;
        type Action = i8;
        type ReduceIndex = i8;
        type NonterminalIndex = usize;

        #[inline]
        fn start_location(&self) -> Self::Location {
              Default::default()
        }

        #[inline]
        fn start_state(&self) -> Self::StateIndex {
              0
        }

        #[inline]
        fn token_to_index(&self, token: &Self::Token) -> Option<usize> {
            __token_to_integer(token, core::marker::PhantomData::<()>)
        }

        #[inline]
        fn action(&self, state: i8, integer: usize) -> i8 {
            __action(state, integer)
        }

        #[inline]
        fn error_action(&self, state: i8) -> i8 {
            __action(state, 35 - 1)
        }

        #[inline]
        fn eof_action(&self, state: i8) -> i8 {
            __EOF_ACTION[state as usize]
        }

        #[inline]
        fn goto(&self, state: i8, nt: usize) -> i8 {
            __goto(state, nt)
        }

        fn token_to_symbol(&self, token_index: usize, token: Self::Token) -> Self::Symbol {
            __token_to_symbol(token_index, token, core::marker::PhantomData::<()>)
        }

        fn expected_tokens(&self, state: i8) -> alloc::vec::Vec<alloc::string::String> {
            __expected_tokens(state)
        }

        fn expected_tokens_from_states(&self, states: &[i8]) -> alloc::vec::Vec<alloc::string::String> {
            __expected_tokens_from_states(states, core::marker::PhantomData::<()>)
        }

        #[inline]
        fn uses_error_recovery(&self) -> bool {
            false
        }

        #[inline]
        fn error_recovery_symbol(
            &self,
            recovery: __state_machine::ErrorRecovery<Self>,
        ) -> Self::Symbol {
            panic!("error recovery not enabled for this grammar")
        }

        fn reduce(
            &mut self,
            action: i8,
            start_location: Option<&Self::Location>,
            states: &mut alloc::vec::Vec<i8>,
            symbols: &mut alloc::vec::Vec<__state_machine::SymbolTriple<Self>>,
        ) -> Option<__state_machine::ParseResult<Self>> {
            __reduce(
                self.input,
                action,
                start_location,
                states,
                symbols,
                core::marker::PhantomData::<()>,
            )
        }

        fn simulate_reduce(&self, action: i8) -> __state_machine::SimulatedReduce<Self> {
            __simulate_reduce(action, core::marker::PhantomData::<()>)
        }
    }
    fn __token_to_integer<
    >(
        __token: &TokenType,
        _: core::marker::PhantomData<()>,
    ) -> Option<usize>
    {
        match *__token {
            TokenType::NotEqual if true => Some(0),
            TokenType::Modulo if true => Some(1),
            TokenType::OpenParantheses if true => Some(2),
            TokenType::CloseParantheses if true => Some(3),
            TokenType::Product if true => Some(4),
            TokenType::Plus if true => Some(5),
            TokenType::Comma if true => Some(6),
            TokenType::Minus if true => Some(7),
            TokenType::Divide if true => Some(8),
            TokenType::SemiColon if true => Some(9),
            TokenType::LessThan if true => Some(10),
            TokenType::Assignment if true => Some(11),
            TokenType::EqualTo if true => Some(12),
            TokenType::GreaterThan if true => Some(13),
            TokenType::AngleClose if true => Some(14),
            TokenType::AngleOpen if true => Some(15),
            TokenType::Nekal if true => Some(16),
            TokenType::Um if true => Some(17),
            TokenType::LeftBrace if true => Some(18),
            TokenType::RightBrace if true => Some(19),
            TokenType::Else if true => Some(20),
            TokenType::Float(f64) if true => Some(21),
            TokenType::Symbol(usize) if true => Some(22),
            TokenType::If if true => Some(23),
            TokenType::IfEqualTo if true => Some(24),
            TokenType::IfGreaterThan if true => Some(25),
            TokenType::IfLessThan if true => Some(26),
            TokenType::IfNotEqual if true => Some(27),
            TokenType::InputNumber if true => Some(28),
            TokenType::InputString if true => Some(29),
            TokenType::Integer(i64) if true => Some(30),
            TokenType::Loop if true => Some(31),
            TokenType::Return if true => Some(32),
            TokenType::Literal(usize) if true => Some(33),
            TokenType::Write if true => Some(34),
            _ => None,
        }
    }
    fn __token_to_symbol<
    >(
        __token_index: usize,
        __token: TokenType,
        _: core::marker::PhantomData<()>,
    ) -> __Symbol<>
    {
        match __token_index {
            0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19 | 20 | 21 | 22 | 23 | 24 | 25 | 26 | 27 | 28 | 29 | 30 | 31 | 32 | 33 | 34 => __Symbol::Variant0(__token),
            _ => unreachable!(),
        }
    }
    fn __simulate_reduce<
        '__0,
    >(
        __reduce_index: i8,
        _: core::marker::PhantomData<()>,
    ) -> __state_machine::SimulatedReduce<__StateMachine<'__0>>
    {
        match __reduce_index {
            0 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 0,
                }
            }
            1 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 1,
                }
            }
            2 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 1,
                }
            }
            3 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 2,
                }
            }
            4 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 2,
                }
            }
            5 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 3,
                }
            }
            6 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 4,
                }
            }
            7 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 5,
                }
            }
            8 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 5,
                }
            }
            9 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 5,
                }
            }
            10 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 6,
                }
            }
            11 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 6,
                }
            }
            12 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 6,
                }
            }
            13 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 6,
                }
            }
            14 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 5,
                    nonterminal_produced: 7,
                }
            }
            15 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 5,
                    nonterminal_produced: 7,
                }
            }
            16 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 4,
                    nonterminal_produced: 7,
                }
            }
            17 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 4,
                    nonterminal_produced: 7,
                }
            }
            18 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 7,
                }
            }
            19 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 5,
                    nonterminal_produced: 8,
                }
            }
            20 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 5,
                    nonterminal_produced: 8,
                }
            }
            21 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 4,
                    nonterminal_produced: 8,
                }
            }
            22 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 4,
                    nonterminal_produced: 8,
                }
            }
            23 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 8,
                }
            }
            24 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 9,
                }
            }
            25 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 10,
                }
            }
            26 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 10,
                }
            }
            27 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 11,
                }
            }
            28 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 12,
                }
            }
            29 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 12,
                }
            }
            30 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 12,
                }
            }
            31 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 12,
                }
            }
            32 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 13,
                }
            }
            33 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 14,
                }
            }
            34 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 15,
                }
            }
            35 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 15,
                }
            }
            36 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 4,
                    nonterminal_produced: 16,
                }
            }
            37 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 16,
                }
            }
            38 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 5,
                    nonterminal_produced: 16,
                }
            }
            39 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 9,
                    nonterminal_produced: 16,
                }
            }
            40 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 4,
                    nonterminal_produced: 16,
                }
            }
            41 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 8,
                    nonterminal_produced: 16,
                }
            }
            42 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 7,
                    nonterminal_produced: 16,
                }
            }
            43 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 16,
                }
            }
            44 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 16,
                }
            }
            45 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 6,
                    nonterminal_produced: 16,
                }
            }
            46 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 5,
                    nonterminal_produced: 16,
                }
            }
            47 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 17,
                }
            }
            48 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 17,
                }
            }
            49 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 17,
                }
            }
            50 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 17,
                }
            }
            51 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 17,
                }
            }
            52 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 17,
                }
            }
            53 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 4,
                    nonterminal_produced: 17,
                }
            }
            54 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 17,
                }
            }
            55 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 18,
                }
            }
            56 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 18,
                }
            }
            57 => __state_machine::SimulatedReduce::Accept,
            _ => panic!("invalid reduction index {}", __reduce_index)
        }
    }
    pub struct SourceUnitParser {
        _priv: (),
    }

    impl SourceUnitParser {
        pub fn new() -> SourceUnitParser {
            SourceUnitParser {
                _priv: (),
            }
        }

        #[allow(dead_code)]
        pub fn parse<
            __TOKEN: __ToTriple<>,
            __TOKENS: IntoIterator<Item=__TOKEN>,
        >(
            &self,
            input: &str,
            __tokens0: __TOKENS,
        ) -> Result<SourceUnit, __lalrpop_util::ParseError<usize, TokenType, LexicalError>>
        {
            let __tokens = __tokens0.into_iter();
            let mut __tokens = __tokens.map(|t| __ToTriple::to_triple(t));
            __state_machine::Parser::drive(
                __StateMachine {
                    input,
                    __phantom: core::marker::PhantomData::<()>,
                },
                __tokens,
            )
        }
    }
    fn __accepts<
        '__0,
    >(
        __error_state: Option<i8>,
        __states: &[i8],
        __opt_integer: Option<usize>,
        _: core::marker::PhantomData<()>,
    ) -> bool
    {
        let mut __states = __states.to_vec();
        __states.extend(__error_state);
        loop {
            let mut __states_len = __states.len();
            let __top = __states[__states_len - 1];
            let __action = match __opt_integer {
                None => __EOF_ACTION[__top as usize],
                Some(__integer) => __action(__top, __integer),
            };
            if __action == 0 { return false; }
            if __action > 0 { return true; }
            let (__to_pop, __nt) = match __simulate_reduce(-(__action + 1), core::marker::PhantomData::<()>) {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop, nonterminal_produced
                } => (states_to_pop, nonterminal_produced),
                __state_machine::SimulatedReduce::Accept => return true,
            };
            __states_len -= __to_pop;
            __states.truncate(__states_len);
            let __top = __states[__states_len - 1];
            let __next_state = __goto(__top, __nt);
            __states.push(__next_state);
        }
    }
    pub(crate) fn __reduce<
    >(
        input: &str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut alloc::vec::Vec<i8>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> Option<Result<SourceUnit,__lalrpop_util::ParseError<usize, TokenType, LexicalError>>>
    {
        let (__pop_states, __nonterminal) = match __action {
            0 => {
                __reduce0(input, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            1 => {
                __reduce1(input, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            2 => {
                __reduce2(input, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            3 => {
                __reduce3(input, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            4 => {
                __reduce4(input, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            5 => {
                __reduce5(input, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            6 => {
                __reduce6(input, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            7 => {
                __reduce7(input, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            8 => {
                __reduce8(input, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            9 => {
                __reduce9(input, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            10 => {
                __reduce10(input, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            11 => {
                __reduce11(input, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            12 => {
                __reduce12(input, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            13 => {
                __reduce13(input, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            14 => {
                __reduce14(input, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            15 => {
                __reduce15(input, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            16 => {
                __reduce16(input, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            17 => {
                __reduce17(input, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            18 => {
                __reduce18(input, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            19 => {
                __reduce19(input, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            20 => {
                __reduce20(input, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            21 => {
                __reduce21(input, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            22 => {
                __reduce22(input, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            23 => {
                __reduce23(input, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            24 => {
                __reduce24(input, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            25 => {
                __reduce25(input, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            26 => {
                __reduce26(input, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            27 => {
                __reduce27(input, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            28 => {
                __reduce28(input, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            29 => {
                __reduce29(input, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            30 => {
                __reduce30(input, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            31 => {
                __reduce31(input, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            32 => {
                __reduce32(input, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            33 => {
                __reduce33(input, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            34 => {
                __reduce34(input, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            35 => {
                __reduce35(input, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            36 => {
                __reduce36(input, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            37 => {
                __reduce37(input, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            38 => {
                __reduce38(input, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            39 => {
                __reduce39(input, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            40 => {
                __reduce40(input, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            41 => {
                __reduce41(input, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            42 => {
                __reduce42(input, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            43 => {
                __reduce43(input, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            44 => {
                __reduce44(input, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            45 => {
                __reduce45(input, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            46 => {
                __reduce46(input, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            47 => {
                __reduce47(input, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            48 => {
                __reduce48(input, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            49 => {
                __reduce49(input, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            50 => {
                __reduce50(input, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            51 => {
                __reduce51(input, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            52 => {
                __reduce52(input, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            53 => {
                __reduce53(input, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            54 => {
                __reduce54(input, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            55 => {
                __reduce55(input, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            56 => {
                __reduce56(input, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            57 => {
                // __SourceUnit = SourceUnit => ActionFn(0);
                let __sym0 = __pop_Variant6(__symbols);
                let __start = __sym0.0;
                let __end = __sym0.2;
                let __nt = super::__action0::<>(input, __sym0);
                return Some(Ok(__nt));
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __states_len = __states.len();
        __states.truncate(__states_len - __pop_states);
        let __state = *__states.last().unwrap();
        let __next_state = __goto(__state, __nonterminal);
        __states.push(__next_state);
        None
    }
    #[inline(never)]
    fn __symbol_type_mismatch() -> ! {
        panic!("symbol type mismatch")
    }
    fn __pop_Variant1<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Expression, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant1(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant6<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, SourceUnit, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant6(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant7<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, SourceUnitPart, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant7(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant9<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Statement, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant9(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant0<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, TokenType, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant0(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant4<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Vec<Expression>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant4(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant2<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, alloc::vec::Vec<Expression>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant2(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant8<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, alloc::vec::Vec<SourceUnitPart>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant8(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant5<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, core::option::Option<Expression>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant5(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant3<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, usize, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant3(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    pub(crate) fn __reduce0<
    >(
        input: &str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // (<Expression> ",") = Expression, "," => ActionFn(52);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action52::<>(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (2, 0)
    }
    pub(crate) fn __reduce1<
    >(
        input: &str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // (<Expression> ",")* =  => ActionFn(50);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action50::<>(input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (0, 1)
    }
    pub(crate) fn __reduce2<
    >(
        input: &str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // (<Expression> ",")* = (<Expression> ",")+ => ActionFn(51);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action51::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (1, 1)
    }
    pub(crate) fn __reduce3<
    >(
        input: &str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // (<Expression> ",")+ = Expression, "," => ActionFn(55);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action55::<>(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (2, 2)
    }
    pub(crate) fn __reduce4<
    >(
        input: &str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // (<Expression> ",")+ = (<Expression> ",")+, Expression, "," => ActionFn(56);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action56::<>(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (3, 2)
    }
    pub(crate) fn __reduce5<
    >(
        input: &str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // @L =  => ActionFn(44);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action44::<>(input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (0, 3)
    }
    pub(crate) fn __reduce6<
    >(
        input: &str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // @R =  => ActionFn(43);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action43::<>(input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (0, 4)
    }
    pub(crate) fn __reduce7<
    >(
        input: &str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ArithExpression = ArithExpression, "+", Factor => ActionFn(91);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant1(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action91::<>(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (3, 5)
    }
    pub(crate) fn __reduce8<
    >(
        input: &str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ArithExpression = ArithExpression, "-", Factor => ActionFn(92);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant1(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action92::<>(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (3, 5)
    }
    pub(crate) fn __reduce9<
    >(
        input: &str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ArithExpression = Factor => ActionFn(28);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action28::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 5)
    }
    pub(crate) fn __reduce10<
    >(
        input: &str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Comma<Expression> = Expression => ActionFn(123);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action123::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 6)
    }
    pub(crate) fn __reduce11<
    >(
        input: &str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Comma<Expression> =  => ActionFn(124);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action124::<>(input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (0, 6)
    }
    pub(crate) fn __reduce12<
    >(
        input: &str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Comma<Expression> = (<Expression> ",")+, Expression => ActionFn(125);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action125::<>(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (2, 6)
    }
    pub(crate) fn __reduce13<
    >(
        input: &str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Comma<Expression> = (<Expression> ",")+ => ActionFn(126);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action126::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 6)
    }
    pub(crate) fn __reduce14<
    >(
        input: &str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Conditional = Expression, "um", Expression, "um", "==" => ActionFn(93);
        assert!(__symbols.len() >= 5);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant1(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym4.2;
        let __nt = super::__action93::<>(input, __sym0, __sym1, __sym2, __sym3, __sym4);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (5, 7)
    }
    pub(crate) fn __reduce15<
    >(
        input: &str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Conditional = Expression, "um", Expression, "um", "!=" => ActionFn(94);
        assert!(__symbols.len() >= 5);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant1(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym4.2;
        let __nt = super::__action94::<>(input, __sym0, __sym1, __sym2, __sym3, __sym4);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (5, 7)
    }
    pub(crate) fn __reduce16<
    >(
        input: &str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Conditional = Expression, "ne_kal", Expression, ">" => ActionFn(95);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant1(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym3.2;
        let __nt = super::__action95::<>(input, __sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (4, 7)
    }
    pub(crate) fn __reduce17<
    >(
        input: &str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Conditional = Expression, "ne_kal", Expression, "<" => ActionFn(96);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant1(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym3.2;
        let __nt = super::__action96::<>(input, __sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (4, 7)
    }
    pub(crate) fn __reduce18<
    >(
        input: &str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Conditional = Term => ActionFn(25);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action25::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 7)
    }
    pub(crate) fn __reduce19<
    >(
        input: &str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ConditionalWithAgglutination = Expression, "um", Expression, "um", IfEqualTo => ActionFn(97);
        assert!(__symbols.len() >= 5);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant1(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym4.2;
        let __nt = super::__action97::<>(input, __sym0, __sym1, __sym2, __sym3, __sym4);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (5, 8)
    }
    pub(crate) fn __reduce20<
    >(
        input: &str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ConditionalWithAgglutination = Expression, "um", Expression, "um", IfNotEqual => ActionFn(98);
        assert!(__symbols.len() >= 5);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant1(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym4.2;
        let __nt = super::__action98::<>(input, __sym0, __sym1, __sym2, __sym3, __sym4);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (5, 8)
    }
    pub(crate) fn __reduce21<
    >(
        input: &str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ConditionalWithAgglutination = Expression, "ne_kal", Expression, IfGreaterThan => ActionFn(99);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant1(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym3.2;
        let __nt = super::__action99::<>(input, __sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (4, 8)
    }
    pub(crate) fn __reduce22<
    >(
        input: &str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ConditionalWithAgglutination = Expression, "ne_kal", Expression, IfLessThan => ActionFn(100);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant1(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym3.2;
        let __nt = super::__action100::<>(input, __sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (4, 8)
    }
    pub(crate) fn __reduce23<
    >(
        input: &str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ConditionalWithAgglutination = Conditional => ActionFn(20);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action20::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 8)
    }
    pub(crate) fn __reduce24<
    >(
        input: &str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expression = ArithExpression => ActionFn(15);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action15::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 9)
    }
    pub(crate) fn __reduce25<
    >(
        input: &str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expression? = Expression => ActionFn(48);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action48::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 10)
    }
    pub(crate) fn __reduce26<
    >(
        input: &str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expression? =  => ActionFn(49);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action49::<>(input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (0, 10)
    }
    pub(crate) fn __reduce27<
    >(
        input: &str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expressions = Comma<Expression> => ActionFn(3);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action3::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 11)
    }
    pub(crate) fn __reduce28<
    >(
        input: &str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Factor = Factor, "*", Unary => ActionFn(101);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant1(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action101::<>(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (3, 12)
    }
    pub(crate) fn __reduce29<
    >(
        input: &str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Factor = Factor, "/", Unary => ActionFn(102);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant1(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action102::<>(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (3, 12)
    }
    pub(crate) fn __reduce30<
    >(
        input: &str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Factor = Factor, "%", Unary => ActionFn(103);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant1(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action103::<>(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (3, 12)
    }
    pub(crate) fn __reduce31<
    >(
        input: &str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Factor = Unary => ActionFn(32);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action32::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 12)
    }
    pub(crate) fn __reduce32<
    >(
        input: &str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // SourceUnit = SourceUnitPart+ => ActionFn(1);
        let __sym0 = __pop_Variant8(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action1::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 13)
    }
    pub(crate) fn __reduce33<
    >(
        input: &str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // SourceUnitPart = Statement => ActionFn(2);
        let __sym0 = __pop_Variant9(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action2::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 14)
    }
    pub(crate) fn __reduce34<
    >(
        input: &str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // SourceUnitPart+ = SourceUnitPart => ActionFn(46);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action46::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (1, 15)
    }
    pub(crate) fn __reduce35<
    >(
        input: &str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // SourceUnitPart+ = SourceUnitPart+, SourceUnitPart => ActionFn(47);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant7(__symbols);
        let __sym0 = __pop_Variant8(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action47::<>(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (2, 15)
    }
    pub(crate) fn __reduce36<
    >(
        input: &str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Statement = Expression, "=", Expression, ";" => ActionFn(104);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant1(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym3.2;
        let __nt = super::__action104::<>(input, __sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (4, 16)
    }
    pub(crate) fn __reduce37<
    >(
        input: &str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Statement = Write, Expression, ";" => ActionFn(105);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action105::<>(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (3, 16)
    }
    pub(crate) fn __reduce38<
    >(
        input: &str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Statement = Conditional, If, "{", SourceUnit, "}" => ActionFn(106);
        assert!(__symbols.len() >= 5);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant6(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym4.2;
        let __nt = super::__action106::<>(input, __sym0, __sym1, __sym2, __sym3, __sym4);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (5, 16)
    }
    pub(crate) fn __reduce39<
    >(
        input: &str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Statement = Conditional, If, "{", SourceUnit, "}", Else, "{", SourceUnit, "}" => ActionFn(107);
        assert!(__symbols.len() >= 9);
        let __sym8 = __pop_Variant0(__symbols);
        let __sym7 = __pop_Variant6(__symbols);
        let __sym6 = __pop_Variant0(__symbols);
        let __sym5 = __pop_Variant0(__symbols);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant6(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym8.2;
        let __nt = super::__action107::<>(input, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5, __sym6, __sym7, __sym8);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (9, 16)
    }
    pub(crate) fn __reduce40<
    >(
        input: &str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Statement = ConditionalWithAgglutination, "{", SourceUnit, "}" => ActionFn(108);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant6(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym3.2;
        let __nt = super::__action108::<>(input, __sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (4, 16)
    }
    pub(crate) fn __reduce41<
    >(
        input: &str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Statement = ConditionalWithAgglutination, "{", SourceUnit, "}", Else, "{", SourceUnit, "}" => ActionFn(109);
        assert!(__symbols.len() >= 8);
        let __sym7 = __pop_Variant0(__symbols);
        let __sym6 = __pop_Variant6(__symbols);
        let __sym5 = __pop_Variant0(__symbols);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant6(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym7.2;
        let __nt = super::__action109::<>(input, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5, __sym6, __sym7);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (8, 16)
    }
    pub(crate) fn __reduce42<
    >(
        input: &str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Statement = Expression, "(", Expressions, ")", "{", SourceUnit, "}" => ActionFn(110);
        assert!(__symbols.len() >= 7);
        let __sym6 = __pop_Variant0(__symbols);
        let __sym5 = __pop_Variant6(__symbols);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant4(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym6.2;
        let __nt = super::__action110::<>(input, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5, __sym6);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (7, 16)
    }
    pub(crate) fn __reduce43<
    >(
        input: &str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Statement = Return, Expression, ";" => ActionFn(111);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action111::<>(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (3, 16)
    }
    pub(crate) fn __reduce44<
    >(
        input: &str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Statement = Expression, ";" => ActionFn(112);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action112::<>(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (2, 16)
    }
    pub(crate) fn __reduce45<
    >(
        input: &str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Statement = Conditional, If, Loop, "{", SourceUnit, "}" => ActionFn(113);
        assert!(__symbols.len() >= 6);
        let __sym5 = __pop_Variant0(__symbols);
        let __sym4 = __pop_Variant6(__symbols);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym5.2;
        let __nt = super::__action113::<>(input, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (6, 16)
    }
    pub(crate) fn __reduce46<
    >(
        input: &str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Statement = ConditionalWithAgglutination, Loop, "{", SourceUnit, "}" => ActionFn(114);
        assert!(__symbols.len() >= 5);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant6(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym4.2;
        let __nt = super::__action114::<>(input, __sym0, __sym1, __sym2, __sym3, __sym4);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (5, 16)
    }
    pub(crate) fn __reduce47<
    >(
        input: &str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Term = Integer => ActionFn(115);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action115::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 17)
    }
    pub(crate) fn __reduce48<
    >(
        input: &str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Term = Float => ActionFn(116);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action116::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 17)
    }
    pub(crate) fn __reduce49<
    >(
        input: &str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Term = Identifier => ActionFn(117);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action117::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 17)
    }
    pub(crate) fn __reduce50<
    >(
        input: &str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Term = StringLiteral => ActionFn(118);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action118::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 17)
    }
    pub(crate) fn __reduce51<
    >(
        input: &str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Term = InputNumber => ActionFn(119);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action119::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 17)
    }
    pub(crate) fn __reduce52<
    >(
        input: &str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Term = InputString => ActionFn(120);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action120::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 17)
    }
    pub(crate) fn __reduce53<
    >(
        input: &str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Term = Term, "Open", Expressions, "Close" => ActionFn(121);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant4(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym3.2;
        let __nt = super::__action121::<>(input, __sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (4, 17)
    }
    pub(crate) fn __reduce54<
    >(
        input: &str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Term = "(", Expression, ")" => ActionFn(42);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action42::<>(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (3, 17)
    }
    pub(crate) fn __reduce55<
    >(
        input: &str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Unary = "-", Unary => ActionFn(122);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action122::<>(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (2, 18)
    }
    pub(crate) fn __reduce56<
    >(
        input: &str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Unary = Term => ActionFn(34);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action34::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 18)
    }
}
pub use self::__parse__SourceUnit::SourceUnitParser;

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action0<
>(
    input: &str,
    (_, __0, _): (usize, SourceUnit, usize),
) -> SourceUnit
{
    __0
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action1<
>(
    input: &str,
    (_, __0, _): (usize, alloc::vec::Vec<SourceUnitPart>, usize),
) -> SourceUnit
{
    SourceUnit(__0)
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action2<
>(
    input: &str,
    (_, __0, _): (usize, Statement, usize),
) -> SourceUnitPart
{
    SourceUnitPart::Statement(__0)
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action3<
>(
    input: &str,
    (_, __0, _): (usize, Vec<Expression>, usize),
) -> Vec<Expression>
{
    __0
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action4<
>(
    input: &str,
    (_, a, _): (usize, usize, usize),
    (_, l, _): (usize, Expression, usize),
    (_, _, _): (usize, TokenType, usize),
    (_, r, _): (usize, Expression, usize),
    (_, _, _): (usize, TokenType, usize),
    (_, b, _): (usize, usize, usize),
) -> Statement
{
    Statement::Assignment((a,b),l,r)
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action5<
>(
    input: &str,
    (_, a, _): (usize, usize, usize),
    (_, _, _): (usize, TokenType, usize),
    (_, e, _): (usize, Expression, usize),
    (_, _, _): (usize, TokenType, usize),
    (_, b, _): (usize, usize, usize),
) -> Statement
{
    Statement::Write((a,b),e)
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action6<
>(
    input: &str,
    (_, a, _): (usize, usize, usize),
    (_, condition, _): (usize, Expression, usize),
    (_, _, _): (usize, TokenType, usize),
    (_, _, _): (usize, TokenType, usize),
    (_, s, _): (usize, SourceUnit, usize),
    (_, _, _): (usize, TokenType, usize),
    (_, b, _): (usize, usize, usize),
) -> Statement
{
    Statement::Conditional((a,b),condition,s,None)
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action7<
>(
    input: &str,
    (_, a, _): (usize, usize, usize),
    (_, condition, _): (usize, Expression, usize),
    (_, _, _): (usize, TokenType, usize),
    (_, _, _): (usize, TokenType, usize),
    (_, s, _): (usize, SourceUnit, usize),
    (_, _, _): (usize, TokenType, usize),
    (_, _, _): (usize, TokenType, usize),
    (_, _, _): (usize, TokenType, usize),
    (_, f, _): (usize, SourceUnit, usize),
    (_, _, _): (usize, TokenType, usize),
    (_, b, _): (usize, usize, usize),
) -> Statement
{
    Statement::Conditional((a,b),condition,s,Some(f))
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action8<
>(
    input: &str,
    (_, a, _): (usize, usize, usize),
    (_, condition, _): (usize, Expression, usize),
    (_, _, _): (usize, TokenType, usize),
    (_, s, _): (usize, SourceUnit, usize),
    (_, _, _): (usize, TokenType, usize),
    (_, b, _): (usize, usize, usize),
) -> Statement
{
    Statement::Conditional((a,b),condition,s,None)
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action9<
>(
    input: &str,
    (_, a, _): (usize, usize, usize),
    (_, condition, _): (usize, Expression, usize),
    (_, _, _): (usize, TokenType, usize),
    (_, s, _): (usize, SourceUnit, usize),
    (_, _, _): (usize, TokenType, usize),
    (_, _, _): (usize, TokenType, usize),
    (_, _, _): (usize, TokenType, usize),
    (_, f, _): (usize, SourceUnit, usize),
    (_, _, _): (usize, TokenType, usize),
    (_, b, _): (usize, usize, usize),
) -> Statement
{
    Statement::Conditional((a,b),condition,s,Some(f))
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action10<
>(
    input: &str,
    (_, a, _): (usize, usize, usize),
    (_, name, _): (usize, Expression, usize),
    (_, _, _): (usize, TokenType, usize),
    (_, params, _): (usize, Vec<Expression>, usize),
    (_, _, _): (usize, TokenType, usize),
    (_, _, _): (usize, TokenType, usize),
    (_, s, _): (usize, SourceUnit, usize),
    (_, _, _): (usize, TokenType, usize),
    (_, b, _): (usize, usize, usize),
) -> Statement
{
    Statement::FunctionDeclaration((a,b),name,params,s)
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action11<
>(
    input: &str,
    (_, a, _): (usize, usize, usize),
    (_, _, _): (usize, TokenType, usize),
    (_, expr, _): (usize, Expression, usize),
    (_, _, _): (usize, TokenType, usize),
    (_, b, _): (usize, usize, usize),
) -> Statement
{
    Statement::Return((a,b),expr)
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action12<
>(
    input: &str,
    (_, a, _): (usize, usize, usize),
    (_, expr, _): (usize, Expression, usize),
    (_, _, _): (usize, TokenType, usize),
    (_, b, _): (usize, usize, usize),
) -> Statement
{
    Statement::EmptyExpression((a,b),expr)
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action13<
>(
    input: &str,
    (_, a, _): (usize, usize, usize),
    (_, condition, _): (usize, Expression, usize),
    (_, _, _): (usize, TokenType, usize),
    (_, _, _): (usize, TokenType, usize),
    (_, _, _): (usize, TokenType, usize),
    (_, s, _): (usize, SourceUnit, usize),
    (_, _, _): (usize, TokenType, usize),
    (_, b, _): (usize, usize, usize),
) -> Statement
{
    Statement::Loop((a,b),condition,s)
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action14<
>(
    input: &str,
    (_, a, _): (usize, usize, usize),
    (_, condition, _): (usize, Expression, usize),
    (_, _, _): (usize, TokenType, usize),
    (_, _, _): (usize, TokenType, usize),
    (_, s, _): (usize, SourceUnit, usize),
    (_, _, _): (usize, TokenType, usize),
    (_, b, _): (usize, usize, usize),
) -> Statement
{
    Statement::Loop((a,b),condition,s)
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action15<
>(
    input: &str,
    (_, __0, _): (usize, Expression, usize),
) -> Expression
{
    __0
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action16<
>(
    input: &str,
    (_, l, _): (usize, Expression, usize),
    (_, _, _): (usize, TokenType, usize),
    (_, r, _): (usize, Expression, usize),
    (_, _, _): (usize, TokenType, usize),
    (_, a, _): (usize, usize, usize),
    (_, _, _): (usize, TokenType, usize),
    (_, b, _): (usize, usize, usize),
) -> Expression
{
    Expression::Equals((a,b),Box::new(l),Box::new(r))
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action17<
>(
    input: &str,
    (_, l, _): (usize, Expression, usize),
    (_, _, _): (usize, TokenType, usize),
    (_, r, _): (usize, Expression, usize),
    (_, _, _): (usize, TokenType, usize),
    (_, a, _): (usize, usize, usize),
    (_, _, _): (usize, TokenType, usize),
    (_, b, _): (usize, usize, usize),
) -> Expression
{
    Expression::NotEquals((a,b),Box::new(l),Box::new(r))
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action18<
>(
    input: &str,
    (_, r, _): (usize, Expression, usize),
    (_, _, _): (usize, TokenType, usize),
    (_, l, _): (usize, Expression, usize),
    (_, a, _): (usize, usize, usize),
    (_, _, _): (usize, TokenType, usize),
    (_, b, _): (usize, usize, usize),
) -> Expression
{
    Expression::GreaterThan((a,b),Box::new(l),Box::new(r))
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action19<
>(
    input: &str,
    (_, r, _): (usize, Expression, usize),
    (_, _, _): (usize, TokenType, usize),
    (_, l, _): (usize, Expression, usize),
    (_, a, _): (usize, usize, usize),
    (_, _, _): (usize, TokenType, usize),
    (_, b, _): (usize, usize, usize),
) -> Expression
{
    Expression::LessThan((a,b),Box::new(l),Box::new(r))
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action20<
>(
    input: &str,
    (_, __0, _): (usize, Expression, usize),
) -> Expression
{
    __0
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action21<
>(
    input: &str,
    (_, l, _): (usize, Expression, usize),
    (_, _, _): (usize, TokenType, usize),
    (_, r, _): (usize, Expression, usize),
    (_, _, _): (usize, TokenType, usize),
    (_, a, _): (usize, usize, usize),
    (_, _, _): (usize, TokenType, usize),
    (_, b, _): (usize, usize, usize),
) -> Expression
{
    Expression::Equals((a,b),Box::new(l),Box::new(r))
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action22<
>(
    input: &str,
    (_, l, _): (usize, Expression, usize),
    (_, _, _): (usize, TokenType, usize),
    (_, r, _): (usize, Expression, usize),
    (_, _, _): (usize, TokenType, usize),
    (_, a, _): (usize, usize, usize),
    (_, _, _): (usize, TokenType, usize),
    (_, b, _): (usize, usize, usize),
) -> Expression
{
    Expression::NotEquals((a,b),Box::new(l),Box::new(r))
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action23<
>(
    input: &str,
    (_, r, _): (usize, Expression, usize),
    (_, _, _): (usize, TokenType, usize),
    (_, l, _): (usize, Expression, usize),
    (_, a, _): (usize, usize, usize),
    (_, _, _): (usize, TokenType, usize),
    (_, b, _): (usize, usize, usize),
) -> Expression
{
    Expression::GreaterThan((a,b),Box::new(l),Box::new(r))
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action24<
>(
    input: &str,
    (_, r, _): (usize, Expression, usize),
    (_, _, _): (usize, TokenType, usize),
    (_, l, _): (usize, Expression, usize),
    (_, a, _): (usize, usize, usize),
    (_, _, _): (usize, TokenType, usize),
    (_, b, _): (usize, usize, usize),
) -> Expression
{
    Expression::LessThan((a,b),Box::new(l),Box::new(r))
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action25<
>(
    input: &str,
    (_, __0, _): (usize, Expression, usize),
) -> Expression
{
    __0
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action26<
>(
    input: &str,
    (_, l, _): (usize, Expression, usize),
    (_, a, _): (usize, usize, usize),
    (_, _, _): (usize, TokenType, usize),
    (_, b, _): (usize, usize, usize),
    (_, r, _): (usize, Expression, usize),
) -> Expression
{
    Expression::Add((a,b),Box::new(l),Box::new(r))
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action27<
>(
    input: &str,
    (_, l, _): (usize, Expression, usize),
    (_, a, _): (usize, usize, usize),
    (_, _, _): (usize, TokenType, usize),
    (_, b, _): (usize, usize, usize),
    (_, r, _): (usize, Expression, usize),
) -> Expression
{
    Expression::Subtract((a,b),Box::new(l),Box::new(r))
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action28<
>(
    input: &str,
    (_, __0, _): (usize, Expression, usize),
) -> Expression
{
    __0
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action29<
>(
    input: &str,
    (_, l, _): (usize, Expression, usize),
    (_, a, _): (usize, usize, usize),
    (_, _, _): (usize, TokenType, usize),
    (_, b, _): (usize, usize, usize),
    (_, r, _): (usize, Expression, usize),
) -> Expression
{
    Expression::Multiply((a,b),Box::new(l),Box::new(r))
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action30<
>(
    input: &str,
    (_, l, _): (usize, Expression, usize),
    (_, a, _): (usize, usize, usize),
    (_, _, _): (usize, TokenType, usize),
    (_, b, _): (usize, usize, usize),
    (_, r, _): (usize, Expression, usize),
) -> Expression
{
    Expression::Divide((a,b),Box::new(l),Box::new(r))
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action31<
>(
    input: &str,
    (_, l, _): (usize, Expression, usize),
    (_, a, _): (usize, usize, usize),
    (_, _, _): (usize, TokenType, usize),
    (_, b, _): (usize, usize, usize),
    (_, r, _): (usize, Expression, usize),
) -> Expression
{
    Expression::Modulo((a,b),Box::new(l),Box::new(r))
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action32<
>(
    input: &str,
    (_, __0, _): (usize, Expression, usize),
) -> Expression
{
    __0
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action33<
>(
    input: &str,
    (_, a, _): (usize, usize, usize),
    (_, _, _): (usize, TokenType, usize),
    (_, b, _): (usize, usize, usize),
    (_, r, _): (usize, Expression, usize),
) -> Expression
{
    Expression::UnaryMinus((a,b),Box::new(r))
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action34<
>(
    input: &str,
    (_, __0, _): (usize, Expression, usize),
) -> Expression
{
    __0
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action35<
>(
    input: &str,
    (_, a, _): (usize, usize, usize),
    (_, v, _): (usize, TokenType, usize),
    (_, b, _): (usize, usize, usize),
) -> Expression
{
    Expression::Integer((a,b),v)
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action36<
>(
    input: &str,
    (_, a, _): (usize, usize, usize),
    (_, v, _): (usize, TokenType, usize),
    (_, b, _): (usize, usize, usize),
) -> Expression
{
    Expression::Float((a,b),v)
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action37<
>(
    input: &str,
    (_, a, _): (usize, usize, usize),
    (_, id, _): (usize, TokenType, usize),
    (_, b, _): (usize, usize, usize),
) -> Expression
{
    Expression::Symbol((a,b),id)
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action38<
>(
    input: &str,
    (_, a, _): (usize, usize, usize),
    (_, string, _): (usize, TokenType, usize),
    (_, b, _): (usize, usize, usize),
) -> Expression
{
    Expression::StringLiteral((a,b),string)
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action39<
>(
    input: &str,
    (_, a, _): (usize, usize, usize),
    (_, d, _): (usize, TokenType, usize),
    (_, b, _): (usize, usize, usize),
) -> Expression
{
    Expression::InputNumber((a,b))
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action40<
>(
    input: &str,
    (_, a, _): (usize, usize, usize),
    (_, d, _): (usize, TokenType, usize),
    (_, b, _): (usize, usize, usize),
) -> Expression
{
    Expression::InputString((a,b))
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action41<
>(
    input: &str,
    (_, a, _): (usize, usize, usize),
    (_, id, _): (usize, Expression, usize),
    (_, _, _): (usize, TokenType, usize),
    (_, args, _): (usize, Vec<Expression>, usize),
    (_, _, _): (usize, TokenType, usize),
    (_, b, _): (usize, usize, usize),
) -> Expression
{
    Expression::FunctionCall((a,b),Box::new(id),args)
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action42<
>(
    input: &str,
    (_, _, _): (usize, TokenType, usize),
    (_, e, _): (usize, Expression, usize),
    (_, _, _): (usize, TokenType, usize),
) -> Expression
{
    e
}

#[allow(unused_variables)]
fn __action43<
>(
    input: &str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> usize
{
    *__lookbehind
}

#[allow(unused_variables)]
fn __action44<
>(
    input: &str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> usize
{
    *__lookahead
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action45<
>(
    input: &str,
    (_, v, _): (usize, alloc::vec::Vec<Expression>, usize),
    (_, e, _): (usize, core::option::Option<Expression>, usize),
) -> Vec<Expression>
{
    match e {
        None => v,
        Some(e) => {
            let mut v = v;
            v.push(e);
            v
        }
    }
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action46<
>(
    input: &str,
    (_, __0, _): (usize, SourceUnitPart, usize),
) -> alloc::vec::Vec<SourceUnitPart>
{
    alloc::vec![__0]
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action47<
>(
    input: &str,
    (_, v, _): (usize, alloc::vec::Vec<SourceUnitPart>, usize),
    (_, e, _): (usize, SourceUnitPart, usize),
) -> alloc::vec::Vec<SourceUnitPart>
{
    { let mut v = v; v.push(e); v }
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action48<
>(
    input: &str,
    (_, __0, _): (usize, Expression, usize),
) -> core::option::Option<Expression>
{
    Some(__0)
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action49<
>(
    input: &str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> core::option::Option<Expression>
{
    None
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action50<
>(
    input: &str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> alloc::vec::Vec<Expression>
{
    alloc::vec![]
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action51<
>(
    input: &str,
    (_, v, _): (usize, alloc::vec::Vec<Expression>, usize),
) -> alloc::vec::Vec<Expression>
{
    v
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action52<
>(
    input: &str,
    (_, __0, _): (usize, Expression, usize),
    (_, _, _): (usize, TokenType, usize),
) -> Expression
{
    __0
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action53<
>(
    input: &str,
    (_, __0, _): (usize, Expression, usize),
) -> alloc::vec::Vec<Expression>
{
    alloc::vec![__0]
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action54<
>(
    input: &str,
    (_, v, _): (usize, alloc::vec::Vec<Expression>, usize),
    (_, e, _): (usize, Expression, usize),
) -> alloc::vec::Vec<Expression>
{
    { let mut v = v; v.push(e); v }
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action55<
>(
    input: &str,
    __0: (usize, Expression, usize),
    __1: (usize, TokenType, usize),
) -> alloc::vec::Vec<Expression>
{
    let __start0 = __0.0;
    let __end0 = __1.2;
    let __temp0 = __action52(
        input,
        __0,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action53(
        input,
        __temp0,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action56<
>(
    input: &str,
    __0: (usize, alloc::vec::Vec<Expression>, usize),
    __1: (usize, Expression, usize),
    __2: (usize, TokenType, usize),
) -> alloc::vec::Vec<Expression>
{
    let __start0 = __1.0;
    let __end0 = __2.2;
    let __temp0 = __action52(
        input,
        __1,
        __2,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action54(
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action57<
>(
    input: &str,
    __0: (usize, core::option::Option<Expression>, usize),
) -> Vec<Expression>
{
    let __start0 = __0.0;
    let __end0 = __0.0;
    let __temp0 = __action50(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action45(
        input,
        __temp0,
        __0,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action58<
>(
    input: &str,
    __0: (usize, alloc::vec::Vec<Expression>, usize),
    __1: (usize, core::option::Option<Expression>, usize),
) -> Vec<Expression>
{
    let __start0 = __0.0;
    let __end0 = __0.2;
    let __temp0 = __action51(
        input,
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action45(
        input,
        __temp0,
        __1,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action59<
>(
    input: &str,
    __0: (usize, Expression, usize),
    __1: (usize, TokenType, usize),
    __2: (usize, usize, usize),
    __3: (usize, Expression, usize),
) -> Expression
{
    let __start0 = __0.2;
    let __end0 = __1.0;
    let __temp0 = __action44(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action26(
        input,
        __0,
        __temp0,
        __1,
        __2,
        __3,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action60<
>(
    input: &str,
    __0: (usize, Expression, usize),
    __1: (usize, TokenType, usize),
    __2: (usize, usize, usize),
    __3: (usize, Expression, usize),
) -> Expression
{
    let __start0 = __0.2;
    let __end0 = __1.0;
    let __temp0 = __action44(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action27(
        input,
        __0,
        __temp0,
        __1,
        __2,
        __3,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action61<
>(
    input: &str,
    __0: (usize, Expression, usize),
    __1: (usize, TokenType, usize),
    __2: (usize, Expression, usize),
    __3: (usize, TokenType, usize),
    __4: (usize, TokenType, usize),
    __5: (usize, usize, usize),
) -> Expression
{
    let __start0 = __3.2;
    let __end0 = __4.0;
    let __temp0 = __action44(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action21(
        input,
        __0,
        __1,
        __2,
        __3,
        __temp0,
        __4,
        __5,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action62<
>(
    input: &str,
    __0: (usize, Expression, usize),
    __1: (usize, TokenType, usize),
    __2: (usize, Expression, usize),
    __3: (usize, TokenType, usize),
    __4: (usize, TokenType, usize),
    __5: (usize, usize, usize),
) -> Expression
{
    let __start0 = __3.2;
    let __end0 = __4.0;
    let __temp0 = __action44(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action22(
        input,
        __0,
        __1,
        __2,
        __3,
        __temp0,
        __4,
        __5,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action63<
>(
    input: &str,
    __0: (usize, Expression, usize),
    __1: (usize, TokenType, usize),
    __2: (usize, Expression, usize),
    __3: (usize, TokenType, usize),
    __4: (usize, usize, usize),
) -> Expression
{
    let __start0 = __2.2;
    let __end0 = __3.0;
    let __temp0 = __action44(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action23(
        input,
        __0,
        __1,
        __2,
        __temp0,
        __3,
        __4,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action64<
>(
    input: &str,
    __0: (usize, Expression, usize),
    __1: (usize, TokenType, usize),
    __2: (usize, Expression, usize),
    __3: (usize, TokenType, usize),
    __4: (usize, usize, usize),
) -> Expression
{
    let __start0 = __2.2;
    let __end0 = __3.0;
    let __temp0 = __action44(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action24(
        input,
        __0,
        __1,
        __2,
        __temp0,
        __3,
        __4,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action65<
>(
    input: &str,
    __0: (usize, Expression, usize),
    __1: (usize, TokenType, usize),
    __2: (usize, Expression, usize),
    __3: (usize, TokenType, usize),
    __4: (usize, TokenType, usize),
    __5: (usize, usize, usize),
) -> Expression
{
    let __start0 = __3.2;
    let __end0 = __4.0;
    let __temp0 = __action44(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action16(
        input,
        __0,
        __1,
        __2,
        __3,
        __temp0,
        __4,
        __5,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action66<
>(
    input: &str,
    __0: (usize, Expression, usize),
    __1: (usize, TokenType, usize),
    __2: (usize, Expression, usize),
    __3: (usize, TokenType, usize),
    __4: (usize, TokenType, usize),
    __5: (usize, usize, usize),
) -> Expression
{
    let __start0 = __3.2;
    let __end0 = __4.0;
    let __temp0 = __action44(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action17(
        input,
        __0,
        __1,
        __2,
        __3,
        __temp0,
        __4,
        __5,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action67<
>(
    input: &str,
    __0: (usize, Expression, usize),
    __1: (usize, TokenType, usize),
    __2: (usize, Expression, usize),
    __3: (usize, TokenType, usize),
    __4: (usize, usize, usize),
) -> Expression
{
    let __start0 = __2.2;
    let __end0 = __3.0;
    let __temp0 = __action44(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action18(
        input,
        __0,
        __1,
        __2,
        __temp0,
        __3,
        __4,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action68<
>(
    input: &str,
    __0: (usize, Expression, usize),
    __1: (usize, TokenType, usize),
    __2: (usize, Expression, usize),
    __3: (usize, TokenType, usize),
    __4: (usize, usize, usize),
) -> Expression
{
    let __start0 = __2.2;
    let __end0 = __3.0;
    let __temp0 = __action44(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action19(
        input,
        __0,
        __1,
        __2,
        __temp0,
        __3,
        __4,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action69<
>(
    input: &str,
    __0: (usize, Expression, usize),
    __1: (usize, TokenType, usize),
    __2: (usize, usize, usize),
    __3: (usize, Expression, usize),
) -> Expression
{
    let __start0 = __0.2;
    let __end0 = __1.0;
    let __temp0 = __action44(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action29(
        input,
        __0,
        __temp0,
        __1,
        __2,
        __3,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action70<
>(
    input: &str,
    __0: (usize, Expression, usize),
    __1: (usize, TokenType, usize),
    __2: (usize, usize, usize),
    __3: (usize, Expression, usize),
) -> Expression
{
    let __start0 = __0.2;
    let __end0 = __1.0;
    let __temp0 = __action44(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action30(
        input,
        __0,
        __temp0,
        __1,
        __2,
        __3,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action71<
>(
    input: &str,
    __0: (usize, Expression, usize),
    __1: (usize, TokenType, usize),
    __2: (usize, usize, usize),
    __3: (usize, Expression, usize),
) -> Expression
{
    let __start0 = __0.2;
    let __end0 = __1.0;
    let __temp0 = __action44(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action31(
        input,
        __0,
        __temp0,
        __1,
        __2,
        __3,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action72<
>(
    input: &str,
    __0: (usize, Expression, usize),
    __1: (usize, TokenType, usize),
    __2: (usize, Expression, usize),
    __3: (usize, TokenType, usize),
    __4: (usize, usize, usize),
) -> Statement
{
    let __start0 = __0.0;
    let __end0 = __0.0;
    let __temp0 = __action44(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action4(
        input,
        __temp0,
        __0,
        __1,
        __2,
        __3,
        __4,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action73<
>(
    input: &str,
    __0: (usize, TokenType, usize),
    __1: (usize, Expression, usize),
    __2: (usize, TokenType, usize),
    __3: (usize, usize, usize),
) -> Statement
{
    let __start0 = __0.0;
    let __end0 = __0.0;
    let __temp0 = __action44(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action5(
        input,
        __temp0,
        __0,
        __1,
        __2,
        __3,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action74<
>(
    input: &str,
    __0: (usize, Expression, usize),
    __1: (usize, TokenType, usize),
    __2: (usize, TokenType, usize),
    __3: (usize, SourceUnit, usize),
    __4: (usize, TokenType, usize),
    __5: (usize, usize, usize),
) -> Statement
{
    let __start0 = __0.0;
    let __end0 = __0.0;
    let __temp0 = __action44(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action6(
        input,
        __temp0,
        __0,
        __1,
        __2,
        __3,
        __4,
        __5,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action75<
>(
    input: &str,
    __0: (usize, Expression, usize),
    __1: (usize, TokenType, usize),
    __2: (usize, TokenType, usize),
    __3: (usize, SourceUnit, usize),
    __4: (usize, TokenType, usize),
    __5: (usize, TokenType, usize),
    __6: (usize, TokenType, usize),
    __7: (usize, SourceUnit, usize),
    __8: (usize, TokenType, usize),
    __9: (usize, usize, usize),
) -> Statement
{
    let __start0 = __0.0;
    let __end0 = __0.0;
    let __temp0 = __action44(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action7(
        input,
        __temp0,
        __0,
        __1,
        __2,
        __3,
        __4,
        __5,
        __6,
        __7,
        __8,
        __9,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action76<
>(
    input: &str,
    __0: (usize, Expression, usize),
    __1: (usize, TokenType, usize),
    __2: (usize, SourceUnit, usize),
    __3: (usize, TokenType, usize),
    __4: (usize, usize, usize),
) -> Statement
{
    let __start0 = __0.0;
    let __end0 = __0.0;
    let __temp0 = __action44(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action8(
        input,
        __temp0,
        __0,
        __1,
        __2,
        __3,
        __4,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action77<
>(
    input: &str,
    __0: (usize, Expression, usize),
    __1: (usize, TokenType, usize),
    __2: (usize, SourceUnit, usize),
    __3: (usize, TokenType, usize),
    __4: (usize, TokenType, usize),
    __5: (usize, TokenType, usize),
    __6: (usize, SourceUnit, usize),
    __7: (usize, TokenType, usize),
    __8: (usize, usize, usize),
) -> Statement
{
    let __start0 = __0.0;
    let __end0 = __0.0;
    let __temp0 = __action44(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action9(
        input,
        __temp0,
        __0,
        __1,
        __2,
        __3,
        __4,
        __5,
        __6,
        __7,
        __8,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action78<
>(
    input: &str,
    __0: (usize, Expression, usize),
    __1: (usize, TokenType, usize),
    __2: (usize, Vec<Expression>, usize),
    __3: (usize, TokenType, usize),
    __4: (usize, TokenType, usize),
    __5: (usize, SourceUnit, usize),
    __6: (usize, TokenType, usize),
    __7: (usize, usize, usize),
) -> Statement
{
    let __start0 = __0.0;
    let __end0 = __0.0;
    let __temp0 = __action44(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action10(
        input,
        __temp0,
        __0,
        __1,
        __2,
        __3,
        __4,
        __5,
        __6,
        __7,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action79<
>(
    input: &str,
    __0: (usize, TokenType, usize),
    __1: (usize, Expression, usize),
    __2: (usize, TokenType, usize),
    __3: (usize, usize, usize),
) -> Statement
{
    let __start0 = __0.0;
    let __end0 = __0.0;
    let __temp0 = __action44(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action11(
        input,
        __temp0,
        __0,
        __1,
        __2,
        __3,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action80<
>(
    input: &str,
    __0: (usize, Expression, usize),
    __1: (usize, TokenType, usize),
    __2: (usize, usize, usize),
) -> Statement
{
    let __start0 = __0.0;
    let __end0 = __0.0;
    let __temp0 = __action44(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action12(
        input,
        __temp0,
        __0,
        __1,
        __2,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action81<
>(
    input: &str,
    __0: (usize, Expression, usize),
    __1: (usize, TokenType, usize),
    __2: (usize, TokenType, usize),
    __3: (usize, TokenType, usize),
    __4: (usize, SourceUnit, usize),
    __5: (usize, TokenType, usize),
    __6: (usize, usize, usize),
) -> Statement
{
    let __start0 = __0.0;
    let __end0 = __0.0;
    let __temp0 = __action44(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action13(
        input,
        __temp0,
        __0,
        __1,
        __2,
        __3,
        __4,
        __5,
        __6,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action82<
>(
    input: &str,
    __0: (usize, Expression, usize),
    __1: (usize, TokenType, usize),
    __2: (usize, TokenType, usize),
    __3: (usize, SourceUnit, usize),
    __4: (usize, TokenType, usize),
    __5: (usize, usize, usize),
) -> Statement
{
    let __start0 = __0.0;
    let __end0 = __0.0;
    let __temp0 = __action44(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action14(
        input,
        __temp0,
        __0,
        __1,
        __2,
        __3,
        __4,
        __5,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action83<
>(
    input: &str,
    __0: (usize, TokenType, usize),
    __1: (usize, usize, usize),
) -> Expression
{
    let __start0 = __0.0;
    let __end0 = __0.0;
    let __temp0 = __action44(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action35(
        input,
        __temp0,
        __0,
        __1,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action84<
>(
    input: &str,
    __0: (usize, TokenType, usize),
    __1: (usize, usize, usize),
) -> Expression
{
    let __start0 = __0.0;
    let __end0 = __0.0;
    let __temp0 = __action44(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action36(
        input,
        __temp0,
        __0,
        __1,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action85<
>(
    input: &str,
    __0: (usize, TokenType, usize),
    __1: (usize, usize, usize),
) -> Expression
{
    let __start0 = __0.0;
    let __end0 = __0.0;
    let __temp0 = __action44(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action37(
        input,
        __temp0,
        __0,
        __1,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action86<
>(
    input: &str,
    __0: (usize, TokenType, usize),
    __1: (usize, usize, usize),
) -> Expression
{
    let __start0 = __0.0;
    let __end0 = __0.0;
    let __temp0 = __action44(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action38(
        input,
        __temp0,
        __0,
        __1,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action87<
>(
    input: &str,
    __0: (usize, TokenType, usize),
    __1: (usize, usize, usize),
) -> Expression
{
    let __start0 = __0.0;
    let __end0 = __0.0;
    let __temp0 = __action44(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action39(
        input,
        __temp0,
        __0,
        __1,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action88<
>(
    input: &str,
    __0: (usize, TokenType, usize),
    __1: (usize, usize, usize),
) -> Expression
{
    let __start0 = __0.0;
    let __end0 = __0.0;
    let __temp0 = __action44(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action40(
        input,
        __temp0,
        __0,
        __1,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action89<
>(
    input: &str,
    __0: (usize, Expression, usize),
    __1: (usize, TokenType, usize),
    __2: (usize, Vec<Expression>, usize),
    __3: (usize, TokenType, usize),
    __4: (usize, usize, usize),
) -> Expression
{
    let __start0 = __0.0;
    let __end0 = __0.0;
    let __temp0 = __action44(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action41(
        input,
        __temp0,
        __0,
        __1,
        __2,
        __3,
        __4,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action90<
>(
    input: &str,
    __0: (usize, TokenType, usize),
    __1: (usize, usize, usize),
    __2: (usize, Expression, usize),
) -> Expression
{
    let __start0 = __0.0;
    let __end0 = __0.0;
    let __temp0 = __action44(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action33(
        input,
        __temp0,
        __0,
        __1,
        __2,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action91<
>(
    input: &str,
    __0: (usize, Expression, usize),
    __1: (usize, TokenType, usize),
    __2: (usize, Expression, usize),
) -> Expression
{
    let __start0 = __1.2;
    let __end0 = __2.0;
    let __temp0 = __action43(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action59(
        input,
        __0,
        __1,
        __temp0,
        __2,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action92<
>(
    input: &str,
    __0: (usize, Expression, usize),
    __1: (usize, TokenType, usize),
    __2: (usize, Expression, usize),
) -> Expression
{
    let __start0 = __1.2;
    let __end0 = __2.0;
    let __temp0 = __action43(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action60(
        input,
        __0,
        __1,
        __temp0,
        __2,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action93<
>(
    input: &str,
    __0: (usize, Expression, usize),
    __1: (usize, TokenType, usize),
    __2: (usize, Expression, usize),
    __3: (usize, TokenType, usize),
    __4: (usize, TokenType, usize),
) -> Expression
{
    let __start0 = __4.2;
    let __end0 = __4.2;
    let __temp0 = __action43(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action61(
        input,
        __0,
        __1,
        __2,
        __3,
        __4,
        __temp0,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action94<
>(
    input: &str,
    __0: (usize, Expression, usize),
    __1: (usize, TokenType, usize),
    __2: (usize, Expression, usize),
    __3: (usize, TokenType, usize),
    __4: (usize, TokenType, usize),
) -> Expression
{
    let __start0 = __4.2;
    let __end0 = __4.2;
    let __temp0 = __action43(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action62(
        input,
        __0,
        __1,
        __2,
        __3,
        __4,
        __temp0,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action95<
>(
    input: &str,
    __0: (usize, Expression, usize),
    __1: (usize, TokenType, usize),
    __2: (usize, Expression, usize),
    __3: (usize, TokenType, usize),
) -> Expression
{
    let __start0 = __3.2;
    let __end0 = __3.2;
    let __temp0 = __action43(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action63(
        input,
        __0,
        __1,
        __2,
        __3,
        __temp0,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action96<
>(
    input: &str,
    __0: (usize, Expression, usize),
    __1: (usize, TokenType, usize),
    __2: (usize, Expression, usize),
    __3: (usize, TokenType, usize),
) -> Expression
{
    let __start0 = __3.2;
    let __end0 = __3.2;
    let __temp0 = __action43(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action64(
        input,
        __0,
        __1,
        __2,
        __3,
        __temp0,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action97<
>(
    input: &str,
    __0: (usize, Expression, usize),
    __1: (usize, TokenType, usize),
    __2: (usize, Expression, usize),
    __3: (usize, TokenType, usize),
    __4: (usize, TokenType, usize),
) -> Expression
{
    let __start0 = __4.2;
    let __end0 = __4.2;
    let __temp0 = __action43(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action65(
        input,
        __0,
        __1,
        __2,
        __3,
        __4,
        __temp0,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action98<
>(
    input: &str,
    __0: (usize, Expression, usize),
    __1: (usize, TokenType, usize),
    __2: (usize, Expression, usize),
    __3: (usize, TokenType, usize),
    __4: (usize, TokenType, usize),
) -> Expression
{
    let __start0 = __4.2;
    let __end0 = __4.2;
    let __temp0 = __action43(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action66(
        input,
        __0,
        __1,
        __2,
        __3,
        __4,
        __temp0,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action99<
>(
    input: &str,
    __0: (usize, Expression, usize),
    __1: (usize, TokenType, usize),
    __2: (usize, Expression, usize),
    __3: (usize, TokenType, usize),
) -> Expression
{
    let __start0 = __3.2;
    let __end0 = __3.2;
    let __temp0 = __action43(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action67(
        input,
        __0,
        __1,
        __2,
        __3,
        __temp0,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action100<
>(
    input: &str,
    __0: (usize, Expression, usize),
    __1: (usize, TokenType, usize),
    __2: (usize, Expression, usize),
    __3: (usize, TokenType, usize),
) -> Expression
{
    let __start0 = __3.2;
    let __end0 = __3.2;
    let __temp0 = __action43(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action68(
        input,
        __0,
        __1,
        __2,
        __3,
        __temp0,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action101<
>(
    input: &str,
    __0: (usize, Expression, usize),
    __1: (usize, TokenType, usize),
    __2: (usize, Expression, usize),
) -> Expression
{
    let __start0 = __1.2;
    let __end0 = __2.0;
    let __temp0 = __action43(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action69(
        input,
        __0,
        __1,
        __temp0,
        __2,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action102<
>(
    input: &str,
    __0: (usize, Expression, usize),
    __1: (usize, TokenType, usize),
    __2: (usize, Expression, usize),
) -> Expression
{
    let __start0 = __1.2;
    let __end0 = __2.0;
    let __temp0 = __action43(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action70(
        input,
        __0,
        __1,
        __temp0,
        __2,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action103<
>(
    input: &str,
    __0: (usize, Expression, usize),
    __1: (usize, TokenType, usize),
    __2: (usize, Expression, usize),
) -> Expression
{
    let __start0 = __1.2;
    let __end0 = __2.0;
    let __temp0 = __action43(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action71(
        input,
        __0,
        __1,
        __temp0,
        __2,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action104<
>(
    input: &str,
    __0: (usize, Expression, usize),
    __1: (usize, TokenType, usize),
    __2: (usize, Expression, usize),
    __3: (usize, TokenType, usize),
) -> Statement
{
    let __start0 = __3.2;
    let __end0 = __3.2;
    let __temp0 = __action43(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action72(
        input,
        __0,
        __1,
        __2,
        __3,
        __temp0,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action105<
>(
    input: &str,
    __0: (usize, TokenType, usize),
    __1: (usize, Expression, usize),
    __2: (usize, TokenType, usize),
) -> Statement
{
    let __start0 = __2.2;
    let __end0 = __2.2;
    let __temp0 = __action43(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action73(
        input,
        __0,
        __1,
        __2,
        __temp0,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action106<
>(
    input: &str,
    __0: (usize, Expression, usize),
    __1: (usize, TokenType, usize),
    __2: (usize, TokenType, usize),
    __3: (usize, SourceUnit, usize),
    __4: (usize, TokenType, usize),
) -> Statement
{
    let __start0 = __4.2;
    let __end0 = __4.2;
    let __temp0 = __action43(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action74(
        input,
        __0,
        __1,
        __2,
        __3,
        __4,
        __temp0,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action107<
>(
    input: &str,
    __0: (usize, Expression, usize),
    __1: (usize, TokenType, usize),
    __2: (usize, TokenType, usize),
    __3: (usize, SourceUnit, usize),
    __4: (usize, TokenType, usize),
    __5: (usize, TokenType, usize),
    __6: (usize, TokenType, usize),
    __7: (usize, SourceUnit, usize),
    __8: (usize, TokenType, usize),
) -> Statement
{
    let __start0 = __8.2;
    let __end0 = __8.2;
    let __temp0 = __action43(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action75(
        input,
        __0,
        __1,
        __2,
        __3,
        __4,
        __5,
        __6,
        __7,
        __8,
        __temp0,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action108<
>(
    input: &str,
    __0: (usize, Expression, usize),
    __1: (usize, TokenType, usize),
    __2: (usize, SourceUnit, usize),
    __3: (usize, TokenType, usize),
) -> Statement
{
    let __start0 = __3.2;
    let __end0 = __3.2;
    let __temp0 = __action43(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action76(
        input,
        __0,
        __1,
        __2,
        __3,
        __temp0,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action109<
>(
    input: &str,
    __0: (usize, Expression, usize),
    __1: (usize, TokenType, usize),
    __2: (usize, SourceUnit, usize),
    __3: (usize, TokenType, usize),
    __4: (usize, TokenType, usize),
    __5: (usize, TokenType, usize),
    __6: (usize, SourceUnit, usize),
    __7: (usize, TokenType, usize),
) -> Statement
{
    let __start0 = __7.2;
    let __end0 = __7.2;
    let __temp0 = __action43(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action77(
        input,
        __0,
        __1,
        __2,
        __3,
        __4,
        __5,
        __6,
        __7,
        __temp0,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action110<
>(
    input: &str,
    __0: (usize, Expression, usize),
    __1: (usize, TokenType, usize),
    __2: (usize, Vec<Expression>, usize),
    __3: (usize, TokenType, usize),
    __4: (usize, TokenType, usize),
    __5: (usize, SourceUnit, usize),
    __6: (usize, TokenType, usize),
) -> Statement
{
    let __start0 = __6.2;
    let __end0 = __6.2;
    let __temp0 = __action43(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action78(
        input,
        __0,
        __1,
        __2,
        __3,
        __4,
        __5,
        __6,
        __temp0,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action111<
>(
    input: &str,
    __0: (usize, TokenType, usize),
    __1: (usize, Expression, usize),
    __2: (usize, TokenType, usize),
) -> Statement
{
    let __start0 = __2.2;
    let __end0 = __2.2;
    let __temp0 = __action43(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action79(
        input,
        __0,
        __1,
        __2,
        __temp0,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action112<
>(
    input: &str,
    __0: (usize, Expression, usize),
    __1: (usize, TokenType, usize),
) -> Statement
{
    let __start0 = __1.2;
    let __end0 = __1.2;
    let __temp0 = __action43(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action80(
        input,
        __0,
        __1,
        __temp0,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action113<
>(
    input: &str,
    __0: (usize, Expression, usize),
    __1: (usize, TokenType, usize),
    __2: (usize, TokenType, usize),
    __3: (usize, TokenType, usize),
    __4: (usize, SourceUnit, usize),
    __5: (usize, TokenType, usize),
) -> Statement
{
    let __start0 = __5.2;
    let __end0 = __5.2;
    let __temp0 = __action43(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action81(
        input,
        __0,
        __1,
        __2,
        __3,
        __4,
        __5,
        __temp0,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action114<
>(
    input: &str,
    __0: (usize, Expression, usize),
    __1: (usize, TokenType, usize),
    __2: (usize, TokenType, usize),
    __3: (usize, SourceUnit, usize),
    __4: (usize, TokenType, usize),
) -> Statement
{
    let __start0 = __4.2;
    let __end0 = __4.2;
    let __temp0 = __action43(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action82(
        input,
        __0,
        __1,
        __2,
        __3,
        __4,
        __temp0,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action115<
>(
    input: &str,
    __0: (usize, TokenType, usize),
) -> Expression
{
    let __start0 = __0.2;
    let __end0 = __0.2;
    let __temp0 = __action43(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action83(
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action116<
>(
    input: &str,
    __0: (usize, TokenType, usize),
) -> Expression
{
    let __start0 = __0.2;
    let __end0 = __0.2;
    let __temp0 = __action43(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action84(
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action117<
>(
    input: &str,
    __0: (usize, TokenType, usize),
) -> Expression
{
    let __start0 = __0.2;
    let __end0 = __0.2;
    let __temp0 = __action43(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action85(
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action118<
>(
    input: &str,
    __0: (usize, TokenType, usize),
) -> Expression
{
    let __start0 = __0.2;
    let __end0 = __0.2;
    let __temp0 = __action43(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action86(
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action119<
>(
    input: &str,
    __0: (usize, TokenType, usize),
) -> Expression
{
    let __start0 = __0.2;
    let __end0 = __0.2;
    let __temp0 = __action43(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action87(
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action120<
>(
    input: &str,
    __0: (usize, TokenType, usize),
) -> Expression
{
    let __start0 = __0.2;
    let __end0 = __0.2;
    let __temp0 = __action43(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action88(
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action121<
>(
    input: &str,
    __0: (usize, Expression, usize),
    __1: (usize, TokenType, usize),
    __2: (usize, Vec<Expression>, usize),
    __3: (usize, TokenType, usize),
) -> Expression
{
    let __start0 = __3.2;
    let __end0 = __3.2;
    let __temp0 = __action43(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action89(
        input,
        __0,
        __1,
        __2,
        __3,
        __temp0,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action122<
>(
    input: &str,
    __0: (usize, TokenType, usize),
    __1: (usize, Expression, usize),
) -> Expression
{
    let __start0 = __0.2;
    let __end0 = __1.0;
    let __temp0 = __action43(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action90(
        input,
        __0,
        __temp0,
        __1,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action123<
>(
    input: &str,
    __0: (usize, Expression, usize),
) -> Vec<Expression>
{
    let __start0 = __0.0;
    let __end0 = __0.2;
    let __temp0 = __action48(
        input,
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action57(
        input,
        __temp0,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action124<
>(
    input: &str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> Vec<Expression>
{
    let __start0 = *__lookbehind;
    let __end0 = *__lookahead;
    let __temp0 = __action49(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action57(
        input,
        __temp0,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action125<
>(
    input: &str,
    __0: (usize, alloc::vec::Vec<Expression>, usize),
    __1: (usize, Expression, usize),
) -> Vec<Expression>
{
    let __start0 = __1.0;
    let __end0 = __1.2;
    let __temp0 = __action48(
        input,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action58(
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action126<
>(
    input: &str,
    __0: (usize, alloc::vec::Vec<Expression>, usize),
) -> Vec<Expression>
{
    let __start0 = __0.2;
    let __end0 = __0.2;
    let __temp0 = __action49(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action58(
        input,
        __0,
        __temp0,
    )
}
#[allow(clippy::type_complexity)]

pub trait __ToTriple<>
{
    fn to_triple(value: Self) -> Result<(usize,TokenType,usize), __lalrpop_util::ParseError<usize, TokenType, LexicalError>>;
}

impl<> __ToTriple<> for (usize, TokenType, usize)
{
    fn to_triple(value: Self) -> Result<(usize,TokenType,usize), __lalrpop_util::ParseError<usize, TokenType, LexicalError>> {
        Ok(value)
    }
}
impl<> __ToTriple<> for Result<(usize, TokenType, usize), LexicalError>
{
    fn to_triple(value: Self) -> Result<(usize,TokenType,usize), __lalrpop_util::ParseError<usize, TokenType, LexicalError>> {
        match value {
            Ok(v) => Ok(v),
            Err(error) => Err(__lalrpop_util::ParseError::User { error }),
        }
    }
}
