use crate::lexer::*;
use crate::ast::*;
#[allow(unused_extern_crates)]
extern crate lalrpop_util as __lalrpop_util;
#[allow(unused_imports)]
use self::__lalrpop_util::state_machine as __state_machine;

#[cfg_attr(rustfmt, rustfmt_skip)]
mod __parse__SourceUnit {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports, unused_parens)]

    use crate::lexer::*;
    use crate::ast::*;
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
        Variant2(SourceUnit),
        Variant3(SourceUnitPart),
        Variant4(::std::vec::Vec<SourceUnitPart>),
        Variant5(Statement),
    }
    const __ACTION: &'static [i8] = &[
        // State 0
        11, 0, 0, 0, 0, 0, 0, 0, 12, 13, 14, 0, 15,
        // State 1
        0, -6, 0, -6, -6, 0, -6, 0, 0, 0, 0, 0, 0,
        // State 2
        -16, 0, 0, 0, 0, 0, 0, 0, -16, -16, -16, 0, -16,
        // State 3
        0, 0, 0, 16, 17, 0, 18, 0, 0, 0, 0, 0, 0,
        // State 4
        0, -3, 19, -3, -3, 20, -3, 0, 0, 0, 0, 0, 0,
        // State 5
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 6
        -13, 0, 0, 0, 0, 0, 0, 0, -13, -13, -13, 0, -13,
        // State 7
        11, 0, 0, 0, 0, 0, 0, 0, 12, 13, 14, 0, 15,
        // State 8
        -12, 0, 0, 0, 0, 0, 0, 0, -12, -12, -12, 0, -12,
        // State 9
        0, -9, -9, -9, -9, -9, -9, 0, 0, 0, 0, 0, 0,
        // State 10
        11, 0, 0, 0, 0, 0, 0, 0, 0, 23, 14, 0, 0,
        // State 11
        0, 0, 0, 0, 0, 0, 0, 0, 0, 24, 0, 0, 0,
        // State 12
        0, 0, -20, -20, -20, -20, -20, 25, 0, 0, 0, 0, 0,
        // State 13
        0, -19, -19, -19, -19, -19, -19, 0, 0, 0, 0, 0, 0,
        // State 14
        11, 0, 0, 0, 0, 0, 0, 0, 0, 23, 14, 27, 0,
        // State 15
        11, 0, 0, 0, 0, 0, 0, 0, 0, 23, 14, 0, 0,
        // State 16
        11, 0, 0, 0, 0, 0, 0, 0, 0, 23, 14, 0, 0,
        // State 17
        -11, 0, 0, 0, 0, 0, 0, 0, -11, -11, -11, 0, -11,
        // State 18
        11, 0, 0, 0, 0, 0, 0, 0, 0, 23, 14, 0, 0,
        // State 19
        11, 0, 0, 0, 0, 0, 0, 0, 0, 23, 14, 0, 0,
        // State 20
        -14, 0, 0, 0, 0, 0, 0, 0, -14, -14, -14, 0, -14,
        // State 21
        0, 32, 0, 16, 17, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 22
        0, -20, -20, -20, -20, -20, -20, 0, 0, 0, 0, 0, 0,
        // State 23
        0, 0, 0, 0, 0, 0, 33, 0, 0, 0, 0, 0, 0,
        // State 24
        11, 0, 0, 0, 0, 0, 0, 0, 0, 23, 14, 35, 0,
        // State 25
        0, 0, 0, 16, 17, 0, 36, 0, 0, 0, 0, 0, 0,
        // State 26
        0, 0, 0, 0, 0, 0, 37, 0, 0, 0, 0, 0, 0,
        // State 27
        0, -1, 19, -1, -1, 20, -1, 0, 0, 0, 0, 0, 0,
        // State 28
        0, -2, 19, -2, -2, 20, -2, 0, 0, 0, 0, 0, 0,
        // State 29
        0, -7, -7, -7, -7, -7, -7, 0, 0, 0, 0, 0, 0,
        // State 30
        0, -8, -8, -8, -8, -8, -8, 0, 0, 0, 0, 0, 0,
        // State 31
        0, -21, -21, -21, -21, -21, -21, 0, 0, 0, 0, 0, 0,
        // State 32
        -15, 0, 0, 0, 0, 0, 0, 0, -15, -15, -15, 0, -15,
        // State 33
        0, 0, 0, 16, 17, 0, 38, 0, 0, 0, 0, 0, 0,
        // State 34
        0, 0, 0, 0, 0, 0, 39, 0, 0, 0, 0, 0, 0,
        // State 35
        -17, 0, 0, 0, 0, 0, 0, 0, -17, -17, -17, 0, -17,
        // State 36
        -18, 0, 0, 0, 0, 0, 0, 0, -18, -18, -18, 0, -18,
        // State 37
        -5, 0, 0, 0, 0, 0, 0, 0, -5, -5, -5, 0, -5,
        // State 38
        -4, 0, 0, 0, 0, 0, 0, 0, -4, -4, -4, 0, -4,
    ];
    const __EOF_ACTION: &'static [i8] = &[
        // State 0
        0,
        // State 1
        0,
        // State 2
        -16,
        // State 3
        0,
        // State 4
        0,
        // State 5
        -22,
        // State 6
        -13,
        // State 7
        -10,
        // State 8
        -12,
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
        -11,
        // State 18
        0,
        // State 19
        0,
        // State 20
        -14,
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
        0,
        // State 31
        0,
        // State 32
        -15,
        // State 33
        0,
        // State 34
        0,
        // State 35
        -17,
        // State 36
        -18,
        // State 37
        -5,
        // State 38
        -4,
    ];
    const __GOTO: &'static [i8] = &[
        // State 0
        2, 3, 4, 5, 6, 7, 8, 9, 10, 0,
        // State 1
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 2
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 3
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 4
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 5
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 6
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 7
        2, 3, 4, 5, 0, 21, 0, 9, 10, 0,
        // State 8
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 9
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 10
        2, 0, 22, 5, 0, 0, 0, 0, 10, 0,
        // State 11
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 12
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 13
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 14
        2, 0, 26, 5, 0, 0, 0, 0, 10, 0,
        // State 15
        0, 0, 0, 28, 0, 0, 0, 0, 10, 0,
        // State 16
        0, 0, 0, 29, 0, 0, 0, 0, 10, 0,
        // State 17
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 18
        0, 0, 0, 0, 0, 0, 0, 0, 30, 0,
        // State 19
        0, 0, 0, 0, 0, 0, 0, 0, 31, 0,
        // State 20
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 21
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 22
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 23
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 24
        2, 0, 34, 5, 0, 0, 0, 0, 10, 0,
        // State 25
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 26
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 27
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 28
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 29
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 30
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 31
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 32
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 33
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 34
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 35
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 36
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 37
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 38
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    fn __expected_tokens(__state: usize) -> Vec<::std::string::String> {
        const __TERMINAL: &'static [&'static str] = &[
            r###""(""###,
            r###"")""###,
            r###""*""###,
            r###""+""###,
            r###""-""###,
            r###""/""###,
            r###"";""###,
            r###""=""###,
            r###"Declaration"###,
            r###"Identifier"###,
            r###"Integer"###,
            r###"StringLiteral"###,
            r###"Write"###,
        ];
        __ACTION[(__state * 13)..].iter().zip(__TERMINAL).filter_map(|(&state, terminal)| {
            if state == 0 {
                None
            } else {
                Some(terminal.to_string())
            }
        }).collect()
    }
    pub struct __StateMachine<>
    where 
    {
        __phantom: ::std::marker::PhantomData<()>,
    }
    impl<> __state_machine::ParserDefinition for __StateMachine<>
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
            __ACTION[(state as usize) * 13 + integer]
        }

        #[inline]
        fn error_action(&self, state: i8) -> i8 {
            __ACTION[(state as usize) * 13 + (13 - 1)]
        }

        #[inline]
        fn eof_action(&self, state: i8) -> i8 {
            __EOF_ACTION[state as usize]
        }

        #[inline]
        fn goto(&self, state: i8, nt: usize) -> i8 {
            __GOTO[(state as usize) * 10 + nt] - 1
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
        match __token {
            TokenType::OpenParantheses if true => Some(0),
            TokenType::CloseParantheses if true => Some(1),
            TokenType::Product if true => Some(2),
            TokenType::Plus if true => Some(3),
            TokenType::Minus if true => Some(4),
            TokenType::Divide if true => Some(5),
            TokenType::SemiColon if true => Some(6),
            TokenType::Assignment if true => Some(7),
            TokenType::Declaration if true => Some(8),
            TokenType::Symbol(String) if true => Some(9),
            TokenType::Number(i64) if true => Some(10),
            TokenType::Literal(String) if true => Some(11),
            TokenType::Write if true => Some(12),
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
            0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 => __Symbol::Variant0(__token),
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
            __tokens0: __TOKENS,
        ) -> Result<SourceUnit, __lalrpop_util::ParseError<usize, TokenType, LexicalError>>
        {
            let __tokens = __tokens0.into_iter();
            let mut __tokens = __tokens.map(|t| __ToTriple::to_triple(t));
            let __r = __state_machine::Parser::drive(
                __StateMachine {
                    __phantom: ::std::marker::PhantomData::<()>,
                },
                __tokens,
            );
            __r
        }
    }
    pub(crate) fn __reduce<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> Option<Result<SourceUnit,__lalrpop_util::ParseError<usize, TokenType, LexicalError>>>
    {
        let (__pop_states, __nonterminal) = match __action {
            0 => {
                __reduce0(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            1 => {
                __reduce1(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            2 => {
                __reduce2(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            3 => {
                __reduce3(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            4 => {
                __reduce4(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            5 => {
                __reduce5(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            6 => {
                __reduce6(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            7 => {
                __reduce7(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            8 => {
                __reduce8(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            9 => {
                __reduce9(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            10 => {
                __reduce10(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            11 => {
                __reduce11(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            12 => {
                __reduce12(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            13 => {
                __reduce13(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            14 => {
                __reduce14(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            15 => {
                __reduce15(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            16 => {
                __reduce16(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            17 => {
                __reduce17(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            18 => {
                __reduce18(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            19 => {
                __reduce19(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            20 => {
                __reduce20(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            21 => {
                // __SourceUnit = SourceUnit => ActionFn(0);
                let __sym0 = __pop_Variant2(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action0::<>(__sym0);
                return Some(Ok(__nt));
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __states_len = __states.len();
        __states.truncate(__states_len - __pop_states);
        let __state = *__states.last().unwrap() as usize;
        let __next_state = __GOTO[__state * 10 + __nonterminal] - 1;
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
    fn __pop_Variant2<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, SourceUnit, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant2(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant3<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, SourceUnitPart, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant3(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant5<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Statement, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant5(__v), __r) => (__l, __v, __r),
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
    ) -> (usize, ::std::vec::Vec<SourceUnitPart>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant4(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    pub(crate) fn __reduce0<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ArithExpression = Expression, "+", Factor => ActionFn(11);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant1(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action11::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (3, 0)
    }
    pub(crate) fn __reduce1<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ArithExpression = Expression, "-", Factor => ActionFn(12);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant1(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action12::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (3, 0)
    }
    pub(crate) fn __reduce2<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ArithExpression = Factor => ActionFn(13);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action13::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 0)
    }
    pub(crate) fn __reduce3<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Assignment = Identifier, "=", StringLiteral, ";" => ActionFn(8);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action8::<>(__sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (4, 1)
    }
    pub(crate) fn __reduce4<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Assignment = Identifier, "=", Expression, ";" => ActionFn(9);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant1(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action9::<>(__sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (4, 1)
    }
    pub(crate) fn __reduce5<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expression = ArithExpression => ActionFn(10);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action10::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 2)
    }
    pub(crate) fn __reduce6<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Factor = Factor, "*", Term => ActionFn(14);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant1(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action14::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (3, 3)
    }
    pub(crate) fn __reduce7<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Factor = Factor, "/", Term => ActionFn(15);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant1(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action15::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (3, 3)
    }
    pub(crate) fn __reduce8<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Factor = Term => ActionFn(16);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action16::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 3)
    }
    pub(crate) fn __reduce9<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // SourceUnit = SourceUnitPart+ => ActionFn(1);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action1::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (1, 4)
    }
    pub(crate) fn __reduce10<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // SourceUnitPart = Expression, ";" => ActionFn(2);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action2::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (2, 5)
    }
    pub(crate) fn __reduce11<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // SourceUnitPart = Statement => ActionFn(3);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action3::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 5)
    }
    pub(crate) fn __reduce12<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // SourceUnitPart+ = SourceUnitPart => ActionFn(20);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action20::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 6)
    }
    pub(crate) fn __reduce13<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // SourceUnitPart+ = SourceUnitPart+, SourceUnitPart => ActionFn(21);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant3(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action21::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (2, 6)
    }
    pub(crate) fn __reduce14<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Statement = Declaration, Identifier, ";" => ActionFn(4);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action4::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (3, 7)
    }
    pub(crate) fn __reduce15<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Statement = Assignment => ActionFn(5);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action5::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 7)
    }
    pub(crate) fn __reduce16<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Statement = Write, Expression, ";" => ActionFn(6);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action6::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (3, 7)
    }
    pub(crate) fn __reduce17<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Statement = Write, StringLiteral, ";" => ActionFn(7);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action7::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (3, 7)
    }
    pub(crate) fn __reduce18<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Term = Integer => ActionFn(17);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action17::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 8)
    }
    pub(crate) fn __reduce19<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Term = Identifier => ActionFn(18);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action18::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 8)
    }
    pub(crate) fn __reduce20<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Term = "(", Expression, ")" => ActionFn(19);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action19::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (3, 8)
    }
}
pub use self::__parse__SourceUnit::SourceUnitParser;

fn __action0<
>(
    (_, __0, _): (usize, SourceUnit, usize),
) -> SourceUnit
{
    __0
}

fn __action1<
>(
    (_, __0, _): (usize, ::std::vec::Vec<SourceUnitPart>, usize),
) -> SourceUnit
{
    SourceUnit(__0)
}

fn __action2<
>(
    (_, e, _): (usize, Expression, usize),
    (_, _, _): (usize, TokenType, usize),
) -> SourceUnitPart
{
    SourceUnitPart::Expression(e)
}

fn __action3<
>(
    (_, __0, _): (usize, Statement, usize),
) -> SourceUnitPart
{
    SourceUnitPart::Statement(__0)
}

fn __action4<
>(
    (_, _, _): (usize, TokenType, usize),
    (_, id, _): (usize, TokenType, usize),
    (_, _, _): (usize, TokenType, usize),
) -> Statement
{
    Statement::Declaration(id)
}

fn __action5<
>(
    (_, __0, _): (usize, Expression, usize),
) -> Statement
{
    Statement::Allocation(__0)
}

fn __action6<
>(
    (_, _, _): (usize, TokenType, usize),
    (_, e, _): (usize, Expression, usize),
    (_, _, _): (usize, TokenType, usize),
) -> Statement
{
    Statement::WriteExpr(e)
}

fn __action7<
>(
    (_, _, _): (usize, TokenType, usize),
    (_, s, _): (usize, TokenType, usize),
    (_, _, _): (usize, TokenType, usize),
) -> Statement
{
    Statement::WriteString(s)
}

fn __action8<
>(
    (_, l, _): (usize, TokenType, usize),
    (_, _, _): (usize, TokenType, usize),
    (_, r, _): (usize, TokenType, usize),
    (_, _, _): (usize, TokenType, usize),
) -> Expression
{
    Expression::StringAlloc(l,r)
}

fn __action9<
>(
    (_, l, _): (usize, TokenType, usize),
    (_, _, _): (usize, TokenType, usize),
    (_, r, _): (usize, Expression, usize),
    (_, _, _): (usize, TokenType, usize),
) -> Expression
{
    Expression::Assignment(l,Box::new(r))
}

fn __action10<
>(
    (_, __0, _): (usize, Expression, usize),
) -> Expression
{
    __0
}

fn __action11<
>(
    (_, l, _): (usize, Expression, usize),
    (_, _, _): (usize, TokenType, usize),
    (_, r, _): (usize, Expression, usize),
) -> Expression
{
    Expression::Add(Box::new(l),Box::new(r))
}

fn __action12<
>(
    (_, l, _): (usize, Expression, usize),
    (_, _, _): (usize, TokenType, usize),
    (_, r, _): (usize, Expression, usize),
) -> Expression
{
    Expression::Subtract(Box::new(l),Box::new(r))
}

fn __action13<
>(
    (_, __0, _): (usize, Expression, usize),
) -> Expression
{
    __0
}

fn __action14<
>(
    (_, l, _): (usize, Expression, usize),
    (_, _, _): (usize, TokenType, usize),
    (_, r, _): (usize, Expression, usize),
) -> Expression
{
    Expression::Multiply(Box::new(l),Box::new(r))
}

fn __action15<
>(
    (_, l, _): (usize, Expression, usize),
    (_, _, _): (usize, TokenType, usize),
    (_, r, _): (usize, Expression, usize),
) -> Expression
{
    Expression::Divide(Box::new(l),Box::new(r))
}

fn __action16<
>(
    (_, __0, _): (usize, Expression, usize),
) -> Expression
{
    __0
}

fn __action17<
>(
    (_, v, _): (usize, TokenType, usize),
) -> Expression
{
    Expression::Integer(v)
}

fn __action18<
>(
    (_, id, _): (usize, TokenType, usize),
) -> Expression
{
    Expression::Symbol(id)
}

fn __action19<
>(
    (_, _, _): (usize, TokenType, usize),
    (_, __0, _): (usize, Expression, usize),
    (_, _, _): (usize, TokenType, usize),
) -> Expression
{
    __0
}

fn __action20<
>(
    (_, __0, _): (usize, SourceUnitPart, usize),
) -> ::std::vec::Vec<SourceUnitPart>
{
    vec![__0]
}

fn __action21<
>(
    (_, v, _): (usize, ::std::vec::Vec<SourceUnitPart>, usize),
    (_, e, _): (usize, SourceUnitPart, usize),
) -> ::std::vec::Vec<SourceUnitPart>
{
    { let mut v = v; v.push(e); v }
}

pub trait __ToTriple<> {
    fn to_triple(value: Self) -> Result<(usize,TokenType,usize), __lalrpop_util::ParseError<usize, TokenType, LexicalError>>;
}

impl<> __ToTriple<> for (usize, TokenType, usize) {
    fn to_triple(value: Self) -> Result<(usize,TokenType,usize), __lalrpop_util::ParseError<usize, TokenType, LexicalError>> {
        Ok(value)
    }
}
impl<> __ToTriple<> for Result<(usize, TokenType, usize), LexicalError> {
    fn to_triple(value: Self) -> Result<(usize,TokenType,usize), __lalrpop_util::ParseError<usize, TokenType, LexicalError>> {
        match value {
            Ok(v) => Ok(v),
            Err(error) => Err(__lalrpop_util::ParseError::User { error }),
        }
    }
}
