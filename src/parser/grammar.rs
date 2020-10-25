// auto-generated: "lalrpop 0.18.1"
// sha256: 6d6925aed3ea702a74b955b0f85229c3a679c6846b511ca4d4a9f41c8dabd
use crate::executor::ast::*;
use crate::lexer::tokens::*;
use crate::lexer::LexicalError;
#[allow(unused_extern_crates)]
extern crate lalrpop_util as __lalrpop_util;
#[allow(unused_imports)]
use self::__lalrpop_util::state_machine as __state_machine;

#[cfg_attr(rustfmt, rustfmt_skip)]
mod __parse__Expressions {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports, unused_parens)]

    use crate::lexer::tokens::*;
    use crate::lexer::LexicalError;
    use crate::executor::ast::*;
    #[allow(unused_extern_crates)]
    extern crate lalrpop_util as __lalrpop_util;
    #[allow(unused_imports)]
    use self::__lalrpop_util::state_machine as __state_machine;
    use super::__ToTriple;
    #[allow(dead_code)]
    pub enum __Symbol<>
     {
        Variant0(TokenType),
        Variant1(Expression),
        Variant2(::std::vec::Vec<Expression>),
        Variant3(usize),
        Variant4(Vec<Expression>),
        Variant5(::std::option::Option<Expression>),
        Variant6(SourceUnit),
        Variant7(SourceUnitPart),
        Variant8(::std::vec::Vec<SourceUnitPart>),
        Variant9(Statement),
    }
    const __ACTION: &'static [i8] = &[
        // State 0
        0, 0, 10, 0, 0, 0, 0, 11, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 12, 0, 13, 0, 14, 15, 16, 0, 17, 0,
        // State 1
        0, 0, 10, 0, 0, 0, 0, 11, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 12, 0, 13, 0, 14, 15, 16, 0, 17, 0,
        // State 2
        0, 0, 0, -20, 0, 19, -20, 20, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 3
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 4
        0, 0, 0, 0, 0, 0, 21, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 5
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 6
        0, 22, 0, -10, 23, -10, -10, -10, 24, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 7
        0, -48, 0, -48, -48, -48, -48, -48, -48, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 8
        0, -27, 0, -27, -27, -27, -27, -27, -27, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 9
        0, 0, 10, 0, 0, 0, 0, 11, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 12, 0, 13, 0, 14, 15, 16, 0, 17, 0,
        // State 10
        0, 0, 10, 0, 0, 0, 0, 11, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 12, 0, 13, 0, 14, 15, 16, 0, 17, 0,
        // State 11
        0, -41, 0, -41, -41, -41, -41, -41, -41, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 12
        0, -42, 0, -42, -42, -42, -42, -42, -42, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 13
        0, -44, 0, -44, -44, -44, -44, -44, -44, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 14
        0, -45, 0, -45, -45, -45, -45, -45, -45, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 15
        0, -40, 0, -40, -40, -40, -40, -40, -40, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 16
        0, -43, 0, -43, -43, -43, -43, -43, -43, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 17
        0, 0, 0, 0, 0, 0, 27, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 18
        0, 0, 10, 0, 0, 0, 0, 11, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 12, 0, 13, 0, 14, 15, 16, 0, 17, 0,
        // State 19
        0, 0, 10, 0, 0, 0, 0, 11, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 12, 0, 13, 0, 14, 15, 16, 0, 17, 0,
        // State 20
        0, 0, -4, 0, 0, 0, 0, -4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -4, 0, -4, 0, -4, -4, -4, 0, -4, 0,
        // State 21
        0, 0, 10, 0, 0, 0, 0, 11, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 12, 0, 13, 0, 14, 15, 16, 0, 17, 0,
        // State 22
        0, 0, 10, 0, 0, 0, 0, 11, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 12, 0, 13, 0, 14, 15, 16, 0, 17, 0,
        // State 23
        0, 0, 10, 0, 0, 0, 0, 11, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 12, 0, 13, 0, 14, 15, 16, 0, 17, 0,
        // State 24
        0, 0, 0, 33, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 25
        0, -47, 0, -47, -47, -47, -47, -47, -47, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 26
        0, 0, -5, 0, 0, 0, 0, -5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -5, 0, -5, 0, -5, -5, -5, 0, -5, 0,
        // State 27
        0, 22, 0, -8, 23, -8, -8, -8, 24, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 28
        0, 22, 0, -9, 23, -9, -9, -9, 24, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 29
        0, -26, 0, -26, -26, -26, -26, -26, -26, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 30
        0, -24, 0, -24, -24, -24, -24, -24, -24, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 31
        0, -25, 0, -25, -25, -25, -25, -25, -25, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 32
        0, -46, 0, -46, -46, -46, -46, -46, -46, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    const __EOF_ACTION: &'static [i8] = &[
        // State 0
        -12,
        // State 1
        -14,
        // State 2
        -20,
        // State 3
        -23,
        // State 4
        -11,
        // State 5
        -49,
        // State 6
        -10,
        // State 7
        -48,
        // State 8
        -27,
        // State 9
        0,
        // State 10
        0,
        // State 11
        -41,
        // State 12
        -42,
        // State 13
        -44,
        // State 14
        -45,
        // State 15
        -40,
        // State 16
        -43,
        // State 17
        -13,
        // State 18
        0,
        // State 19
        0,
        // State 20
        -4,
        // State 21
        0,
        // State 22
        0,
        // State 23
        0,
        // State 24
        0,
        // State 25
        -47,
        // State 26
        -5,
        // State 27
        -8,
        // State 28
        -9,
        // State 29
        -26,
        // State 30
        -24,
        // State 31
        -25,
        // State 32
        -46,
    ];
    const __GOTO: &'static [i8] = &[
        // State 0
        0, 0, 2, 0, 0, 3, 4, 0, 5, 0, 6, 7, 0, 0, 0, 0, 8, 9, 0, 0,
        // State 1
        0, 0, 0, 0, 0, 3, 0, 0, 18, 0, 0, 7, 0, 0, 0, 0, 8, 9, 0, 0,
        // State 2
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 3
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 4
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 5
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 6
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 7
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 8
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 9
        0, 0, 0, 0, 0, 3, 0, 0, 25, 0, 0, 7, 0, 0, 0, 0, 8, 9, 0, 0,
        // State 10
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 8, 26, 0, 0,
        // State 11
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 12
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 13
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 14
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 15
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 16
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 17
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 18
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 28, 0, 0, 0, 0, 8, 9, 0, 0,
        // State 19
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 29, 0, 0, 0, 0, 8, 9, 0, 0,
        // State 20
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 21
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 8, 30, 0, 0,
        // State 22
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 8, 31, 0, 0,
        // State 23
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 8, 32, 0, 0,
        // State 24
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 25
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 26
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 27
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 28
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 29
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 30
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 31
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 32
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    fn __expected_tokens(__state: usize) -> Vec<::std::string::String> {
        const __TERMINAL: &'static [&'static str] = &[
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
            r###""[""###,
            r###""]""###,
            r###""ne_kal""###,
            r###""um""###,
            r###""{""###,
            r###""}""###,
            r###"Declaration"###,
            r###"Else"###,
            r###"Float"###,
            r###"Function"###,
            r###"Identifier"###,
            r###"If"###,
            r###"InputNumber"###,
            r###"InputString"###,
            r###"Integer"###,
            r###"Loop"###,
            r###"StringLiteral"###,
            r###"Write"###,
        ];
        __ACTION[(__state * 32)..].iter().zip(__TERMINAL).filter_map(|(&state, terminal)| {
            if state == 0 {
                None
            } else {
                Some(terminal.to_string())
            }
        }).collect()
    }
    pub struct __StateMachine<'__0>
    where 
    {
        input: &'__0 str,
        __phantom: ::std::marker::PhantomData<()>,
    }
    impl<'__0> __state_machine::ParserDefinition for __StateMachine<'__0>
    where 
    {
        type Location = usize;
        type Error = LexicalError;
        type Token = TokenType;
        type TokenIndex = usize;
        type Symbol = __Symbol<>;
        type Success = Vec<Expression>;
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
            __token_to_integer(token, ::std::marker::PhantomData::<()>)
        }

        #[inline]
        fn action(&self, state: i8, integer: usize) -> i8 {
            __ACTION[(state as usize) * 32 + integer]
        }

        #[inline]
        fn error_action(&self, state: i8) -> i8 {
            __ACTION[(state as usize) * 32 + (32 - 1)]
        }

        #[inline]
        fn eof_action(&self, state: i8) -> i8 {
            __EOF_ACTION[state as usize]
        }

        #[inline]
        fn goto(&self, state: i8, nt: usize) -> i8 {
            __GOTO[(state as usize) * 20 + nt] - 1
        }

        fn token_to_symbol(&self, token_index: usize, token: Self::Token) -> Self::Symbol {
            __token_to_symbol(token_index, token, ::std::marker::PhantomData::<()>)
        }

        fn expected_tokens(&self, state: i8) -> Vec<String> {
            __expected_tokens(state as usize)
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
            states: &mut Vec<i8>,
            symbols: &mut Vec<__state_machine::SymbolTriple<Self>>,
        ) -> Option<__state_machine::ParseResult<Self>> {
            __reduce(
                self.input,
                action,
                start_location,
                states,
                symbols,
                ::std::marker::PhantomData::<()>,
            )
        }

        fn simulate_reduce(&self, action: i8) -> __state_machine::SimulatedReduce<Self> {
            panic!("error recovery not enabled for this grammar")
        }
    }
    fn __token_to_integer<
    >(
        __token: &TokenType,
        _: ::std::marker::PhantomData<()>,
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
            TokenType::RectO if true => Some(14),
            TokenType::RectC if true => Some(15),
            TokenType::Nekal if true => Some(16),
            TokenType::Um if true => Some(17),
            TokenType::LeftBrace if true => Some(18),
            TokenType::RightBrace if true => Some(19),
            TokenType::Declaration if true => Some(20),
            TokenType::Else if true => Some(21),
            TokenType::Float(f64) if true => Some(22),
            TokenType::Function if true => Some(23),
            TokenType::Symbol(usize) if true => Some(24),
            TokenType::If if true => Some(25),
            TokenType::InputNumber if true => Some(26),
            TokenType::InputString if true => Some(27),
            TokenType::Integer(i64) if true => Some(28),
            TokenType::Loop if true => Some(29),
            TokenType::Literal(usize) if true => Some(30),
            TokenType::Write if true => Some(31),
            _ => None,
        }
    }
    fn __token_to_symbol<
    >(
        __token_index: usize,
        __token: TokenType,
        _: ::std::marker::PhantomData<()>,
    ) -> __Symbol<>
    {
        match __token_index {
            0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19 | 20 | 21 | 22 | 23 | 24 | 25 | 26 | 27 | 28 | 29 | 30 | 31 => __Symbol::Variant0(__token),
            _ => unreachable!(),
        }
    }
    pub struct ExpressionsParser {
        _priv: (),
    }

    impl ExpressionsParser {
        pub fn new() -> ExpressionsParser {
            ExpressionsParser {
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
        ) -> Result<Vec<Expression>, __lalrpop_util::ParseError<usize, TokenType, LexicalError>>
        {
            let __tokens = __tokens0.into_iter();
            let mut __tokens = __tokens.map(|t| __ToTriple::to_triple(t));
            let __r = __state_machine::Parser::drive(
                __StateMachine {
                    input,
                    __phantom: ::std::marker::PhantomData::<()>,
                },
                __tokens,
            );
            __r
        }
    }
    pub(crate) fn __reduce<
    >(
        input: &str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> Option<Result<Vec<Expression>,__lalrpop_util::ParseError<usize, TokenType, LexicalError>>>
    {
        let (__pop_states, __nonterminal) = match __action {
            0 => {
                __reduce0(input, __lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            1 => {
                __reduce1(input, __lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            2 => {
                __reduce2(input, __lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            3 => {
                __reduce3(input, __lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            4 => {
                __reduce4(input, __lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            5 => {
                __reduce5(input, __lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            6 => {
                __reduce6(input, __lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            7 => {
                __reduce7(input, __lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            8 => {
                __reduce8(input, __lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            9 => {
                __reduce9(input, __lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            10 => {
                __reduce10(input, __lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            11 => {
                __reduce11(input, __lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            12 => {
                __reduce12(input, __lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            13 => {
                __reduce13(input, __lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            14 => {
                __reduce14(input, __lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            15 => {
                __reduce15(input, __lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            16 => {
                __reduce16(input, __lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            17 => {
                __reduce17(input, __lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            18 => {
                __reduce18(input, __lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            19 => {
                __reduce19(input, __lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            20 => {
                __reduce20(input, __lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            21 => {
                __reduce21(input, __lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            22 => {
                __reduce22(input, __lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            23 => {
                __reduce23(input, __lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            24 => {
                __reduce24(input, __lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            25 => {
                __reduce25(input, __lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            26 => {
                __reduce26(input, __lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            27 => {
                __reduce27(input, __lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            28 => {
                __reduce28(input, __lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            29 => {
                __reduce29(input, __lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            30 => {
                __reduce30(input, __lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            31 => {
                __reduce31(input, __lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            32 => {
                __reduce32(input, __lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            33 => {
                __reduce33(input, __lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            34 => {
                __reduce34(input, __lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            35 => {
                __reduce35(input, __lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            36 => {
                __reduce36(input, __lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            37 => {
                __reduce37(input, __lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            38 => {
                __reduce38(input, __lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            39 => {
                __reduce39(input, __lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            40 => {
                __reduce40(input, __lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            41 => {
                __reduce41(input, __lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            42 => {
                __reduce42(input, __lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            43 => {
                __reduce43(input, __lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            44 => {
                __reduce44(input, __lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            45 => {
                __reduce45(input, __lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            46 => {
                __reduce46(input, __lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            47 => {
                __reduce47(input, __lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            48 => {
                // __Expressions = Expressions => ActionFn(1);
                let __sym0 = __pop_Variant4(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action1::<>(input, __sym0);
                return Some(Ok(__nt));
            }
            49 => {
                __reduce49(input, __lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __states_len = __states.len();
        __states.truncate(__states_len - __pop_states);
        let __state = *__states.last().unwrap() as usize;
        let __next_state = __GOTO[__state * 20 + __nonterminal] - 1;
        __states.push(__next_state);
        None
    }
    fn __pop_Variant1<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Expression, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant1(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant6<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, SourceUnit, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant6(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant7<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, SourceUnitPart, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant7(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant9<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Statement, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant9(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant0<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, TokenType, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant0(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant4<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Vec<Expression>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant4(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant3<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, usize, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant3(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant5<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, ::std::option::Option<Expression>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant5(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant2<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, ::std::vec::Vec<Expression>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant2(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant8<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, ::std::vec::Vec<SourceUnitPart>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant8(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    pub(crate) fn __reduce0<
    >(
        input: &str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // (<Expression> ",") = Expression, "," => ActionFn(44);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action44::<>(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (2, 0)
    }
    pub(crate) fn __reduce1<
    >(
        input: &str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // (<Expression> ",")* =  => ActionFn(42);
        let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
        let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
        let __nt = super::__action42::<>(input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (0, 1)
    }
    pub(crate) fn __reduce2<
    >(
        input: &str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // (<Expression> ",")* = (<Expression> ",")+ => ActionFn(43);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action43::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (1, 1)
    }
    pub(crate) fn __reduce3<
    >(
        input: &str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // (<Expression> ",")+ = Expression, "," => ActionFn(47);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action47::<>(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (2, 2)
    }
    pub(crate) fn __reduce4<
    >(
        input: &str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // (<Expression> ",")+ = (<Expression> ",")+, Expression, "," => ActionFn(48);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action48::<>(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (3, 2)
    }
    pub(crate) fn __reduce5<
    >(
        input: &str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // @L =  => ActionFn(36);
        let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
        let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
        let __nt = super::__action36::<>(input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (0, 3)
    }
    pub(crate) fn __reduce6<
    >(
        input: &str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // @R =  => ActionFn(35);
        let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
        let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
        let __nt = super::__action35::<>(input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (0, 4)
    }
    pub(crate) fn __reduce7<
    >(
        input: &str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ArithExpression = ArithExpression, "+", Factor => ActionFn(75);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant1(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action75::<>(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (3, 5)
    }
    pub(crate) fn __reduce8<
    >(
        input: &str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ArithExpression = ArithExpression, "-", Factor => ActionFn(76);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant1(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action76::<>(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (3, 5)
    }
    pub(crate) fn __reduce9<
    >(
        input: &str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ArithExpression = Factor => ActionFn(21);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action21::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 5)
    }
    pub(crate) fn __reduce10<
    >(
        input: &str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Comma<Expression> = Expression => ActionFn(99);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action99::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 6)
    }
    pub(crate) fn __reduce11<
    >(
        input: &str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Comma<Expression> =  => ActionFn(100);
        let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
        let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
        let __nt = super::__action100::<>(input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (0, 6)
    }
    pub(crate) fn __reduce12<
    >(
        input: &str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Comma<Expression> = (<Expression> ",")+, Expression => ActionFn(101);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action101::<>(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (2, 6)
    }
    pub(crate) fn __reduce13<
    >(
        input: &str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Comma<Expression> = (<Expression> ",")+ => ActionFn(102);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action102::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 6)
    }
    pub(crate) fn __reduce14<
    >(
        input: &str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Conditional = Expression, "um", Expression, "um", "==" => ActionFn(77);
        assert!(__symbols.len() >= 5);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant1(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym4.2.clone();
        let __nt = super::__action77::<>(input, __sym0, __sym1, __sym2, __sym3, __sym4);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (5, 7)
    }
    pub(crate) fn __reduce15<
    >(
        input: &str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Conditional = Expression, "um", Expression, "um", "!=" => ActionFn(78);
        assert!(__symbols.len() >= 5);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant1(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym4.2.clone();
        let __nt = super::__action78::<>(input, __sym0, __sym1, __sym2, __sym3, __sym4);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (5, 7)
    }
    pub(crate) fn __reduce16<
    >(
        input: &str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Conditional = Expression, "ne_kal", Expression, ">" => ActionFn(79);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant1(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action79::<>(input, __sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (4, 7)
    }
    pub(crate) fn __reduce17<
    >(
        input: &str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Conditional = Expression, "ne_kal", Expression, "<" => ActionFn(80);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant1(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action80::<>(input, __sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (4, 7)
    }
    pub(crate) fn __reduce18<
    >(
        input: &str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Conditional = Term => ActionFn(18);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action18::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 7)
    }
    pub(crate) fn __reduce19<
    >(
        input: &str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expression = ArithExpression => ActionFn(13);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action13::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 8)
    }
    pub(crate) fn __reduce20<
    >(
        input: &str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expression? = Expression => ActionFn(40);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action40::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 9)
    }
    pub(crate) fn __reduce21<
    >(
        input: &str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expression? =  => ActionFn(41);
        let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
        let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
        let __nt = super::__action41::<>(input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (0, 9)
    }
    pub(crate) fn __reduce22<
    >(
        input: &str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expressions = Comma<Expression> => ActionFn(4);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action4::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 10)
    }
    pub(crate) fn __reduce23<
    >(
        input: &str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Factor = Factor, "*", Unary => ActionFn(81);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant1(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action81::<>(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (3, 11)
    }
    pub(crate) fn __reduce24<
    >(
        input: &str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Factor = Factor, "/", Unary => ActionFn(82);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant1(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action82::<>(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (3, 11)
    }
    pub(crate) fn __reduce25<
    >(
        input: &str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Factor = Factor, "%", Unary => ActionFn(83);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant1(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action83::<>(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (3, 11)
    }
    pub(crate) fn __reduce26<
    >(
        input: &str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Factor = Unary => ActionFn(25);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action25::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 11)
    }
    pub(crate) fn __reduce27<
    >(
        input: &str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // SourceUnit = SourceUnitPart+ => ActionFn(2);
        let __sym0 = __pop_Variant8(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action2::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 12)
    }
    pub(crate) fn __reduce28<
    >(
        input: &str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // SourceUnitPart = Statement => ActionFn(3);
        let __sym0 = __pop_Variant9(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action3::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 13)
    }
    pub(crate) fn __reduce29<
    >(
        input: &str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // SourceUnitPart+ = SourceUnitPart => ActionFn(38);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action38::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (1, 14)
    }
    pub(crate) fn __reduce30<
    >(
        input: &str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // SourceUnitPart+ = SourceUnitPart+, SourceUnitPart => ActionFn(39);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant7(__symbols);
        let __sym0 = __pop_Variant8(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action39::<>(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (2, 14)
    }
    pub(crate) fn __reduce31<
    >(
        input: &str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Statement = Declaration, Expression, ";" => ActionFn(84);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action84::<>(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (3, 15)
    }
    pub(crate) fn __reduce32<
    >(
        input: &str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Statement = Expression, "=", Expression, ";" => ActionFn(85);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant1(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action85::<>(input, __sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (4, 15)
    }
    pub(crate) fn __reduce33<
    >(
        input: &str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Statement = Write, Expression, ";" => ActionFn(86);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action86::<>(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (3, 15)
    }
    pub(crate) fn __reduce34<
    >(
        input: &str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Statement = If, Conditional, "{", SourceUnit, "}" => ActionFn(87);
        assert!(__symbols.len() >= 5);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant6(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym4.2.clone();
        let __nt = super::__action87::<>(input, __sym0, __sym1, __sym2, __sym3, __sym4);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (5, 15)
    }
    pub(crate) fn __reduce35<
    >(
        input: &str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Statement = If, Conditional, "{", SourceUnit, "}", Else, "{", SourceUnit, "}" => ActionFn(88);
        assert!(__symbols.len() >= 9);
        let __sym8 = __pop_Variant0(__symbols);
        let __sym7 = __pop_Variant6(__symbols);
        let __sym6 = __pop_Variant0(__symbols);
        let __sym5 = __pop_Variant0(__symbols);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant6(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym8.2.clone();
        let __nt = super::__action88::<>(input, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5, __sym6, __sym7, __sym8);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (9, 15)
    }
    pub(crate) fn __reduce36<
    >(
        input: &str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Statement = Function, Expression, "(", Expressions, ")", "{", SourceUnit, "}" => ActionFn(89);
        assert!(__symbols.len() >= 8);
        let __sym7 = __pop_Variant0(__symbols);
        let __sym6 = __pop_Variant6(__symbols);
        let __sym5 = __pop_Variant0(__symbols);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant4(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym7.2.clone();
        let __nt = super::__action89::<>(input, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5, __sym6, __sym7);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (8, 15)
    }
    pub(crate) fn __reduce37<
    >(
        input: &str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Statement = Expression, "(", Expressions, ")", ";" => ActionFn(90);
        assert!(__symbols.len() >= 5);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant4(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym4.2.clone();
        let __nt = super::__action90::<>(input, __sym0, __sym1, __sym2, __sym3, __sym4);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (5, 15)
    }
    pub(crate) fn __reduce38<
    >(
        input: &str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Statement = Loop, Conditional, "{", SourceUnit, "}" => ActionFn(91);
        assert!(__symbols.len() >= 5);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant6(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym4.2.clone();
        let __nt = super::__action91::<>(input, __sym0, __sym1, __sym2, __sym3, __sym4);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (5, 15)
    }
    pub(crate) fn __reduce39<
    >(
        input: &str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Term = Integer => ActionFn(92);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action92::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 16)
    }
    pub(crate) fn __reduce40<
    >(
        input: &str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Term = Float => ActionFn(93);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action93::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 16)
    }
    pub(crate) fn __reduce41<
    >(
        input: &str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Term = Identifier => ActionFn(94);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action94::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 16)
    }
    pub(crate) fn __reduce42<
    >(
        input: &str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Term = StringLiteral => ActionFn(95);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action95::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 16)
    }
    pub(crate) fn __reduce43<
    >(
        input: &str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Term = InputNumber => ActionFn(96);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action96::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 16)
    }
    pub(crate) fn __reduce44<
    >(
        input: &str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Term = InputString => ActionFn(97);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action97::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 16)
    }
    pub(crate) fn __reduce45<
    >(
        input: &str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Term = "(", Expression, ")" => ActionFn(34);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action34::<>(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (3, 16)
    }
    pub(crate) fn __reduce46<
    >(
        input: &str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Unary = "-", Unary => ActionFn(98);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action98::<>(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (2, 17)
    }
    pub(crate) fn __reduce47<
    >(
        input: &str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Unary = Term => ActionFn(27);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action27::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 17)
    }
    pub(crate) fn __reduce49<
    >(
        input: &str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // __SourceUnit = SourceUnit => ActionFn(0);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action0::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 19)
    }
}
pub use self::__parse__Expressions::ExpressionsParser;

#[cfg_attr(rustfmt, rustfmt_skip)]
mod __parse__SourceUnit {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports, unused_parens)]

    use crate::lexer::tokens::*;
    use crate::lexer::LexicalError;
    use crate::executor::ast::*;
    #[allow(unused_extern_crates)]
    extern crate lalrpop_util as __lalrpop_util;
    #[allow(unused_imports)]
    use self::__lalrpop_util::state_machine as __state_machine;
    use super::__ToTriple;
    #[allow(dead_code)]
    pub enum __Symbol<>
     {
        Variant0(TokenType),
        Variant1(Expression),
        Variant2(::std::vec::Vec<Expression>),
        Variant3(usize),
        Variant4(Vec<Expression>),
        Variant5(::std::option::Option<Expression>),
        Variant6(SourceUnit),
        Variant7(SourceUnitPart),
        Variant8(::std::vec::Vec<SourceUnitPart>),
        Variant9(Statement),
    }
    const __ACTION: &'static [i8] = &[
        // State 0
        0, 0, 11, 0, 0, 0, 0, 12, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 13, 0, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23,
        // State 1
        0, 0, -20, -20, 0, 24, -20, 25, 0, -20, -20, -20, 0, -20, 0, 0, -20, -20, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 2
        0, 0, 26, 0, 0, 0, 0, 0, 0, 0, 0, 27, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 3
        0, 28, -10, -10, 29, -10, -10, -10, 30, -10, -10, -10, 0, -10, 0, 0, -10, -10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 4
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 5
        0, 0, -30, 0, 0, 0, 0, -30, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -30, -30, 0, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30,
        // State 6
        0, 0, 11, 0, 0, 0, 0, 12, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -28, 13, 0, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23,
        // State 7
        0, 0, -29, 0, 0, 0, 0, -29, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -29, -29, 0, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29,
        // State 8
        0, -48, -48, -48, -48, -48, -48, -48, -48, -48, -48, -48, 0, -48, 0, 0, -48, -48, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 9
        0, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, 0, -27, 0, 0, -27, -27, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 10
        0, 0, 11, 0, 0, 0, 0, 12, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 14, 0, 16, 0, 18, 19, 20, 0, 22, 0,
        // State 11
        0, 0, 11, 0, 0, 0, 0, 12, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 14, 0, 16, 0, 18, 19, 20, 0, 22, 0,
        // State 12
        0, 0, 11, 0, 0, 0, 0, 12, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 14, 0, 16, 0, 18, 19, 20, 0, 22, 0,
        // State 13
        0, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, 0, -41, 0, 0, -41, -41, -41, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 14
        0, 0, 11, 0, 0, 0, 0, 12, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 14, 0, 16, 0, 18, 19, 20, 0, 22, 0,
        // State 15
        0, -42, -42, -42, -42, -42, -42, -42, -42, -42, -42, -42, 0, -42, 0, 0, -42, -42, -42, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 16
        0, 0, 11, 0, 0, 0, 0, 12, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 14, 0, 16, 0, 18, 19, 20, 0, 22, 0,
        // State 17
        0, -44, -44, -44, -44, -44, -44, -44, -44, -44, -44, -44, 0, -44, 0, 0, -44, -44, -44, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 18
        0, -45, -45, -45, -45, -45, -45, -45, -45, -45, -45, -45, 0, -45, 0, 0, -45, -45, -45, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 19
        0, -40, -40, -40, -40, -40, -40, -40, -40, -40, -40, -40, 0, -40, 0, 0, -40, -40, -40, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 20
        0, 0, 11, 0, 0, 0, 0, 12, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 14, 0, 16, 0, 18, 19, 20, 0, 22, 0,
        // State 21
        0, -43, -43, -43, -43, -43, -43, -43, -43, -43, -43, -43, 0, -43, 0, 0, -43, -43, -43, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 22
        0, 0, 11, 0, 0, 0, 0, 12, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 14, 0, 16, 0, 18, 19, 20, 0, 22, 0,
        // State 23
        0, 0, 11, 0, 0, 0, 0, 12, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 14, 0, 16, 0, 18, 19, 20, 0, 22, 0,
        // State 24
        0, 0, 11, 0, 0, 0, 0, 12, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 14, 0, 16, 0, 18, 19, 20, 0, 22, 0,
        // State 25
        0, 0, 11, -12, 0, 0, 0, 12, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 14, 0, 16, 0, 18, 19, 20, 0, 22, 0,
        // State 26
        0, 0, 11, 0, 0, 0, 0, 12, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 14, 0, 16, 0, 18, 19, 20, 0, 22, 0,
        // State 27
        0, 0, 11, 0, 0, 0, 0, 12, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 14, 0, 16, 0, 18, 19, 20, 0, 22, 0,
        // State 28
        0, 0, 11, 0, 0, 0, 0, 12, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 14, 0, 16, 0, 18, 19, 20, 0, 22, 0,
        // State 29
        0, 0, 11, 0, 0, 0, 0, 12, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 14, 0, 16, 0, 18, 19, 20, 0, 22, 0,
        // State 30
        0, 0, -31, 0, 0, 0, 0, -31, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -31, -31, 0, -31, -31, -31, -31, -31, -31, -31, -31, -31, -31,
        // State 31
        0, 0, 0, 51, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 32
        0, -47, -47, -47, -47, -47, -47, -47, -47, -47, -47, -47, 0, -47, 0, 0, -47, -47, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 33
        0, 0, 0, 0, 0, 0, 0, 0, 0, 52, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 34
        0, 0, 53, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 35
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 54, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 36
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 55, 56, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 37
        0, -48, 0, 0, -48, -48, 0, -48, -48, 0, 0, 0, 0, 0, 0, 0, -48, -48, -19, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 38
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 57, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 39
        0, 0, 0, 0, 0, 0, 0, 0, 0, 58, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 40
        0, 28, -8, -8, 29, -8, -8, -8, 30, -8, -8, -8, 0, -8, 0, 0, -8, -8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 41
        0, 28, -9, -9, 29, -9, -9, -9, 30, -9, -9, -9, 0, -9, 0, 0, -9, -9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 42
        0, 0, 11, -14, 0, 0, 0, 12, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 14, 0, 16, 0, 18, 19, 20, 0, 22, 0,
        // State 43
        0, 0, 0, -23, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 44
        0, 0, 0, -11, 0, 0, 60, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 45
        0, 0, 0, 61, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 46
        0, 0, 0, 0, 0, 0, 0, 0, 0, 62, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 47
        0, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, 0, -26, 0, 0, -26, -26, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 48
        0, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, 0, -24, 0, 0, -24, -24, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 49
        0, -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, 0, -25, 0, 0, -25, -25, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 50
        0, -46, -46, -46, -46, -46, -46, -46, -46, -46, -46, -46, 0, -46, 0, 0, -46, -46, -46, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 51
        0, 0, -32, 0, 0, 0, 0, -32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -32, -32, 0, -32, -32, -32, -32, -32, -32, -32, -32, -32, -32,
        // State 52
        0, 0, 11, -12, 0, 0, 0, 12, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 14, 0, 16, 0, 18, 19, 20, 0, 22, 0,
        // State 53
        0, 0, 11, 0, 0, 0, 0, 12, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 13, 0, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23,
        // State 54
        0, 0, 11, 0, 0, 0, 0, 12, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 14, 0, 16, 0, 18, 19, 20, 0, 22, 0,
        // State 55
        0, 0, 11, 0, 0, 0, 0, 12, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 14, 0, 16, 0, 18, 19, 20, 0, 22, 0,
        // State 56
        0, 0, 11, 0, 0, 0, 0, 12, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 13, 0, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23,
        // State 57
        0, 0, -34, 0, 0, 0, 0, -34, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -34, -34, 0, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34,
        // State 58
        0, 0, 0, -13, 0, 0, 68, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 59
        0, 0, -4, -4, 0, 0, 0, -4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -4, 0, -4, 0, -4, -4, -4, 0, -4, 0,
        // State 60
        0, 0, 0, 0, 0, 0, 0, 0, 0, 69, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 61
        0, 0, -33, 0, 0, 0, 0, -33, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -33, -33, 0, -33, -33, -33, -33, -33, -33, -33, -33, -33, -33,
        // State 62
        0, 0, 0, 70, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 63
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 71, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 64
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 72, 0, 0, 73, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 65
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 74, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 66
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 75, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 67
        0, 0, -5, -5, 0, 0, 0, -5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -5, 0, -5, 0, -5, -5, -5, 0, -5, 0,
        // State 68
        0, 0, -38, 0, 0, 0, 0, -38, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -38, -38, 0, -38, -38, -38, -38, -38, -38, -38, -38, -38, -38,
        // State 69
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 76, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 70
        0, 0, -35, 0, 0, 0, 0, -35, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -35, -35, 77, -35, -35, -35, -35, -35, -35, -35, -35, -35, -35,
        // State 71
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -18, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 72
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -17, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 73
        78, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 79, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 74
        0, 0, -39, 0, 0, 0, 0, -39, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -39, -39, 0, -39, -39, -39, -39, -39, -39, -39, -39, -39, -39,
        // State 75
        0, 0, 11, 0, 0, 0, 0, 12, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 13, 0, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23,
        // State 76
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 81, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 77
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 78
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -15, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 79
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 82, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 80
        0, 0, 11, 0, 0, 0, 0, 12, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 13, 0, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23,
        // State 81
        0, 0, -37, 0, 0, 0, 0, -37, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -37, -37, 0, -37, -37, -37, -37, -37, -37, -37, -37, -37, -37,
        // State 82
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 84, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 83
        0, 0, -36, 0, 0, 0, 0, -36, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -36, -36, 0, -36, -36, -36, -36, -36, -36, -36, -36, -36, -36,
    ];
    const __EOF_ACTION: &'static [i8] = &[
        // State 0
        0,
        // State 1
        0,
        // State 2
        0,
        // State 3
        0,
        // State 4
        -50,
        // State 5
        -30,
        // State 6
        -28,
        // State 7
        -29,
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
        0,
        // State 30
        -31,
        // State 31
        0,
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
        0,
        // State 43
        0,
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
        -32,
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
        -34,
        // State 58
        0,
        // State 59
        0,
        // State 60
        0,
        // State 61
        -33,
        // State 62
        0,
        // State 63
        0,
        // State 64
        0,
        // State 65
        0,
        // State 66
        0,
        // State 67
        0,
        // State 68
        -38,
        // State 69
        0,
        // State 70
        -35,
        // State 71
        0,
        // State 72
        0,
        // State 73
        0,
        // State 74
        -39,
        // State 75
        0,
        // State 76
        0,
        // State 77
        0,
        // State 78
        0,
        // State 79
        0,
        // State 80
        0,
        // State 81
        -37,
        // State 82
        0,
        // State 83
        -36,
    ];
    const __GOTO: &'static [i8] = &[
        // State 0
        0, 0, 0, 0, 0, 2, 0, 0, 3, 0, 0, 4, 5, 6, 7, 8, 9, 10, 0, 0,
        // State 1
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 2
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 3
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 4
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 5
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 6
        0, 0, 0, 0, 0, 2, 0, 0, 3, 0, 0, 4, 0, 31, 0, 8, 9, 10, 0, 0,
        // State 7
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 8
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 9
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 10
        0, 0, 0, 0, 0, 2, 0, 0, 32, 0, 0, 4, 0, 0, 0, 0, 9, 10, 0, 0,
        // State 11
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 9, 33, 0, 0,
        // State 12
        0, 0, 0, 0, 0, 2, 0, 0, 34, 0, 0, 4, 0, 0, 0, 0, 9, 10, 0, 0,
        // State 13
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 14
        0, 0, 0, 0, 0, 2, 0, 0, 35, 0, 0, 4, 0, 0, 0, 0, 9, 10, 0, 0,
        // State 15
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 16
        0, 0, 0, 0, 0, 2, 0, 36, 37, 0, 0, 4, 0, 0, 0, 0, 38, 10, 0, 0,
        // State 17
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 18
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 19
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 20
        0, 0, 0, 0, 0, 2, 0, 39, 37, 0, 0, 4, 0, 0, 0, 0, 38, 10, 0, 0,
        // State 21
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 22
        0, 0, 0, 0, 0, 2, 0, 0, 40, 0, 0, 4, 0, 0, 0, 0, 9, 10, 0, 0,
        // State 23
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 41, 0, 0, 0, 0, 9, 10, 0, 0,
        // State 24
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 42, 0, 0, 0, 0, 9, 10, 0, 0,
        // State 25
        0, 0, 43, 0, 0, 2, 44, 0, 45, 0, 46, 4, 0, 0, 0, 0, 9, 10, 0, 0,
        // State 26
        0, 0, 0, 0, 0, 2, 0, 0, 47, 0, 0, 4, 0, 0, 0, 0, 9, 10, 0, 0,
        // State 27
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 9, 48, 0, 0,
        // State 28
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 9, 49, 0, 0,
        // State 29
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 9, 50, 0, 0,
        // State 30
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 31
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 32
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 33
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 34
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 35
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 36
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 37
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 38
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 39
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 40
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 41
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 42
        0, 0, 0, 0, 0, 2, 0, 0, 59, 0, 0, 4, 0, 0, 0, 0, 9, 10, 0, 0,
        // State 43
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 44
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 45
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 46
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 47
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 48
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 49
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 50
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 51
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 52
        0, 0, 43, 0, 0, 2, 44, 0, 45, 0, 63, 4, 0, 0, 0, 0, 9, 10, 0, 0,
        // State 53
        0, 0, 0, 0, 0, 2, 0, 0, 3, 0, 0, 4, 64, 6, 7, 8, 9, 10, 0, 0,
        // State 54
        0, 0, 0, 0, 0, 2, 0, 0, 65, 0, 0, 4, 0, 0, 0, 0, 9, 10, 0, 0,
        // State 55
        0, 0, 0, 0, 0, 2, 0, 0, 66, 0, 0, 4, 0, 0, 0, 0, 9, 10, 0, 0,
        // State 56
        0, 0, 0, 0, 0, 2, 0, 0, 3, 0, 0, 4, 67, 6, 7, 8, 9, 10, 0, 0,
        // State 57
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 58
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 59
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 60
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 61
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 62
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 63
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 64
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 65
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 66
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 67
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 68
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 69
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 70
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 71
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 72
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 73
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 74
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 75
        0, 0, 0, 0, 0, 2, 0, 0, 3, 0, 0, 4, 80, 6, 7, 8, 9, 10, 0, 0,
        // State 76
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 77
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 78
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 79
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 80
        0, 0, 0, 0, 0, 2, 0, 0, 3, 0, 0, 4, 83, 6, 7, 8, 9, 10, 0, 0,
        // State 81
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 82
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 83
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    fn __expected_tokens(__state: usize) -> Vec<::std::string::String> {
        const __TERMINAL: &'static [&'static str] = &[
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
            r###""[""###,
            r###""]""###,
            r###""ne_kal""###,
            r###""um""###,
            r###""{""###,
            r###""}""###,
            r###"Declaration"###,
            r###"Else"###,
            r###"Float"###,
            r###"Function"###,
            r###"Identifier"###,
            r###"If"###,
            r###"InputNumber"###,
            r###"InputString"###,
            r###"Integer"###,
            r###"Loop"###,
            r###"StringLiteral"###,
            r###"Write"###,
        ];
        __ACTION[(__state * 32)..].iter().zip(__TERMINAL).filter_map(|(&state, terminal)| {
            if state == 0 {
                None
            } else {
                Some(terminal.to_string())
            }
        }).collect()
    }
    pub struct __StateMachine<'__0>
    where 
    {
        input: &'__0 str,
        __phantom: ::std::marker::PhantomData<()>,
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
            __token_to_integer(token, ::std::marker::PhantomData::<()>)
        }

        #[inline]
        fn action(&self, state: i8, integer: usize) -> i8 {
            __ACTION[(state as usize) * 32 + integer]
        }

        #[inline]
        fn error_action(&self, state: i8) -> i8 {
            __ACTION[(state as usize) * 32 + (32 - 1)]
        }

        #[inline]
        fn eof_action(&self, state: i8) -> i8 {
            __EOF_ACTION[state as usize]
        }

        #[inline]
        fn goto(&self, state: i8, nt: usize) -> i8 {
            __GOTO[(state as usize) * 20 + nt] - 1
        }

        fn token_to_symbol(&self, token_index: usize, token: Self::Token) -> Self::Symbol {
            __token_to_symbol(token_index, token, ::std::marker::PhantomData::<()>)
        }

        fn expected_tokens(&self, state: i8) -> Vec<String> {
            __expected_tokens(state as usize)
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
            states: &mut Vec<i8>,
            symbols: &mut Vec<__state_machine::SymbolTriple<Self>>,
        ) -> Option<__state_machine::ParseResult<Self>> {
            __reduce(
                self.input,
                action,
                start_location,
                states,
                symbols,
                ::std::marker::PhantomData::<()>,
            )
        }

        fn simulate_reduce(&self, action: i8) -> __state_machine::SimulatedReduce<Self> {
            panic!("error recovery not enabled for this grammar")
        }
    }
    fn __token_to_integer<
    >(
        __token: &TokenType,
        _: ::std::marker::PhantomData<()>,
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
            TokenType::RectO if true => Some(14),
            TokenType::RectC if true => Some(15),
            TokenType::Nekal if true => Some(16),
            TokenType::Um if true => Some(17),
            TokenType::LeftBrace if true => Some(18),
            TokenType::RightBrace if true => Some(19),
            TokenType::Declaration if true => Some(20),
            TokenType::Else if true => Some(21),
            TokenType::Float(f64) if true => Some(22),
            TokenType::Function if true => Some(23),
            TokenType::Symbol(usize) if true => Some(24),
            TokenType::If if true => Some(25),
            TokenType::InputNumber if true => Some(26),
            TokenType::InputString if true => Some(27),
            TokenType::Integer(i64) if true => Some(28),
            TokenType::Loop if true => Some(29),
            TokenType::Literal(usize) if true => Some(30),
            TokenType::Write if true => Some(31),
            _ => None,
        }
    }
    fn __token_to_symbol<
    >(
        __token_index: usize,
        __token: TokenType,
        _: ::std::marker::PhantomData<()>,
    ) -> __Symbol<>
    {
        match __token_index {
            0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19 | 20 | 21 | 22 | 23 | 24 | 25 | 26 | 27 | 28 | 29 | 30 | 31 => __Symbol::Variant0(__token),
            _ => unreachable!(),
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
            let __r = __state_machine::Parser::drive(
                __StateMachine {
                    input,
                    __phantom: ::std::marker::PhantomData::<()>,
                },
                __tokens,
            );
            __r
        }
    }
    pub(crate) fn __reduce<
    >(
        input: &str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> Option<Result<SourceUnit,__lalrpop_util::ParseError<usize, TokenType, LexicalError>>>
    {
        let (__pop_states, __nonterminal) = match __action {
            0 => {
                __reduce0(input, __lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            1 => {
                __reduce1(input, __lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            2 => {
                __reduce2(input, __lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            3 => {
                __reduce3(input, __lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            4 => {
                __reduce4(input, __lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            5 => {
                __reduce5(input, __lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            6 => {
                __reduce6(input, __lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            7 => {
                __reduce7(input, __lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            8 => {
                __reduce8(input, __lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            9 => {
                __reduce9(input, __lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            10 => {
                __reduce10(input, __lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            11 => {
                __reduce11(input, __lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            12 => {
                __reduce12(input, __lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            13 => {
                __reduce13(input, __lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            14 => {
                __reduce14(input, __lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            15 => {
                __reduce15(input, __lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            16 => {
                __reduce16(input, __lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            17 => {
                __reduce17(input, __lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            18 => {
                __reduce18(input, __lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            19 => {
                __reduce19(input, __lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            20 => {
                __reduce20(input, __lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            21 => {
                __reduce21(input, __lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            22 => {
                __reduce22(input, __lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            23 => {
                __reduce23(input, __lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            24 => {
                __reduce24(input, __lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            25 => {
                __reduce25(input, __lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            26 => {
                __reduce26(input, __lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            27 => {
                __reduce27(input, __lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            28 => {
                __reduce28(input, __lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            29 => {
                __reduce29(input, __lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            30 => {
                __reduce30(input, __lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            31 => {
                __reduce31(input, __lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            32 => {
                __reduce32(input, __lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            33 => {
                __reduce33(input, __lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            34 => {
                __reduce34(input, __lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            35 => {
                __reduce35(input, __lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            36 => {
                __reduce36(input, __lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            37 => {
                __reduce37(input, __lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            38 => {
                __reduce38(input, __lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            39 => {
                __reduce39(input, __lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            40 => {
                __reduce40(input, __lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            41 => {
                __reduce41(input, __lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            42 => {
                __reduce42(input, __lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            43 => {
                __reduce43(input, __lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            44 => {
                __reduce44(input, __lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            45 => {
                __reduce45(input, __lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            46 => {
                __reduce46(input, __lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            47 => {
                __reduce47(input, __lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            48 => {
                __reduce48(input, __lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            49 => {
                // __SourceUnit = SourceUnit => ActionFn(0);
                let __sym0 = __pop_Variant6(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action0::<>(input, __sym0);
                return Some(Ok(__nt));
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __states_len = __states.len();
        __states.truncate(__states_len - __pop_states);
        let __state = *__states.last().unwrap() as usize;
        let __next_state = __GOTO[__state * 20 + __nonterminal] - 1;
        __states.push(__next_state);
        None
    }
    fn __pop_Variant1<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Expression, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant1(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant6<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, SourceUnit, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant6(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant7<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, SourceUnitPart, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant7(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant9<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Statement, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant9(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant0<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, TokenType, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant0(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant4<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Vec<Expression>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant4(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant3<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, usize, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant3(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant5<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, ::std::option::Option<Expression>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant5(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant2<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, ::std::vec::Vec<Expression>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant2(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant8<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, ::std::vec::Vec<SourceUnitPart>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant8(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    pub(crate) fn __reduce0<
    >(
        input: &str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // (<Expression> ",") = Expression, "," => ActionFn(44);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action44::<>(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (2, 0)
    }
    pub(crate) fn __reduce1<
    >(
        input: &str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // (<Expression> ",")* =  => ActionFn(42);
        let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
        let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
        let __nt = super::__action42::<>(input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (0, 1)
    }
    pub(crate) fn __reduce2<
    >(
        input: &str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // (<Expression> ",")* = (<Expression> ",")+ => ActionFn(43);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action43::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (1, 1)
    }
    pub(crate) fn __reduce3<
    >(
        input: &str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // (<Expression> ",")+ = Expression, "," => ActionFn(47);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action47::<>(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (2, 2)
    }
    pub(crate) fn __reduce4<
    >(
        input: &str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // (<Expression> ",")+ = (<Expression> ",")+, Expression, "," => ActionFn(48);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action48::<>(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (3, 2)
    }
    pub(crate) fn __reduce5<
    >(
        input: &str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // @L =  => ActionFn(36);
        let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
        let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
        let __nt = super::__action36::<>(input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (0, 3)
    }
    pub(crate) fn __reduce6<
    >(
        input: &str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // @R =  => ActionFn(35);
        let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
        let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
        let __nt = super::__action35::<>(input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (0, 4)
    }
    pub(crate) fn __reduce7<
    >(
        input: &str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ArithExpression = ArithExpression, "+", Factor => ActionFn(75);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant1(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action75::<>(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (3, 5)
    }
    pub(crate) fn __reduce8<
    >(
        input: &str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ArithExpression = ArithExpression, "-", Factor => ActionFn(76);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant1(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action76::<>(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (3, 5)
    }
    pub(crate) fn __reduce9<
    >(
        input: &str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ArithExpression = Factor => ActionFn(21);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action21::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 5)
    }
    pub(crate) fn __reduce10<
    >(
        input: &str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Comma<Expression> = Expression => ActionFn(99);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action99::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 6)
    }
    pub(crate) fn __reduce11<
    >(
        input: &str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Comma<Expression> =  => ActionFn(100);
        let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
        let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
        let __nt = super::__action100::<>(input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (0, 6)
    }
    pub(crate) fn __reduce12<
    >(
        input: &str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Comma<Expression> = (<Expression> ",")+, Expression => ActionFn(101);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action101::<>(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (2, 6)
    }
    pub(crate) fn __reduce13<
    >(
        input: &str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Comma<Expression> = (<Expression> ",")+ => ActionFn(102);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action102::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 6)
    }
    pub(crate) fn __reduce14<
    >(
        input: &str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Conditional = Expression, "um", Expression, "um", "==" => ActionFn(77);
        assert!(__symbols.len() >= 5);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant1(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym4.2.clone();
        let __nt = super::__action77::<>(input, __sym0, __sym1, __sym2, __sym3, __sym4);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (5, 7)
    }
    pub(crate) fn __reduce15<
    >(
        input: &str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Conditional = Expression, "um", Expression, "um", "!=" => ActionFn(78);
        assert!(__symbols.len() >= 5);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant1(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym4.2.clone();
        let __nt = super::__action78::<>(input, __sym0, __sym1, __sym2, __sym3, __sym4);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (5, 7)
    }
    pub(crate) fn __reduce16<
    >(
        input: &str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Conditional = Expression, "ne_kal", Expression, ">" => ActionFn(79);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant1(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action79::<>(input, __sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (4, 7)
    }
    pub(crate) fn __reduce17<
    >(
        input: &str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Conditional = Expression, "ne_kal", Expression, "<" => ActionFn(80);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant1(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action80::<>(input, __sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (4, 7)
    }
    pub(crate) fn __reduce18<
    >(
        input: &str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Conditional = Term => ActionFn(18);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action18::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 7)
    }
    pub(crate) fn __reduce19<
    >(
        input: &str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expression = ArithExpression => ActionFn(13);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action13::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 8)
    }
    pub(crate) fn __reduce20<
    >(
        input: &str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expression? = Expression => ActionFn(40);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action40::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 9)
    }
    pub(crate) fn __reduce21<
    >(
        input: &str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expression? =  => ActionFn(41);
        let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
        let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
        let __nt = super::__action41::<>(input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (0, 9)
    }
    pub(crate) fn __reduce22<
    >(
        input: &str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expressions = Comma<Expression> => ActionFn(4);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action4::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 10)
    }
    pub(crate) fn __reduce23<
    >(
        input: &str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Factor = Factor, "*", Unary => ActionFn(81);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant1(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action81::<>(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (3, 11)
    }
    pub(crate) fn __reduce24<
    >(
        input: &str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Factor = Factor, "/", Unary => ActionFn(82);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant1(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action82::<>(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (3, 11)
    }
    pub(crate) fn __reduce25<
    >(
        input: &str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Factor = Factor, "%", Unary => ActionFn(83);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant1(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action83::<>(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (3, 11)
    }
    pub(crate) fn __reduce26<
    >(
        input: &str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Factor = Unary => ActionFn(25);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action25::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 11)
    }
    pub(crate) fn __reduce27<
    >(
        input: &str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // SourceUnit = SourceUnitPart+ => ActionFn(2);
        let __sym0 = __pop_Variant8(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action2::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 12)
    }
    pub(crate) fn __reduce28<
    >(
        input: &str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // SourceUnitPart = Statement => ActionFn(3);
        let __sym0 = __pop_Variant9(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action3::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 13)
    }
    pub(crate) fn __reduce29<
    >(
        input: &str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // SourceUnitPart+ = SourceUnitPart => ActionFn(38);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action38::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (1, 14)
    }
    pub(crate) fn __reduce30<
    >(
        input: &str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // SourceUnitPart+ = SourceUnitPart+, SourceUnitPart => ActionFn(39);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant7(__symbols);
        let __sym0 = __pop_Variant8(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action39::<>(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (2, 14)
    }
    pub(crate) fn __reduce31<
    >(
        input: &str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Statement = Declaration, Expression, ";" => ActionFn(84);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action84::<>(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (3, 15)
    }
    pub(crate) fn __reduce32<
    >(
        input: &str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Statement = Expression, "=", Expression, ";" => ActionFn(85);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant1(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action85::<>(input, __sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (4, 15)
    }
    pub(crate) fn __reduce33<
    >(
        input: &str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Statement = Write, Expression, ";" => ActionFn(86);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action86::<>(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (3, 15)
    }
    pub(crate) fn __reduce34<
    >(
        input: &str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Statement = If, Conditional, "{", SourceUnit, "}" => ActionFn(87);
        assert!(__symbols.len() >= 5);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant6(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym4.2.clone();
        let __nt = super::__action87::<>(input, __sym0, __sym1, __sym2, __sym3, __sym4);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (5, 15)
    }
    pub(crate) fn __reduce35<
    >(
        input: &str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Statement = If, Conditional, "{", SourceUnit, "}", Else, "{", SourceUnit, "}" => ActionFn(88);
        assert!(__symbols.len() >= 9);
        let __sym8 = __pop_Variant0(__symbols);
        let __sym7 = __pop_Variant6(__symbols);
        let __sym6 = __pop_Variant0(__symbols);
        let __sym5 = __pop_Variant0(__symbols);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant6(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym8.2.clone();
        let __nt = super::__action88::<>(input, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5, __sym6, __sym7, __sym8);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (9, 15)
    }
    pub(crate) fn __reduce36<
    >(
        input: &str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Statement = Function, Expression, "(", Expressions, ")", "{", SourceUnit, "}" => ActionFn(89);
        assert!(__symbols.len() >= 8);
        let __sym7 = __pop_Variant0(__symbols);
        let __sym6 = __pop_Variant6(__symbols);
        let __sym5 = __pop_Variant0(__symbols);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant4(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym7.2.clone();
        let __nt = super::__action89::<>(input, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5, __sym6, __sym7);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (8, 15)
    }
    pub(crate) fn __reduce37<
    >(
        input: &str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Statement = Expression, "(", Expressions, ")", ";" => ActionFn(90);
        assert!(__symbols.len() >= 5);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant4(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym4.2.clone();
        let __nt = super::__action90::<>(input, __sym0, __sym1, __sym2, __sym3, __sym4);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (5, 15)
    }
    pub(crate) fn __reduce38<
    >(
        input: &str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Statement = Loop, Conditional, "{", SourceUnit, "}" => ActionFn(91);
        assert!(__symbols.len() >= 5);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant6(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym4.2.clone();
        let __nt = super::__action91::<>(input, __sym0, __sym1, __sym2, __sym3, __sym4);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (5, 15)
    }
    pub(crate) fn __reduce39<
    >(
        input: &str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Term = Integer => ActionFn(92);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action92::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 16)
    }
    pub(crate) fn __reduce40<
    >(
        input: &str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Term = Float => ActionFn(93);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action93::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 16)
    }
    pub(crate) fn __reduce41<
    >(
        input: &str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Term = Identifier => ActionFn(94);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action94::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 16)
    }
    pub(crate) fn __reduce42<
    >(
        input: &str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Term = StringLiteral => ActionFn(95);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action95::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 16)
    }
    pub(crate) fn __reduce43<
    >(
        input: &str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Term = InputNumber => ActionFn(96);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action96::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 16)
    }
    pub(crate) fn __reduce44<
    >(
        input: &str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Term = InputString => ActionFn(97);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action97::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 16)
    }
    pub(crate) fn __reduce45<
    >(
        input: &str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Term = "(", Expression, ")" => ActionFn(34);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action34::<>(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (3, 16)
    }
    pub(crate) fn __reduce46<
    >(
        input: &str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Unary = "-", Unary => ActionFn(98);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action98::<>(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (2, 17)
    }
    pub(crate) fn __reduce47<
    >(
        input: &str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Unary = Term => ActionFn(27);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action27::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 17)
    }
    pub(crate) fn __reduce48<
    >(
        input: &str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // __Expressions = Expressions => ActionFn(1);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action1::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 18)
    }
}
pub use self::__parse__SourceUnit::SourceUnitParser;

#[allow(unused_variables)]
fn __action0(input: &str, (_, __0, _): (usize, SourceUnit, usize)) -> SourceUnit {
    __0
}

#[allow(unused_variables)]
fn __action1(input: &str, (_, __0, _): (usize, Vec<Expression>, usize)) -> Vec<Expression> {
    __0
}

#[allow(unused_variables)]
fn __action2(
    input: &str,
    (_, __0, _): (usize, ::std::vec::Vec<SourceUnitPart>, usize),
) -> SourceUnit {
    SourceUnit(__0)
}

#[allow(unused_variables)]
fn __action3(input: &str, (_, __0, _): (usize, Statement, usize)) -> SourceUnitPart {
    SourceUnitPart::Statement(__0)
}

#[allow(unused_variables)]
fn __action4(input: &str, (_, __0, _): (usize, Vec<Expression>, usize)) -> Vec<Expression> {
    __0
}

#[allow(unused_variables)]
fn __action5(
    input: &str,
    (_, a, _): (usize, usize, usize),
    (_, _, _): (usize, TokenType, usize),
    (_, id, _): (usize, Expression, usize),
    (_, _, _): (usize, TokenType, usize),
    (_, b, _): (usize, usize, usize),
) -> Statement {
    Statement::Declaration((a, b), id)
}

#[allow(unused_variables)]
fn __action6(
    input: &str,
    (_, a, _): (usize, usize, usize),
    (_, l, _): (usize, Expression, usize),
    (_, _, _): (usize, TokenType, usize),
    (_, r, _): (usize, Expression, usize),
    (_, _, _): (usize, TokenType, usize),
    (_, b, _): (usize, usize, usize),
) -> Statement {
    Statement::Assignment((a, b), l, r)
}

#[allow(unused_variables)]
fn __action7(
    input: &str,
    (_, a, _): (usize, usize, usize),
    (_, _, _): (usize, TokenType, usize),
    (_, e, _): (usize, Expression, usize),
    (_, _, _): (usize, TokenType, usize),
    (_, b, _): (usize, usize, usize),
) -> Statement {
    Statement::Write((a, b), e)
}

#[allow(unused_variables)]
fn __action8(
    input: &str,
    (_, a, _): (usize, usize, usize),
    (_, _, _): (usize, TokenType, usize),
    (_, condition, _): (usize, Expression, usize),
    (_, _, _): (usize, TokenType, usize),
    (_, s, _): (usize, SourceUnit, usize),
    (_, _, _): (usize, TokenType, usize),
    (_, b, _): (usize, usize, usize),
) -> Statement {
    Statement::Conditional((a, b), condition, s, None)
}

#[allow(unused_variables)]
fn __action9(
    input: &str,
    (_, a, _): (usize, usize, usize),
    (_, _, _): (usize, TokenType, usize),
    (_, condition, _): (usize, Expression, usize),
    (_, _, _): (usize, TokenType, usize),
    (_, s, _): (usize, SourceUnit, usize),
    (_, _, _): (usize, TokenType, usize),
    (_, _, _): (usize, TokenType, usize),
    (_, _, _): (usize, TokenType, usize),
    (_, f, _): (usize, SourceUnit, usize),
    (_, _, _): (usize, TokenType, usize),
    (_, b, _): (usize, usize, usize),
) -> Statement {
    Statement::Conditional((a, b), condition, s, Some(f))
}

#[allow(unused_variables)]
fn __action10(
    input: &str,
    (_, a, _): (usize, usize, usize),
    (_, _, _): (usize, TokenType, usize),
    (_, name, _): (usize, Expression, usize),
    (_, _, _): (usize, TokenType, usize),
    (_, params, _): (usize, Vec<Expression>, usize),
    (_, _, _): (usize, TokenType, usize),
    (_, _, _): (usize, TokenType, usize),
    (_, s, _): (usize, SourceUnit, usize),
    (_, _, _): (usize, TokenType, usize),
    (_, b, _): (usize, usize, usize),
) -> Statement {
    Statement::FunctionDeclaration((a, b), name, params, s)
}

#[allow(unused_variables)]
fn __action11(
    input: &str,
    (_, a, _): (usize, usize, usize),
    (_, id, _): (usize, Expression, usize),
    (_, _, _): (usize, TokenType, usize),
    (_, args, _): (usize, Vec<Expression>, usize),
    (_, _, _): (usize, TokenType, usize),
    (_, _, _): (usize, TokenType, usize),
    (_, b, _): (usize, usize, usize),
) -> Statement {
    Statement::FunctionCall((a, b), id, args)
}

#[allow(unused_variables)]
fn __action12(
    input: &str,
    (_, a, _): (usize, usize, usize),
    (_, _, _): (usize, TokenType, usize),
    (_, condition, _): (usize, Expression, usize),
    (_, _, _): (usize, TokenType, usize),
    (_, s, _): (usize, SourceUnit, usize),
    (_, _, _): (usize, TokenType, usize),
    (_, b, _): (usize, usize, usize),
) -> Statement {
    Statement::Loop((a, b), condition, s)
}

#[allow(unused_variables)]
fn __action13(input: &str, (_, __0, _): (usize, Expression, usize)) -> Expression {
    __0
}

#[allow(unused_variables)]
fn __action14(
    input: &str,
    (_, l, _): (usize, Expression, usize),
    (_, _, _): (usize, TokenType, usize),
    (_, r, _): (usize, Expression, usize),
    (_, _, _): (usize, TokenType, usize),
    (_, a, _): (usize, usize, usize),
    (_, _, _): (usize, TokenType, usize),
    (_, b, _): (usize, usize, usize),
) -> Expression {
    Expression::Equals((a, b), Box::new(l), Box::new(r))
}

#[allow(unused_variables)]
fn __action15(
    input: &str,
    (_, l, _): (usize, Expression, usize),
    (_, _, _): (usize, TokenType, usize),
    (_, r, _): (usize, Expression, usize),
    (_, _, _): (usize, TokenType, usize),
    (_, a, _): (usize, usize, usize),
    (_, _, _): (usize, TokenType, usize),
    (_, b, _): (usize, usize, usize),
) -> Expression {
    Expression::NotEquals((a, b), Box::new(l), Box::new(r))
}

#[allow(unused_variables)]
fn __action16(
    input: &str,
    (_, r, _): (usize, Expression, usize),
    (_, _, _): (usize, TokenType, usize),
    (_, l, _): (usize, Expression, usize),
    (_, a, _): (usize, usize, usize),
    (_, _, _): (usize, TokenType, usize),
    (_, b, _): (usize, usize, usize),
) -> Expression {
    Expression::GreaterThan((a, b), Box::new(l), Box::new(r))
}

#[allow(unused_variables)]
fn __action17(
    input: &str,
    (_, r, _): (usize, Expression, usize),
    (_, _, _): (usize, TokenType, usize),
    (_, l, _): (usize, Expression, usize),
    (_, a, _): (usize, usize, usize),
    (_, _, _): (usize, TokenType, usize),
    (_, b, _): (usize, usize, usize),
) -> Expression {
    Expression::LessThan((a, b), Box::new(l), Box::new(r))
}

#[allow(unused_variables)]
fn __action18(input: &str, (_, __0, _): (usize, Expression, usize)) -> Expression {
    __0
}

#[allow(unused_variables)]
fn __action19(
    input: &str,
    (_, l, _): (usize, Expression, usize),
    (_, a, _): (usize, usize, usize),
    (_, _, _): (usize, TokenType, usize),
    (_, b, _): (usize, usize, usize),
    (_, r, _): (usize, Expression, usize),
) -> Expression {
    Expression::Add((a, b), Box::new(l), Box::new(r))
}

#[allow(unused_variables)]
fn __action20(
    input: &str,
    (_, l, _): (usize, Expression, usize),
    (_, a, _): (usize, usize, usize),
    (_, _, _): (usize, TokenType, usize),
    (_, b, _): (usize, usize, usize),
    (_, r, _): (usize, Expression, usize),
) -> Expression {
    Expression::Subtract((a, b), Box::new(l), Box::new(r))
}

#[allow(unused_variables)]
fn __action21(input: &str, (_, __0, _): (usize, Expression, usize)) -> Expression {
    __0
}

#[allow(unused_variables)]
fn __action22(
    input: &str,
    (_, l, _): (usize, Expression, usize),
    (_, a, _): (usize, usize, usize),
    (_, _, _): (usize, TokenType, usize),
    (_, b, _): (usize, usize, usize),
    (_, r, _): (usize, Expression, usize),
) -> Expression {
    Expression::Multiply((a, b), Box::new(l), Box::new(r))
}

#[allow(unused_variables)]
fn __action23(
    input: &str,
    (_, l, _): (usize, Expression, usize),
    (_, a, _): (usize, usize, usize),
    (_, _, _): (usize, TokenType, usize),
    (_, b, _): (usize, usize, usize),
    (_, r, _): (usize, Expression, usize),
) -> Expression {
    Expression::Divide((a, b), Box::new(l), Box::new(r))
}

#[allow(unused_variables)]
fn __action24(
    input: &str,
    (_, l, _): (usize, Expression, usize),
    (_, a, _): (usize, usize, usize),
    (_, _, _): (usize, TokenType, usize),
    (_, b, _): (usize, usize, usize),
    (_, r, _): (usize, Expression, usize),
) -> Expression {
    Expression::Modulo((a, b), Box::new(l), Box::new(r))
}

#[allow(unused_variables)]
fn __action25(input: &str, (_, __0, _): (usize, Expression, usize)) -> Expression {
    __0
}

#[allow(unused_variables)]
fn __action26(
    input: &str,
    (_, a, _): (usize, usize, usize),
    (_, _, _): (usize, TokenType, usize),
    (_, b, _): (usize, usize, usize),
    (_, r, _): (usize, Expression, usize),
) -> Expression {
    Expression::UnaryMinus((a, b), Box::new(r))
}

#[allow(unused_variables)]
fn __action27(input: &str, (_, __0, _): (usize, Expression, usize)) -> Expression {
    __0
}

#[allow(unused_variables)]
fn __action28(
    input: &str,
    (_, a, _): (usize, usize, usize),
    (_, v, _): (usize, TokenType, usize),
    (_, b, _): (usize, usize, usize),
) -> Expression {
    Expression::Integer((a, b), v)
}

#[allow(unused_variables)]
fn __action29(
    input: &str,
    (_, a, _): (usize, usize, usize),
    (_, v, _): (usize, TokenType, usize),
    (_, b, _): (usize, usize, usize),
) -> Expression {
    Expression::Float((a, b), v)
}

#[allow(unused_variables)]
fn __action30(
    input: &str,
    (_, a, _): (usize, usize, usize),
    (_, id, _): (usize, TokenType, usize),
    (_, b, _): (usize, usize, usize),
) -> Expression {
    Expression::Symbol((a, b), id)
}

#[allow(unused_variables)]
fn __action31(
    input: &str,
    (_, a, _): (usize, usize, usize),
    (_, string, _): (usize, TokenType, usize),
    (_, b, _): (usize, usize, usize),
) -> Expression {
    Expression::StringLiteral((a, b), string)
}

#[allow(unused_variables)]
fn __action32(
    input: &str,
    (_, a, _): (usize, usize, usize),
    (_, d, _): (usize, TokenType, usize),
    (_, b, _): (usize, usize, usize),
) -> Expression {
    Expression::InputNumber((a, b))
}

#[allow(unused_variables)]
fn __action33(
    input: &str,
    (_, a, _): (usize, usize, usize),
    (_, d, _): (usize, TokenType, usize),
    (_, b, _): (usize, usize, usize),
) -> Expression {
    Expression::InputString((a, b))
}

#[allow(unused_variables)]
fn __action34(
    input: &str,
    (_, _, _): (usize, TokenType, usize),
    (_, e, _): (usize, Expression, usize),
    (_, _, _): (usize, TokenType, usize),
) -> Expression {
    e
}

#[allow(unused_variables)]
fn __action35(input: &str, __lookbehind: &usize, __lookahead: &usize) -> usize {
    __lookbehind.clone()
}

#[allow(unused_variables)]
fn __action36(input: &str, __lookbehind: &usize, __lookahead: &usize) -> usize {
    __lookahead.clone()
}

#[allow(unused_variables)]
fn __action37(
    input: &str,
    (_, v, _): (usize, ::std::vec::Vec<Expression>, usize),
    (_, e, _): (usize, ::std::option::Option<Expression>, usize),
) -> Vec<Expression> {
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
fn __action38(
    input: &str,
    (_, __0, _): (usize, SourceUnitPart, usize),
) -> ::std::vec::Vec<SourceUnitPart> {
    vec![__0]
}

#[allow(unused_variables)]
fn __action39(
    input: &str,
    (_, v, _): (usize, ::std::vec::Vec<SourceUnitPart>, usize),
    (_, e, _): (usize, SourceUnitPart, usize),
) -> ::std::vec::Vec<SourceUnitPart> {
    {
        let mut v = v;
        v.push(e);
        v
    }
}

#[allow(unused_variables)]
fn __action40(
    input: &str,
    (_, __0, _): (usize, Expression, usize),
) -> ::std::option::Option<Expression> {
    Some(__0)
}

#[allow(unused_variables)]
fn __action41(
    input: &str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> ::std::option::Option<Expression> {
    None
}

#[allow(unused_variables)]
fn __action42(
    input: &str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> ::std::vec::Vec<Expression> {
    vec![]
}

#[allow(unused_variables)]
fn __action43(
    input: &str,
    (_, v, _): (usize, ::std::vec::Vec<Expression>, usize),
) -> ::std::vec::Vec<Expression> {
    v
}

#[allow(unused_variables)]
fn __action44(
    input: &str,
    (_, __0, _): (usize, Expression, usize),
    (_, _, _): (usize, TokenType, usize),
) -> Expression {
    __0
}

#[allow(unused_variables)]
fn __action45(input: &str, (_, __0, _): (usize, Expression, usize)) -> ::std::vec::Vec<Expression> {
    vec![__0]
}

#[allow(unused_variables)]
fn __action46(
    input: &str,
    (_, v, _): (usize, ::std::vec::Vec<Expression>, usize),
    (_, e, _): (usize, Expression, usize),
) -> ::std::vec::Vec<Expression> {
    {
        let mut v = v;
        v.push(e);
        v
    }
}

#[allow(unused_variables)]
fn __action47(
    input: &str,
    __0: (usize, Expression, usize),
    __1: (usize, TokenType, usize),
) -> ::std::vec::Vec<Expression> {
    let __start0 = __0.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action44(input, __0, __1);
    let __temp0 = (__start0, __temp0, __end0);
    __action45(input, __temp0)
}

#[allow(unused_variables)]
fn __action48(
    input: &str,
    __0: (usize, ::std::vec::Vec<Expression>, usize),
    __1: (usize, Expression, usize),
    __2: (usize, TokenType, usize),
) -> ::std::vec::Vec<Expression> {
    let __start0 = __1.0.clone();
    let __end0 = __2.2.clone();
    let __temp0 = __action44(input, __1, __2);
    let __temp0 = (__start0, __temp0, __end0);
    __action46(input, __0, __temp0)
}

#[allow(unused_variables)]
fn __action49(
    input: &str,
    __0: (usize, ::std::option::Option<Expression>, usize),
) -> Vec<Expression> {
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action42(input, &__start0, &__end0);
    let __temp0 = (__start0, __temp0, __end0);
    __action37(input, __temp0, __0)
}

#[allow(unused_variables)]
fn __action50(
    input: &str,
    __0: (usize, ::std::vec::Vec<Expression>, usize),
    __1: (usize, ::std::option::Option<Expression>, usize),
) -> Vec<Expression> {
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action43(input, __0);
    let __temp0 = (__start0, __temp0, __end0);
    __action37(input, __temp0, __1)
}

#[allow(unused_variables)]
fn __action51(
    input: &str,
    __0: (usize, Expression, usize),
    __1: (usize, TokenType, usize),
    __2: (usize, usize, usize),
    __3: (usize, Expression, usize),
) -> Expression {
    let __start0 = __0.2.clone();
    let __end0 = __1.0.clone();
    let __temp0 = __action36(input, &__start0, &__end0);
    let __temp0 = (__start0, __temp0, __end0);
    __action19(input, __0, __temp0, __1, __2, __3)
}

#[allow(unused_variables)]
fn __action52(
    input: &str,
    __0: (usize, Expression, usize),
    __1: (usize, TokenType, usize),
    __2: (usize, usize, usize),
    __3: (usize, Expression, usize),
) -> Expression {
    let __start0 = __0.2.clone();
    let __end0 = __1.0.clone();
    let __temp0 = __action36(input, &__start0, &__end0);
    let __temp0 = (__start0, __temp0, __end0);
    __action20(input, __0, __temp0, __1, __2, __3)
}

#[allow(unused_variables)]
fn __action53(
    input: &str,
    __0: (usize, Expression, usize),
    __1: (usize, TokenType, usize),
    __2: (usize, Expression, usize),
    __3: (usize, TokenType, usize),
    __4: (usize, TokenType, usize),
    __5: (usize, usize, usize),
) -> Expression {
    let __start0 = __3.2.clone();
    let __end0 = __4.0.clone();
    let __temp0 = __action36(input, &__start0, &__end0);
    let __temp0 = (__start0, __temp0, __end0);
    __action14(input, __0, __1, __2, __3, __temp0, __4, __5)
}

#[allow(unused_variables)]
fn __action54(
    input: &str,
    __0: (usize, Expression, usize),
    __1: (usize, TokenType, usize),
    __2: (usize, Expression, usize),
    __3: (usize, TokenType, usize),
    __4: (usize, TokenType, usize),
    __5: (usize, usize, usize),
) -> Expression {
    let __start0 = __3.2.clone();
    let __end0 = __4.0.clone();
    let __temp0 = __action36(input, &__start0, &__end0);
    let __temp0 = (__start0, __temp0, __end0);
    __action15(input, __0, __1, __2, __3, __temp0, __4, __5)
}

#[allow(unused_variables)]
fn __action55(
    input: &str,
    __0: (usize, Expression, usize),
    __1: (usize, TokenType, usize),
    __2: (usize, Expression, usize),
    __3: (usize, TokenType, usize),
    __4: (usize, usize, usize),
) -> Expression {
    let __start0 = __2.2.clone();
    let __end0 = __3.0.clone();
    let __temp0 = __action36(input, &__start0, &__end0);
    let __temp0 = (__start0, __temp0, __end0);
    __action16(input, __0, __1, __2, __temp0, __3, __4)
}

#[allow(unused_variables)]
fn __action56(
    input: &str,
    __0: (usize, Expression, usize),
    __1: (usize, TokenType, usize),
    __2: (usize, Expression, usize),
    __3: (usize, TokenType, usize),
    __4: (usize, usize, usize),
) -> Expression {
    let __start0 = __2.2.clone();
    let __end0 = __3.0.clone();
    let __temp0 = __action36(input, &__start0, &__end0);
    let __temp0 = (__start0, __temp0, __end0);
    __action17(input, __0, __1, __2, __temp0, __3, __4)
}

#[allow(unused_variables)]
fn __action57(
    input: &str,
    __0: (usize, Expression, usize),
    __1: (usize, TokenType, usize),
    __2: (usize, usize, usize),
    __3: (usize, Expression, usize),
) -> Expression {
    let __start0 = __0.2.clone();
    let __end0 = __1.0.clone();
    let __temp0 = __action36(input, &__start0, &__end0);
    let __temp0 = (__start0, __temp0, __end0);
    __action22(input, __0, __temp0, __1, __2, __3)
}

#[allow(unused_variables)]
fn __action58(
    input: &str,
    __0: (usize, Expression, usize),
    __1: (usize, TokenType, usize),
    __2: (usize, usize, usize),
    __3: (usize, Expression, usize),
) -> Expression {
    let __start0 = __0.2.clone();
    let __end0 = __1.0.clone();
    let __temp0 = __action36(input, &__start0, &__end0);
    let __temp0 = (__start0, __temp0, __end0);
    __action23(input, __0, __temp0, __1, __2, __3)
}

#[allow(unused_variables)]
fn __action59(
    input: &str,
    __0: (usize, Expression, usize),
    __1: (usize, TokenType, usize),
    __2: (usize, usize, usize),
    __3: (usize, Expression, usize),
) -> Expression {
    let __start0 = __0.2.clone();
    let __end0 = __1.0.clone();
    let __temp0 = __action36(input, &__start0, &__end0);
    let __temp0 = (__start0, __temp0, __end0);
    __action24(input, __0, __temp0, __1, __2, __3)
}

#[allow(unused_variables)]
fn __action60(
    input: &str,
    __0: (usize, TokenType, usize),
    __1: (usize, Expression, usize),
    __2: (usize, TokenType, usize),
    __3: (usize, usize, usize),
) -> Statement {
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action36(input, &__start0, &__end0);
    let __temp0 = (__start0, __temp0, __end0);
    __action5(input, __temp0, __0, __1, __2, __3)
}

#[allow(unused_variables)]
fn __action61(
    input: &str,
    __0: (usize, Expression, usize),
    __1: (usize, TokenType, usize),
    __2: (usize, Expression, usize),
    __3: (usize, TokenType, usize),
    __4: (usize, usize, usize),
) -> Statement {
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action36(input, &__start0, &__end0);
    let __temp0 = (__start0, __temp0, __end0);
    __action6(input, __temp0, __0, __1, __2, __3, __4)
}

#[allow(unused_variables)]
fn __action62(
    input: &str,
    __0: (usize, TokenType, usize),
    __1: (usize, Expression, usize),
    __2: (usize, TokenType, usize),
    __3: (usize, usize, usize),
) -> Statement {
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action36(input, &__start0, &__end0);
    let __temp0 = (__start0, __temp0, __end0);
    __action7(input, __temp0, __0, __1, __2, __3)
}

#[allow(unused_variables)]
fn __action63(
    input: &str,
    __0: (usize, TokenType, usize),
    __1: (usize, Expression, usize),
    __2: (usize, TokenType, usize),
    __3: (usize, SourceUnit, usize),
    __4: (usize, TokenType, usize),
    __5: (usize, usize, usize),
) -> Statement {
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action36(input, &__start0, &__end0);
    let __temp0 = (__start0, __temp0, __end0);
    __action8(input, __temp0, __0, __1, __2, __3, __4, __5)
}

#[allow(unused_variables)]
fn __action64(
    input: &str,
    __0: (usize, TokenType, usize),
    __1: (usize, Expression, usize),
    __2: (usize, TokenType, usize),
    __3: (usize, SourceUnit, usize),
    __4: (usize, TokenType, usize),
    __5: (usize, TokenType, usize),
    __6: (usize, TokenType, usize),
    __7: (usize, SourceUnit, usize),
    __8: (usize, TokenType, usize),
    __9: (usize, usize, usize),
) -> Statement {
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action36(input, &__start0, &__end0);
    let __temp0 = (__start0, __temp0, __end0);
    __action9(
        input, __temp0, __0, __1, __2, __3, __4, __5, __6, __7, __8, __9,
    )
}

#[allow(unused_variables)]
fn __action65(
    input: &str,
    __0: (usize, TokenType, usize),
    __1: (usize, Expression, usize),
    __2: (usize, TokenType, usize),
    __3: (usize, Vec<Expression>, usize),
    __4: (usize, TokenType, usize),
    __5: (usize, TokenType, usize),
    __6: (usize, SourceUnit, usize),
    __7: (usize, TokenType, usize),
    __8: (usize, usize, usize),
) -> Statement {
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action36(input, &__start0, &__end0);
    let __temp0 = (__start0, __temp0, __end0);
    __action10(input, __temp0, __0, __1, __2, __3, __4, __5, __6, __7, __8)
}

#[allow(unused_variables)]
fn __action66(
    input: &str,
    __0: (usize, Expression, usize),
    __1: (usize, TokenType, usize),
    __2: (usize, Vec<Expression>, usize),
    __3: (usize, TokenType, usize),
    __4: (usize, TokenType, usize),
    __5: (usize, usize, usize),
) -> Statement {
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action36(input, &__start0, &__end0);
    let __temp0 = (__start0, __temp0, __end0);
    __action11(input, __temp0, __0, __1, __2, __3, __4, __5)
}

#[allow(unused_variables)]
fn __action67(
    input: &str,
    __0: (usize, TokenType, usize),
    __1: (usize, Expression, usize),
    __2: (usize, TokenType, usize),
    __3: (usize, SourceUnit, usize),
    __4: (usize, TokenType, usize),
    __5: (usize, usize, usize),
) -> Statement {
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action36(input, &__start0, &__end0);
    let __temp0 = (__start0, __temp0, __end0);
    __action12(input, __temp0, __0, __1, __2, __3, __4, __5)
}

#[allow(unused_variables)]
fn __action68(
    input: &str,
    __0: (usize, TokenType, usize),
    __1: (usize, usize, usize),
) -> Expression {
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action36(input, &__start0, &__end0);
    let __temp0 = (__start0, __temp0, __end0);
    __action28(input, __temp0, __0, __1)
}

#[allow(unused_variables)]
fn __action69(
    input: &str,
    __0: (usize, TokenType, usize),
    __1: (usize, usize, usize),
) -> Expression {
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action36(input, &__start0, &__end0);
    let __temp0 = (__start0, __temp0, __end0);
    __action29(input, __temp0, __0, __1)
}

#[allow(unused_variables)]
fn __action70(
    input: &str,
    __0: (usize, TokenType, usize),
    __1: (usize, usize, usize),
) -> Expression {
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action36(input, &__start0, &__end0);
    let __temp0 = (__start0, __temp0, __end0);
    __action30(input, __temp0, __0, __1)
}

#[allow(unused_variables)]
fn __action71(
    input: &str,
    __0: (usize, TokenType, usize),
    __1: (usize, usize, usize),
) -> Expression {
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action36(input, &__start0, &__end0);
    let __temp0 = (__start0, __temp0, __end0);
    __action31(input, __temp0, __0, __1)
}

#[allow(unused_variables)]
fn __action72(
    input: &str,
    __0: (usize, TokenType, usize),
    __1: (usize, usize, usize),
) -> Expression {
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action36(input, &__start0, &__end0);
    let __temp0 = (__start0, __temp0, __end0);
    __action32(input, __temp0, __0, __1)
}

#[allow(unused_variables)]
fn __action73(
    input: &str,
    __0: (usize, TokenType, usize),
    __1: (usize, usize, usize),
) -> Expression {
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action36(input, &__start0, &__end0);
    let __temp0 = (__start0, __temp0, __end0);
    __action33(input, __temp0, __0, __1)
}

#[allow(unused_variables)]
fn __action74(
    input: &str,
    __0: (usize, TokenType, usize),
    __1: (usize, usize, usize),
    __2: (usize, Expression, usize),
) -> Expression {
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action36(input, &__start0, &__end0);
    let __temp0 = (__start0, __temp0, __end0);
    __action26(input, __temp0, __0, __1, __2)
}

#[allow(unused_variables)]
fn __action75(
    input: &str,
    __0: (usize, Expression, usize),
    __1: (usize, TokenType, usize),
    __2: (usize, Expression, usize),
) -> Expression {
    let __start0 = __1.2.clone();
    let __end0 = __2.0.clone();
    let __temp0 = __action35(input, &__start0, &__end0);
    let __temp0 = (__start0, __temp0, __end0);
    __action51(input, __0, __1, __temp0, __2)
}

#[allow(unused_variables)]
fn __action76(
    input: &str,
    __0: (usize, Expression, usize),
    __1: (usize, TokenType, usize),
    __2: (usize, Expression, usize),
) -> Expression {
    let __start0 = __1.2.clone();
    let __end0 = __2.0.clone();
    let __temp0 = __action35(input, &__start0, &__end0);
    let __temp0 = (__start0, __temp0, __end0);
    __action52(input, __0, __1, __temp0, __2)
}

#[allow(unused_variables)]
fn __action77(
    input: &str,
    __0: (usize, Expression, usize),
    __1: (usize, TokenType, usize),
    __2: (usize, Expression, usize),
    __3: (usize, TokenType, usize),
    __4: (usize, TokenType, usize),
) -> Expression {
    let __start0 = __4.2.clone();
    let __end0 = __4.2.clone();
    let __temp0 = __action35(input, &__start0, &__end0);
    let __temp0 = (__start0, __temp0, __end0);
    __action53(input, __0, __1, __2, __3, __4, __temp0)
}

#[allow(unused_variables)]
fn __action78(
    input: &str,
    __0: (usize, Expression, usize),
    __1: (usize, TokenType, usize),
    __2: (usize, Expression, usize),
    __3: (usize, TokenType, usize),
    __4: (usize, TokenType, usize),
) -> Expression {
    let __start0 = __4.2.clone();
    let __end0 = __4.2.clone();
    let __temp0 = __action35(input, &__start0, &__end0);
    let __temp0 = (__start0, __temp0, __end0);
    __action54(input, __0, __1, __2, __3, __4, __temp0)
}

#[allow(unused_variables)]
fn __action79(
    input: &str,
    __0: (usize, Expression, usize),
    __1: (usize, TokenType, usize),
    __2: (usize, Expression, usize),
    __3: (usize, TokenType, usize),
) -> Expression {
    let __start0 = __3.2.clone();
    let __end0 = __3.2.clone();
    let __temp0 = __action35(input, &__start0, &__end0);
    let __temp0 = (__start0, __temp0, __end0);
    __action55(input, __0, __1, __2, __3, __temp0)
}

#[allow(unused_variables)]
fn __action80(
    input: &str,
    __0: (usize, Expression, usize),
    __1: (usize, TokenType, usize),
    __2: (usize, Expression, usize),
    __3: (usize, TokenType, usize),
) -> Expression {
    let __start0 = __3.2.clone();
    let __end0 = __3.2.clone();
    let __temp0 = __action35(input, &__start0, &__end0);
    let __temp0 = (__start0, __temp0, __end0);
    __action56(input, __0, __1, __2, __3, __temp0)
}

#[allow(unused_variables)]
fn __action81(
    input: &str,
    __0: (usize, Expression, usize),
    __1: (usize, TokenType, usize),
    __2: (usize, Expression, usize),
) -> Expression {
    let __start0 = __1.2.clone();
    let __end0 = __2.0.clone();
    let __temp0 = __action35(input, &__start0, &__end0);
    let __temp0 = (__start0, __temp0, __end0);
    __action57(input, __0, __1, __temp0, __2)
}

#[allow(unused_variables)]
fn __action82(
    input: &str,
    __0: (usize, Expression, usize),
    __1: (usize, TokenType, usize),
    __2: (usize, Expression, usize),
) -> Expression {
    let __start0 = __1.2.clone();
    let __end0 = __2.0.clone();
    let __temp0 = __action35(input, &__start0, &__end0);
    let __temp0 = (__start0, __temp0, __end0);
    __action58(input, __0, __1, __temp0, __2)
}

#[allow(unused_variables)]
fn __action83(
    input: &str,
    __0: (usize, Expression, usize),
    __1: (usize, TokenType, usize),
    __2: (usize, Expression, usize),
) -> Expression {
    let __start0 = __1.2.clone();
    let __end0 = __2.0.clone();
    let __temp0 = __action35(input, &__start0, &__end0);
    let __temp0 = (__start0, __temp0, __end0);
    __action59(input, __0, __1, __temp0, __2)
}

#[allow(unused_variables)]
fn __action84(
    input: &str,
    __0: (usize, TokenType, usize),
    __1: (usize, Expression, usize),
    __2: (usize, TokenType, usize),
) -> Statement {
    let __start0 = __2.2.clone();
    let __end0 = __2.2.clone();
    let __temp0 = __action35(input, &__start0, &__end0);
    let __temp0 = (__start0, __temp0, __end0);
    __action60(input, __0, __1, __2, __temp0)
}

#[allow(unused_variables)]
fn __action85(
    input: &str,
    __0: (usize, Expression, usize),
    __1: (usize, TokenType, usize),
    __2: (usize, Expression, usize),
    __3: (usize, TokenType, usize),
) -> Statement {
    let __start0 = __3.2.clone();
    let __end0 = __3.2.clone();
    let __temp0 = __action35(input, &__start0, &__end0);
    let __temp0 = (__start0, __temp0, __end0);
    __action61(input, __0, __1, __2, __3, __temp0)
}

#[allow(unused_variables)]
fn __action86(
    input: &str,
    __0: (usize, TokenType, usize),
    __1: (usize, Expression, usize),
    __2: (usize, TokenType, usize),
) -> Statement {
    let __start0 = __2.2.clone();
    let __end0 = __2.2.clone();
    let __temp0 = __action35(input, &__start0, &__end0);
    let __temp0 = (__start0, __temp0, __end0);
    __action62(input, __0, __1, __2, __temp0)
}

#[allow(unused_variables)]
fn __action87(
    input: &str,
    __0: (usize, TokenType, usize),
    __1: (usize, Expression, usize),
    __2: (usize, TokenType, usize),
    __3: (usize, SourceUnit, usize),
    __4: (usize, TokenType, usize),
) -> Statement {
    let __start0 = __4.2.clone();
    let __end0 = __4.2.clone();
    let __temp0 = __action35(input, &__start0, &__end0);
    let __temp0 = (__start0, __temp0, __end0);
    __action63(input, __0, __1, __2, __3, __4, __temp0)
}

#[allow(unused_variables)]
fn __action88(
    input: &str,
    __0: (usize, TokenType, usize),
    __1: (usize, Expression, usize),
    __2: (usize, TokenType, usize),
    __3: (usize, SourceUnit, usize),
    __4: (usize, TokenType, usize),
    __5: (usize, TokenType, usize),
    __6: (usize, TokenType, usize),
    __7: (usize, SourceUnit, usize),
    __8: (usize, TokenType, usize),
) -> Statement {
    let __start0 = __8.2.clone();
    let __end0 = __8.2.clone();
    let __temp0 = __action35(input, &__start0, &__end0);
    let __temp0 = (__start0, __temp0, __end0);
    __action64(input, __0, __1, __2, __3, __4, __5, __6, __7, __8, __temp0)
}

#[allow(unused_variables)]
fn __action89(
    input: &str,
    __0: (usize, TokenType, usize),
    __1: (usize, Expression, usize),
    __2: (usize, TokenType, usize),
    __3: (usize, Vec<Expression>, usize),
    __4: (usize, TokenType, usize),
    __5: (usize, TokenType, usize),
    __6: (usize, SourceUnit, usize),
    __7: (usize, TokenType, usize),
) -> Statement {
    let __start0 = __7.2.clone();
    let __end0 = __7.2.clone();
    let __temp0 = __action35(input, &__start0, &__end0);
    let __temp0 = (__start0, __temp0, __end0);
    __action65(input, __0, __1, __2, __3, __4, __5, __6, __7, __temp0)
}

#[allow(unused_variables)]
fn __action90(
    input: &str,
    __0: (usize, Expression, usize),
    __1: (usize, TokenType, usize),
    __2: (usize, Vec<Expression>, usize),
    __3: (usize, TokenType, usize),
    __4: (usize, TokenType, usize),
) -> Statement {
    let __start0 = __4.2.clone();
    let __end0 = __4.2.clone();
    let __temp0 = __action35(input, &__start0, &__end0);
    let __temp0 = (__start0, __temp0, __end0);
    __action66(input, __0, __1, __2, __3, __4, __temp0)
}

#[allow(unused_variables)]
fn __action91(
    input: &str,
    __0: (usize, TokenType, usize),
    __1: (usize, Expression, usize),
    __2: (usize, TokenType, usize),
    __3: (usize, SourceUnit, usize),
    __4: (usize, TokenType, usize),
) -> Statement {
    let __start0 = __4.2.clone();
    let __end0 = __4.2.clone();
    let __temp0 = __action35(input, &__start0, &__end0);
    let __temp0 = (__start0, __temp0, __end0);
    __action67(input, __0, __1, __2, __3, __4, __temp0)
}

#[allow(unused_variables)]
fn __action92(input: &str, __0: (usize, TokenType, usize)) -> Expression {
    let __start0 = __0.2.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action35(input, &__start0, &__end0);
    let __temp0 = (__start0, __temp0, __end0);
    __action68(input, __0, __temp0)
}

#[allow(unused_variables)]
fn __action93(input: &str, __0: (usize, TokenType, usize)) -> Expression {
    let __start0 = __0.2.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action35(input, &__start0, &__end0);
    let __temp0 = (__start0, __temp0, __end0);
    __action69(input, __0, __temp0)
}

#[allow(unused_variables)]
fn __action94(input: &str, __0: (usize, TokenType, usize)) -> Expression {
    let __start0 = __0.2.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action35(input, &__start0, &__end0);
    let __temp0 = (__start0, __temp0, __end0);
    __action70(input, __0, __temp0)
}

#[allow(unused_variables)]
fn __action95(input: &str, __0: (usize, TokenType, usize)) -> Expression {
    let __start0 = __0.2.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action35(input, &__start0, &__end0);
    let __temp0 = (__start0, __temp0, __end0);
    __action71(input, __0, __temp0)
}

#[allow(unused_variables)]
fn __action96(input: &str, __0: (usize, TokenType, usize)) -> Expression {
    let __start0 = __0.2.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action35(input, &__start0, &__end0);
    let __temp0 = (__start0, __temp0, __end0);
    __action72(input, __0, __temp0)
}

#[allow(unused_variables)]
fn __action97(input: &str, __0: (usize, TokenType, usize)) -> Expression {
    let __start0 = __0.2.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action35(input, &__start0, &__end0);
    let __temp0 = (__start0, __temp0, __end0);
    __action73(input, __0, __temp0)
}

#[allow(unused_variables)]
fn __action98(
    input: &str,
    __0: (usize, TokenType, usize),
    __1: (usize, Expression, usize),
) -> Expression {
    let __start0 = __0.2.clone();
    let __end0 = __1.0.clone();
    let __temp0 = __action35(input, &__start0, &__end0);
    let __temp0 = (__start0, __temp0, __end0);
    __action74(input, __0, __temp0, __1)
}

#[allow(unused_variables)]
fn __action99(input: &str, __0: (usize, Expression, usize)) -> Vec<Expression> {
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action40(input, __0);
    let __temp0 = (__start0, __temp0, __end0);
    __action49(input, __temp0)
}

#[allow(unused_variables)]
fn __action100(input: &str, __lookbehind: &usize, __lookahead: &usize) -> Vec<Expression> {
    let __start0 = __lookbehind.clone();
    let __end0 = __lookahead.clone();
    let __temp0 = __action41(input, &__start0, &__end0);
    let __temp0 = (__start0, __temp0, __end0);
    __action49(input, __temp0)
}

#[allow(unused_variables)]
fn __action101(
    input: &str,
    __0: (usize, ::std::vec::Vec<Expression>, usize),
    __1: (usize, Expression, usize),
) -> Vec<Expression> {
    let __start0 = __1.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action40(input, __1);
    let __temp0 = (__start0, __temp0, __end0);
    __action50(input, __0, __temp0)
}

#[allow(unused_variables)]
fn __action102(input: &str, __0: (usize, ::std::vec::Vec<Expression>, usize)) -> Vec<Expression> {
    let __start0 = __0.2.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action41(input, &__start0, &__end0);
    let __temp0 = (__start0, __temp0, __end0);
    __action50(input, __0, __temp0)
}

pub trait __ToTriple {
    fn to_triple(
        value: Self,
    ) -> Result<(usize, TokenType, usize), __lalrpop_util::ParseError<usize, TokenType, LexicalError>>;
}

impl __ToTriple for (usize, TokenType, usize) {
    fn to_triple(
        value: Self,
    ) -> Result<(usize, TokenType, usize), __lalrpop_util::ParseError<usize, TokenType, LexicalError>>
    {
        Ok(value)
    }
}
impl __ToTriple for Result<(usize, TokenType, usize), LexicalError> {
    fn to_triple(
        value: Self,
    ) -> Result<(usize, TokenType, usize), __lalrpop_util::ParseError<usize, TokenType, LexicalError>>
    {
        match value {
            Ok(v) => Ok(v),
            Err(error) => Err(__lalrpop_util::ParseError::User { error }),
        }
    }
}
