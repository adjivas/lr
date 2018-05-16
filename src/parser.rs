// auto-generated: "lalrpop 0.15.2"
// sha256: 6ea628ab9552dd58942a6444161cd1435a596341f8b01f4a288749e7797df
use lexer;
use ast;
use std::str::FromStr;
use lexer::{Tok,Lexer,LexicalError};
use lalrpop_util::ParseError;
#[allow(unused_extern_crates)]
extern crate lalrpop_util as __lalrpop_util;

#[cfg_attr(rustfmt, rustfmt_skip)]
mod __parse__Program {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports, unused_parens)]

    use lexer;
    use ast;
    use std::str::FromStr;
    use lexer::{Tok,Lexer,LexicalError};
    use lalrpop_util::ParseError;
    #[allow(unused_extern_crates)]
    extern crate lalrpop_util as __lalrpop_util;
    use super::__ToTriple;
    #[allow(dead_code)]
    pub enum __Symbol<>
     {
        Variant0(lexer::Tok),
        Variant1(lexer::Pos),
        Variant2(::std::vec::Vec<ast::Stmt>),
        Variant3(ast::Stmt),
    }
    const __ACTION: &'static [i8] = &[
        // State 0
        5, 0, 6, 7,
        // State 1
        0, 0, 0, 0,
        // State 2
        -10, 8, -10, -10,
        // State 3
        5, 0, 6, 7,
        // State 4
        -5, -5, -5, -5,
        // State 5
        0, 0, 10, 0,
        // State 6
        0, 0, 11, 0,
        // State 7
        -4, -4, -4, -4,
        // State 8
        -11, 8, -11, -11,
        // State 9
        -7, -7, -7, -7,
        // State 10
        -6, -6, -6, -6,
    ];
    const __EOF_ACTION: &'static [i8] = &[
        // State 0
        -2,
        // State 1
        -12,
        // State 2
        -10,
        // State 3
        -3,
        // State 4
        -5,
        // State 5
        0,
        // State 6
        0,
        // State 7
        -4,
        // State 8
        -11,
        // State 9
        -7,
        // State 10
        -6,
    ];
    const __GOTO: &'static [i8] = &[
        // State 0
        0, 2, 3, 0, 4, 0,
        // State 1
        0, 0, 0, 0, 0, 0,
        // State 2
        0, 0, 0, 0, 0, 0,
        // State 3
        0, 0, 9, 0, 0, 0,
        // State 4
        0, 0, 0, 0, 0, 0,
        // State 5
        0, 0, 0, 0, 0, 0,
        // State 6
        0, 0, 0, 0, 0, 0,
        // State 7
        0, 0, 0, 0, 0, 0,
        // State 8
        0, 0, 0, 0, 0, 0,
        // State 9
        0, 0, 0, 0, 0, 0,
        // State 10
        0, 0, 0, 0, 0, 0,
    ];
    fn __expected_tokens(__state: usize) -> Vec<::std::string::String> {
        const __TERMINAL: &'static [&'static str] = &[
            r###""+""###,
            r###""/""###,
            r###""\\n""###,
            r###""\\t""###,
        ];
        __ACTION[(__state * 4)..].iter().zip(__TERMINAL).filter_map(|(&state, terminal)| {
            if state == 0 {
                None
            } else {
                Some(terminal.to_string())
            }
        }).collect()
    }
    pub struct ProgramParser {
        _priv: (),
    }

    impl ProgramParser {
        pub fn new() -> ProgramParser {
            ProgramParser {
                _priv: (),
            }
        }

        #[allow(dead_code)]
        pub fn parse<
            __TOKEN: __ToTriple<Error=lexer::LexicalError>,
            __TOKENS: IntoIterator<Item=__TOKEN>,
        >(
            &self,
            __tokens0: __TOKENS,
        ) -> Result<::std::vec::Vec<ast::Stmt>, __lalrpop_util::ParseError<lexer::Pos, lexer::Tok, lexer::LexicalError>>
        {
            let __tokens = __tokens0.into_iter();
            let mut __tokens = __tokens.map(|t| __ToTriple::to_triple(t));
            let mut __states = vec![0_i8];
            let mut __symbols = vec![];
            let mut __integer;
            let mut __lookahead;
            let __last_location = &mut Default::default();
            '__shift: loop {
                __lookahead = match __tokens.next() {
                    Some(Ok(v)) => v,
                    None => break '__shift,
                    Some(Err(e)) => return Err(__lalrpop_util::ParseError::User { error: e }),
                };
                *__last_location = __lookahead.2.clone();
                __integer = match __lookahead.1 {
                    lexer::Tok::Add if true => 0,
                    lexer::Tok::Div if true => 1,
                    lexer::Tok::Linefeed if true => 2,
                    lexer::Tok::Tab if true => 3,
                    _ => {
                        let __state = *__states.last().unwrap() as usize;
                        let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                            token: Some(__lookahead),
                            expected: __expected_tokens(__state),
                        };
                        return Err(__error);
                    }
                };
                '__inner: loop {
                    let __state = *__states.last().unwrap() as usize;
                    let __action = __ACTION[__state * 4 + __integer];
                    if __action > 0 {
                        let __symbol = match __integer {
                            0 => match __lookahead.1 {
                                __tok @ lexer::Tok::Add => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            1 => match __lookahead.1 {
                                __tok @ lexer::Tok::Div => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            2 => match __lookahead.1 {
                                __tok @ lexer::Tok::Linefeed => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            3 => match __lookahead.1 {
                                __tok @ lexer::Tok::Tab => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            _ => unreachable!(),
                        };
                        __states.push(__action - 1);
                        __symbols.push((__lookahead.0, __symbol, __lookahead.2));
                        continue '__shift;
                    } else if __action < 0 {
                        if let Some(r) = __reduce(__action, Some(&__lookahead.0), &mut __states, &mut __symbols, ::std::marker::PhantomData::<()>) {
                            if r.is_err() {
                                return r;
                            }
                            return Err(__lalrpop_util::ParseError::ExtraToken { token: __lookahead });
                        }
                    } else {
                        let mut __err_lookahead = Some(__lookahead);
                        let mut __err_integer: Option<usize> = Some(__integer);
                        let __state = *__states.last().unwrap() as usize;
                        let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                            token: __err_lookahead,
                            expected: __expected_tokens(__state),
                        };
                        return Err(__error)
                    }
                }
            }
            loop {
                let __state = *__states.last().unwrap() as usize;
                let __action = __EOF_ACTION[__state];
                if __action < 0 {
                    if let Some(r) = __reduce(__action, None, &mut __states, &mut __symbols, ::std::marker::PhantomData::<()>) {
                        return r;
                    }
                } else {
                    let mut __err_lookahead = None;
                    let mut __err_integer: Option<usize> = None;
                    let __state = *__states.last().unwrap() as usize;
                    let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                        token: __err_lookahead,
                        expected: __expected_tokens(__state),
                    };
                    return Err(__error)
                }
            }
        }
    }
    pub(crate) fn __reduce<
    >(
        __action: i8,
        __lookahead_start: Option<&lexer::Pos>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(lexer::Pos,__Symbol<>,lexer::Pos)>,
        _: ::std::marker::PhantomData<()>,
    ) -> Option<Result<::std::vec::Vec<ast::Stmt>,__lalrpop_util::ParseError<lexer::Pos, lexer::Tok, lexer::LexicalError>>>
    {
        let (__pop_states, __symbol, __nonterminal) = match -__action {
            1 => {
                __reduce1(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            2 => {
                __reduce2(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            3 => {
                __reduce3(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            4 => {
                // Statement = Statement, "/" => ActionFn(11);
                let __sym1 = __pop_Variant0(__symbols);
                let __sym0 = __pop_Variant3(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = match super::__action11::<>(__sym0, __sym1) {
                    Ok(v) => v,
                    Err(e) => return Some(Err(e)),
                };
                let __symbol = (__start, __Symbol::Variant3(__nt), __end);
                (2, __symbol, 2)
            }
            5 => {
                __reduce5(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            6 => {
                __reduce6(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            7 => {
                __reduce7(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            8 => {
                __reduce8(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            9 => {
                __reduce9(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            10 => {
                __reduce10(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            11 => {
                __reduce11(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            12 => {
                // __Program = Program => ActionFn(0);
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
        __symbols.push(__symbol);
        let __state = *__states.last().unwrap() as usize;
        let __next_state = __GOTO[__state * 6 + __nonterminal] - 1;
        __states.push(__next_state);
        None
    }
    fn __pop_Variant3<
    >(
        __symbols: &mut ::std::vec::Vec<(lexer::Pos,__Symbol<>,lexer::Pos)>
    ) -> (lexer::Pos, ast::Stmt, lexer::Pos)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant3(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant1<
    >(
        __symbols: &mut ::std::vec::Vec<(lexer::Pos,__Symbol<>,lexer::Pos)>
    ) -> (lexer::Pos, lexer::Pos, lexer::Pos)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant1(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant0<
    >(
        __symbols: &mut ::std::vec::Vec<(lexer::Pos,__Symbol<>,lexer::Pos)>
    ) -> (lexer::Pos, lexer::Tok, lexer::Pos)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant0(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant2<
    >(
        __symbols: &mut ::std::vec::Vec<(lexer::Pos,__Symbol<>,lexer::Pos)>
    ) -> (lexer::Pos, ::std::vec::Vec<ast::Stmt>, lexer::Pos)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant2(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    pub(crate) fn __reduce1<
    >(
        __action: i8,
        __lookahead_start: Option<&lexer::Pos>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(lexer::Pos,__Symbol<>,lexer::Pos)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (lexer::Pos,__Symbol<>,lexer::Pos), usize)
    {
        // @L =  => ActionFn(6);
        let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
        let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
        let __nt = super::__action6::<>(&__start, &__end);
        let __symbol = (__start, __Symbol::Variant1(__nt), __end);
        (0, __symbol, 0)
    }
    pub(crate) fn __reduce2<
    >(
        __action: i8,
        __lookahead_start: Option<&lexer::Pos>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(lexer::Pos,__Symbol<>,lexer::Pos)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (lexer::Pos,__Symbol<>,lexer::Pos), usize)
    {
        // Program =  => ActionFn(12);
        let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
        let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
        let __nt = super::__action12::<>(&__start, &__end);
        let __symbol = (__start, __Symbol::Variant2(__nt), __end);
        (0, __symbol, 1)
    }
    pub(crate) fn __reduce3<
    >(
        __action: i8,
        __lookahead_start: Option<&lexer::Pos>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(lexer::Pos,__Symbol<>,lexer::Pos)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (lexer::Pos,__Symbol<>,lexer::Pos), usize)
    {
        // Program = Statement+ => ActionFn(13);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action13::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant2(__nt), __end);
        (1, __symbol, 1)
    }
    pub(crate) fn __reduce5<
    >(
        __action: i8,
        __lookahead_start: Option<&lexer::Pos>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(lexer::Pos,__Symbol<>,lexer::Pos)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (lexer::Pos,__Symbol<>,lexer::Pos), usize)
    {
        // Statement = "+" => ActionFn(3);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action3::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant3(__nt), __end);
        (1, __symbol, 2)
    }
    pub(crate) fn __reduce6<
    >(
        __action: i8,
        __lookahead_start: Option<&lexer::Pos>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(lexer::Pos,__Symbol<>,lexer::Pos)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (lexer::Pos,__Symbol<>,lexer::Pos), usize)
    {
        // Statement = "\\t", "\\n" => ActionFn(4);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action4::<>(__sym0, __sym1);
        let __symbol = (__start, __Symbol::Variant3(__nt), __end);
        (2, __symbol, 2)
    }
    pub(crate) fn __reduce7<
    >(
        __action: i8,
        __lookahead_start: Option<&lexer::Pos>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(lexer::Pos,__Symbol<>,lexer::Pos)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (lexer::Pos,__Symbol<>,lexer::Pos), usize)
    {
        // Statement = "\\n", "\\n" => ActionFn(5);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action5::<>(__sym0, __sym1);
        let __symbol = (__start, __Symbol::Variant3(__nt), __end);
        (2, __symbol, 2)
    }
    pub(crate) fn __reduce8<
    >(
        __action: i8,
        __lookahead_start: Option<&lexer::Pos>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(lexer::Pos,__Symbol<>,lexer::Pos)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (lexer::Pos,__Symbol<>,lexer::Pos), usize)
    {
        // Statement* =  => ActionFn(7);
        let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
        let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
        let __nt = super::__action7::<>(&__start, &__end);
        let __symbol = (__start, __Symbol::Variant2(__nt), __end);
        (0, __symbol, 3)
    }
    pub(crate) fn __reduce9<
    >(
        __action: i8,
        __lookahead_start: Option<&lexer::Pos>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(lexer::Pos,__Symbol<>,lexer::Pos)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (lexer::Pos,__Symbol<>,lexer::Pos), usize)
    {
        // Statement* = Statement+ => ActionFn(8);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action8::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant2(__nt), __end);
        (1, __symbol, 3)
    }
    pub(crate) fn __reduce10<
    >(
        __action: i8,
        __lookahead_start: Option<&lexer::Pos>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(lexer::Pos,__Symbol<>,lexer::Pos)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (lexer::Pos,__Symbol<>,lexer::Pos), usize)
    {
        // Statement+ = Statement => ActionFn(9);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action9::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant2(__nt), __end);
        (1, __symbol, 4)
    }
    pub(crate) fn __reduce11<
    >(
        __action: i8,
        __lookahead_start: Option<&lexer::Pos>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(lexer::Pos,__Symbol<>,lexer::Pos)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (lexer::Pos,__Symbol<>,lexer::Pos), usize)
    {
        // Statement+ = Statement+, Statement => ActionFn(10);
        let __sym1 = __pop_Variant3(__symbols);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action10::<>(__sym0, __sym1);
        let __symbol = (__start, __Symbol::Variant2(__nt), __end);
        (2, __symbol, 4)
    }
}
pub use self::__parse__Program::ProgramParser;

fn __action0<
>(
    (_, __0, _): (lexer::Pos, ::std::vec::Vec<ast::Stmt>, lexer::Pos),
) -> ::std::vec::Vec<ast::Stmt>
{
    (__0)
}

fn __action1<
>(
    (_, __0, _): (lexer::Pos, ::std::vec::Vec<ast::Stmt>, lexer::Pos),
) -> ::std::vec::Vec<ast::Stmt>
{
    (__0)
}

fn __action2<
>(
    (_, l, _): (lexer::Pos, lexer::Pos, lexer::Pos),
    (_, _, _): (lexer::Pos, ast::Stmt, lexer::Pos),
    (_, _, _): (lexer::Pos, lexer::Tok, lexer::Pos),
) -> Result<ast::Stmt,__lalrpop_util::ParseError<lexer::Pos,lexer::Tok,lexer::LexicalError>>
{
    Err(ParseError::User { error: LexicalError::Other2(l) })
}

fn __action3<
>(
    (_, __0, _): (lexer::Pos, lexer::Tok, lexer::Pos),
) -> ast::Stmt
{
    ast::Stmt::Add
}

fn __action4<
>(
    (_, __0, _): (lexer::Pos, lexer::Tok, lexer::Pos),
    (_, __1, _): (lexer::Pos, lexer::Tok, lexer::Pos),
) -> ast::Stmt
{
    ast::Stmt::Return
}

fn __action5<
>(
    (_, __0, _): (lexer::Pos, lexer::Tok, lexer::Pos),
    (_, __1, _): (lexer::Pos, lexer::Tok, lexer::Pos),
) -> ast::Stmt
{
    ast::Stmt::Exit
}

fn __action6<
>(
    __lookbehind: &lexer::Pos,
    __lookahead: &lexer::Pos,
) -> lexer::Pos
{
    __lookahead.clone()
}

fn __action7<
>(
    __lookbehind: &lexer::Pos,
    __lookahead: &lexer::Pos,
) -> ::std::vec::Vec<ast::Stmt>
{
    vec![]
}

fn __action8<
>(
    (_, v, _): (lexer::Pos, ::std::vec::Vec<ast::Stmt>, lexer::Pos),
) -> ::std::vec::Vec<ast::Stmt>
{
    v
}

fn __action9<
>(
    (_, __0, _): (lexer::Pos, ast::Stmt, lexer::Pos),
) -> ::std::vec::Vec<ast::Stmt>
{
    vec![__0]
}

fn __action10<
>(
    (_, v, _): (lexer::Pos, ::std::vec::Vec<ast::Stmt>, lexer::Pos),
    (_, e, _): (lexer::Pos, ast::Stmt, lexer::Pos),
) -> ::std::vec::Vec<ast::Stmt>
{
    { let mut v = v; v.push(e); v }
}

fn __action11<
>(
    __0: (lexer::Pos, ast::Stmt, lexer::Pos),
    __1: (lexer::Pos, lexer::Tok, lexer::Pos),
) -> Result<ast::Stmt,__lalrpop_util::ParseError<lexer::Pos,lexer::Tok,lexer::LexicalError>>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action6(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action2(
        __temp0,
        __0,
        __1,
    )
}

fn __action12<
>(
    __lookbehind: &lexer::Pos,
    __lookahead: &lexer::Pos,
) -> ::std::vec::Vec<ast::Stmt>
{
    let __start0 = __lookbehind.clone();
    let __end0 = __lookahead.clone();
    let __temp0 = __action7(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action1(
        __temp0,
    )
}

fn __action13<
>(
    __0: (lexer::Pos, ::std::vec::Vec<ast::Stmt>, lexer::Pos),
) -> ::std::vec::Vec<ast::Stmt>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action8(
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action1(
        __temp0,
    )
}

pub trait __ToTriple<> {
    type Error;
    fn to_triple(value: Self) -> Result<(lexer::Pos,lexer::Tok,lexer::Pos),Self::Error>;
}

impl<> __ToTriple<> for (lexer::Pos, lexer::Tok, lexer::Pos) {
    type Error = lexer::LexicalError;
    fn to_triple(value: Self) -> Result<(lexer::Pos,lexer::Tok,lexer::Pos),lexer::LexicalError> {
        Ok(value)
    }
}
impl<> __ToTriple<> for Result<(lexer::Pos, lexer::Tok, lexer::Pos),lexer::LexicalError> {
    type Error = lexer::LexicalError;
    fn to_triple(value: Self) -> Result<(lexer::Pos,lexer::Tok,lexer::Pos),lexer::LexicalError> {
        value
    }
}
