mod parser_y {
    #![allow(clippy::type_complexity)]

    #[allow(dead_code)]
    pub fn parse<'lexer, 'input: 'lexer>(lexer: &'lexer dyn ::lrpar::NonStreamingLexer<'input, u32>)
          -> (::std::option::Option<Result<Template<'input>,lrpar::Lexeme<u32>>>, ::std::vec::Vec<::lrpar::LexParseError<u32>>)
    {
        let (grm, sgraph, stable) = ::lrpar::ctbuilder::_reconstitute(include_bytes!("src/internals/parser/generated/parser.grm"),
            include_bytes!("src/internals/parser/generated/parser.sgraph"),
            include_bytes!("src/internals/parser/generated/parser.stable"));
        #[allow(clippy::type_complexity)]
        let mut actions: ::std::vec::Vec<&dyn Fn(::cfgrammar::RIdx<u32>,
                       &'lexer dyn ::lrpar::NonStreamingLexer<'input, u32>,
                       ::lrpar::Span,
                       ::std::vec::Drain<::lrpar::parser::AStackType<__GTActionsKind<'input>, u32>>)
                    -> __GTActionsKind<'input>> = ::std::vec::Vec::new();
        actions.push(&__gt_wrapper_0);
        actions.push(&__gt_wrapper_1);
        actions.push(&__gt_wrapper_2);
        actions.push(&__gt_wrapper_3);
        actions.push(&__gt_wrapper_4);
        actions.push(&__gt_wrapper_5);
        actions.push(&__gt_wrapper_6);

        match ::lrpar::RTParserBuilder::new(&grm, &sgraph, &stable)
            .recoverer(::lrpar::RecoveryKind::None)
            .parse_actions(lexer, &actions) {
                (Some(__GTActionsKind::AK2(x)), y) => (Some(x), y),
                (None, y) => (None, y),
                _ => unreachable!()
        }
    }

    #[allow(dead_code)]
    pub const R_IDENTIFIER: u32 = 1;
    #[allow(dead_code)]
    pub const R_TEMPLATEVAR: u32 = 2;
    const __GT_EPP: &[::std::option::Option<&str>] = &[Some("IDENT"), Some("TEMPLATE_START"), Some("TEMPLATE_ASSIGN"), Some("RBRACE"), Some("NUM"), Some("TEMPLATE_FALLBACK"), None];

    /// Return the %epp entry for token `tidx` (where `None` indicates "the token has no
    /// pretty-printed value"). Panics if `tidx` doesn't exist.
    #[allow(dead_code)]
    pub fn token_epp<'a>(tidx: ::cfgrammar::TIdx<u32>) -> ::std::option::Option<&'a str> {
        __GT_EPP[usize::from(tidx)]
    }

    // Wrappers

    fn __gt_wrapper_0<'lexer, 'input: 'lexer>(__gt_ridx: ::cfgrammar::RIdx<u32>,
                      __gt_lexer: &'lexer dyn ::lrpar::NonStreamingLexer<'input, u32>,
                      __gt_span: ::lrpar::Span,
                      mut __gt_args: ::std::vec::Drain<::lrpar::parser::AStackType<__GTActionsKind<'input>, u32>>)
                   -> __GTActionsKind<'input> {
        let __gt_arg_1 = match __gt_args.next().unwrap() {
            ::lrpar::parser::AStackType::Lexeme(l) => {
                if l.inserted() {
                    Err(l)
                } else {
                    Ok(l)
                }
            },
            ::lrpar::parser::AStackType::ActionType(_) => unreachable!()
        };
        __GTActionsKind::AK1(__gt_action_0(__gt_ridx, __gt_lexer, __gt_span, __gt_arg_1))
    }

    fn __gt_wrapper_1<'lexer, 'input: 'lexer>(__gt_ridx: ::cfgrammar::RIdx<u32>,
                      __gt_lexer: &'lexer dyn ::lrpar::NonStreamingLexer<'input, u32>,
                      __gt_span: ::lrpar::Span,
                      mut __gt_args: ::std::vec::Drain<::lrpar::parser::AStackType<__GTActionsKind<'input>, u32>>)
                   -> __GTActionsKind<'input> {
        let __gt_arg_1 = match __gt_args.next().unwrap() {
            ::lrpar::parser::AStackType::Lexeme(l) => {
                if l.inserted() {
                    Err(l)
                } else {
                    Ok(l)
                }
            },
            ::lrpar::parser::AStackType::ActionType(_) => unreachable!()
        };
        let __gt_arg_2 = match __gt_args.next().unwrap() {
            ::lrpar::parser::AStackType::ActionType(__GTActionsKind::AK1(x)) => x,
            _ => unreachable!()
        };
        let __gt_arg_3 = match __gt_args.next().unwrap() {
            ::lrpar::parser::AStackType::Lexeme(l) => {
                if l.inserted() {
                    Err(l)
                } else {
                    Ok(l)
                }
            },
            ::lrpar::parser::AStackType::ActionType(_) => unreachable!()
        };
        let __gt_arg_4 = match __gt_args.next().unwrap() {
            ::lrpar::parser::AStackType::ActionType(__GTActionsKind::AK2(x)) => x,
            _ => unreachable!()
        };
        let __gt_arg_5 = match __gt_args.next().unwrap() {
            ::lrpar::parser::AStackType::Lexeme(l) => {
                if l.inserted() {
                    Err(l)
                } else {
                    Ok(l)
                }
            },
            ::lrpar::parser::AStackType::ActionType(_) => unreachable!()
        };
        __GTActionsKind::AK2(__gt_action_1(__gt_ridx, __gt_lexer, __gt_span, __gt_arg_1, __gt_arg_2, __gt_arg_3, __gt_arg_4, __gt_arg_5))
    }

    fn __gt_wrapper_2<'lexer, 'input: 'lexer>(__gt_ridx: ::cfgrammar::RIdx<u32>,
                      __gt_lexer: &'lexer dyn ::lrpar::NonStreamingLexer<'input, u32>,
                      __gt_span: ::lrpar::Span,
                      mut __gt_args: ::std::vec::Drain<::lrpar::parser::AStackType<__GTActionsKind<'input>, u32>>)
                   -> __GTActionsKind<'input> {
        let __gt_arg_1 = match __gt_args.next().unwrap() {
            ::lrpar::parser::AStackType::Lexeme(l) => {
                if l.inserted() {
                    Err(l)
                } else {
                    Ok(l)
                }
            },
            ::lrpar::parser::AStackType::ActionType(_) => unreachable!()
        };
        let __gt_arg_2 = match __gt_args.next().unwrap() {
            ::lrpar::parser::AStackType::ActionType(__GTActionsKind::AK1(x)) => x,
            _ => unreachable!()
        };
        let __gt_arg_3 = match __gt_args.next().unwrap() {
            ::lrpar::parser::AStackType::Lexeme(l) => {
                if l.inserted() {
                    Err(l)
                } else {
                    Ok(l)
                }
            },
            ::lrpar::parser::AStackType::ActionType(_) => unreachable!()
        };
        let __gt_arg_4 = match __gt_args.next().unwrap() {
            ::lrpar::parser::AStackType::Lexeme(l) => {
                if l.inserted() {
                    Err(l)
                } else {
                    Ok(l)
                }
            },
            ::lrpar::parser::AStackType::ActionType(_) => unreachable!()
        };
        let __gt_arg_5 = match __gt_args.next().unwrap() {
            ::lrpar::parser::AStackType::Lexeme(l) => {
                if l.inserted() {
                    Err(l)
                } else {
                    Ok(l)
                }
            },
            ::lrpar::parser::AStackType::ActionType(_) => unreachable!()
        };
        __GTActionsKind::AK2(__gt_action_2(__gt_ridx, __gt_lexer, __gt_span, __gt_arg_1, __gt_arg_2, __gt_arg_3, __gt_arg_4, __gt_arg_5))
    }

    fn __gt_wrapper_3<'lexer, 'input: 'lexer>(__gt_ridx: ::cfgrammar::RIdx<u32>,
                      __gt_lexer: &'lexer dyn ::lrpar::NonStreamingLexer<'input, u32>,
                      __gt_span: ::lrpar::Span,
                      mut __gt_args: ::std::vec::Drain<::lrpar::parser::AStackType<__GTActionsKind<'input>, u32>>)
                   -> __GTActionsKind<'input> {
        let __gt_arg_1 = match __gt_args.next().unwrap() {
            ::lrpar::parser::AStackType::Lexeme(l) => {
                if l.inserted() {
                    Err(l)
                } else {
                    Ok(l)
                }
            },
            ::lrpar::parser::AStackType::ActionType(_) => unreachable!()
        };
        let __gt_arg_2 = match __gt_args.next().unwrap() {
            ::lrpar::parser::AStackType::ActionType(__GTActionsKind::AK1(x)) => x,
            _ => unreachable!()
        };
        let __gt_arg_3 = match __gt_args.next().unwrap() {
            ::lrpar::parser::AStackType::Lexeme(l) => {
                if l.inserted() {
                    Err(l)
                } else {
                    Ok(l)
                }
            },
            ::lrpar::parser::AStackType::ActionType(_) => unreachable!()
        };
        let __gt_arg_4 = match __gt_args.next().unwrap() {
            ::lrpar::parser::AStackType::ActionType(__GTActionsKind::AK2(x)) => x,
            _ => unreachable!()
        };
        let __gt_arg_5 = match __gt_args.next().unwrap() {
            ::lrpar::parser::AStackType::Lexeme(l) => {
                if l.inserted() {
                    Err(l)
                } else {
                    Ok(l)
                }
            },
            ::lrpar::parser::AStackType::ActionType(_) => unreachable!()
        };
        __GTActionsKind::AK2(__gt_action_3(__gt_ridx, __gt_lexer, __gt_span, __gt_arg_1, __gt_arg_2, __gt_arg_3, __gt_arg_4, __gt_arg_5))
    }

    fn __gt_wrapper_4<'lexer, 'input: 'lexer>(__gt_ridx: ::cfgrammar::RIdx<u32>,
                      __gt_lexer: &'lexer dyn ::lrpar::NonStreamingLexer<'input, u32>,
                      __gt_span: ::lrpar::Span,
                      mut __gt_args: ::std::vec::Drain<::lrpar::parser::AStackType<__GTActionsKind<'input>, u32>>)
                   -> __GTActionsKind<'input> {
        let __gt_arg_1 = match __gt_args.next().unwrap() {
            ::lrpar::parser::AStackType::Lexeme(l) => {
                if l.inserted() {
                    Err(l)
                } else {
                    Ok(l)
                }
            },
            ::lrpar::parser::AStackType::ActionType(_) => unreachable!()
        };
        let __gt_arg_2 = match __gt_args.next().unwrap() {
            ::lrpar::parser::AStackType::ActionType(__GTActionsKind::AK1(x)) => x,
            _ => unreachable!()
        };
        let __gt_arg_3 = match __gt_args.next().unwrap() {
            ::lrpar::parser::AStackType::Lexeme(l) => {
                if l.inserted() {
                    Err(l)
                } else {
                    Ok(l)
                }
            },
            ::lrpar::parser::AStackType::ActionType(_) => unreachable!()
        };
        let __gt_arg_4 = match __gt_args.next().unwrap() {
            ::lrpar::parser::AStackType::Lexeme(l) => {
                if l.inserted() {
                    Err(l)
                } else {
                    Ok(l)
                }
            },
            ::lrpar::parser::AStackType::ActionType(_) => unreachable!()
        };
        let __gt_arg_5 = match __gt_args.next().unwrap() {
            ::lrpar::parser::AStackType::Lexeme(l) => {
                if l.inserted() {
                    Err(l)
                } else {
                    Ok(l)
                }
            },
            ::lrpar::parser::AStackType::ActionType(_) => unreachable!()
        };
        __GTActionsKind::AK2(__gt_action_4(__gt_ridx, __gt_lexer, __gt_span, __gt_arg_1, __gt_arg_2, __gt_arg_3, __gt_arg_4, __gt_arg_5))
    }

    fn __gt_wrapper_5<'lexer, 'input: 'lexer>(__gt_ridx: ::cfgrammar::RIdx<u32>,
                      __gt_lexer: &'lexer dyn ::lrpar::NonStreamingLexer<'input, u32>,
                      __gt_span: ::lrpar::Span,
                      mut __gt_args: ::std::vec::Drain<::lrpar::parser::AStackType<__GTActionsKind<'input>, u32>>)
                   -> __GTActionsKind<'input> {
        let __gt_arg_1 = match __gt_args.next().unwrap() {
            ::lrpar::parser::AStackType::Lexeme(l) => {
                if l.inserted() {
                    Err(l)
                } else {
                    Ok(l)
                }
            },
            ::lrpar::parser::AStackType::ActionType(_) => unreachable!()
        };
        let __gt_arg_2 = match __gt_args.next().unwrap() {
            ::lrpar::parser::AStackType::ActionType(__GTActionsKind::AK1(x)) => x,
            _ => unreachable!()
        };
        let __gt_arg_3 = match __gt_args.next().unwrap() {
            ::lrpar::parser::AStackType::Lexeme(l) => {
                if l.inserted() {
                    Err(l)
                } else {
                    Ok(l)
                }
            },
            ::lrpar::parser::AStackType::ActionType(_) => unreachable!()
        };
        __GTActionsKind::AK2(__gt_action_5(__gt_ridx, __gt_lexer, __gt_span, __gt_arg_1, __gt_arg_2, __gt_arg_3))
    }

    fn __gt_wrapper_6<'lexer, 'input: 'lexer>(__gt_ridx: ::cfgrammar::RIdx<u32>,
                      __gt_lexer: &'lexer dyn ::lrpar::NonStreamingLexer<'input, u32>,
                      __gt_span: ::lrpar::Span,
                      mut __gt_args: ::std::vec::Drain<::lrpar::parser::AStackType<__GTActionsKind<'input>, u32>>)
                   -> __GTActionsKind<'input> {    unreachable!()
    }

    #[allow(dead_code)]
    enum __GTActionsKind<'input> {
        AK1(Result<Ident<'input>,lrpar::Lexeme<u32>>),
        AK2(Result<Template<'input>,lrpar::Lexeme<u32>>),
    ___GTActionsKindHidden(::std::marker::PhantomData<&'input ()>)
    }


// User code from the program section

use crate::internals::parser::span::{Span};
use crate::internals::parser::ast::ident::{Ident};
use crate::internals::parser::ast::template::{Template,TemplateBehavior,TemplateFallback};

    // User actions

    // Identifier
    #[allow(clippy::too_many_arguments)]
    fn __gt_action_0<'lexer, 'input: 'lexer>(__gt_ridx: ::cfgrammar::RIdx<u32>,
                     __gt_lexer: &'lexer dyn ::lrpar::NonStreamingLexer<'input, u32>,
                     __gt_span: ::lrpar::Span,
                     mut __gt_arg_1: ::std::result::Result<::lrpar::Lexeme<u32>, ::lrpar::Lexeme<u32>>) 
->                  Result<Ident<'input>,lrpar::Lexeme<u32>> {
Ok( Ident::new( Span::new(__gt_lexer, None, __gt_span)? ) )
    }

    // TemplateVar
    #[allow(clippy::too_many_arguments)]
    fn __gt_action_1<'lexer, 'input: 'lexer>(__gt_ridx: ::cfgrammar::RIdx<u32>,
                     __gt_lexer: &'lexer dyn ::lrpar::NonStreamingLexer<'input, u32>,
                     __gt_span: ::lrpar::Span,
                     mut __gt_arg_1: ::std::result::Result<::lrpar::Lexeme<u32>, ::lrpar::Lexeme<u32>>,
                     mut __gt_arg_2: Result<Ident<'input>,lrpar::Lexeme<u32>>,
                     mut __gt_arg_3: ::std::result::Result<::lrpar::Lexeme<u32>, ::lrpar::Lexeme<u32>>,
                     mut __gt_arg_4: Result<Template<'input>,lrpar::Lexeme<u32>>,
                     mut __gt_arg_5: ::std::result::Result<::lrpar::Lexeme<u32>, ::lrpar::Lexeme<u32>>) 
->                  Result<Template<'input>,lrpar::Lexeme<u32>> {
Ok(Template::new(__gt_arg_2?, Span::new(__gt_lexer, None, __gt_span)?, TemplateBehavior::assign(__gt_arg_4?)))
    }

    // TemplateVar
    #[allow(clippy::too_many_arguments)]
    fn __gt_action_2<'lexer, 'input: 'lexer>(__gt_ridx: ::cfgrammar::RIdx<u32>,
                     __gt_lexer: &'lexer dyn ::lrpar::NonStreamingLexer<'input, u32>,
                     __gt_span: ::lrpar::Span,
                     mut __gt_arg_1: ::std::result::Result<::lrpar::Lexeme<u32>, ::lrpar::Lexeme<u32>>,
                     mut __gt_arg_2: Result<Ident<'input>,lrpar::Lexeme<u32>>,
                     mut __gt_arg_3: ::std::result::Result<::lrpar::Lexeme<u32>, ::lrpar::Lexeme<u32>>,
                     mut __gt_arg_4: ::std::result::Result<::lrpar::Lexeme<u32>, ::lrpar::Lexeme<u32>>,
                     mut __gt_arg_5: ::std::result::Result<::lrpar::Lexeme<u32>, ::lrpar::Lexeme<u32>>) 
->                  Result<Template<'input>,lrpar::Lexeme<u32>> {
Ok(Template::new(__gt_arg_2?, Span::new(__gt_lexer, None, __gt_span)?, TemplateBehavior::assign(Span::new(__gt_lexer, __gt_arg_4, None)?)))
    }

    // TemplateVar
    #[allow(clippy::too_many_arguments)]
    fn __gt_action_3<'lexer, 'input: 'lexer>(__gt_ridx: ::cfgrammar::RIdx<u32>,
                     __gt_lexer: &'lexer dyn ::lrpar::NonStreamingLexer<'input, u32>,
                     __gt_span: ::lrpar::Span,
                     mut __gt_arg_1: ::std::result::Result<::lrpar::Lexeme<u32>, ::lrpar::Lexeme<u32>>,
                     mut __gt_arg_2: Result<Ident<'input>,lrpar::Lexeme<u32>>,
                     mut __gt_arg_3: ::std::result::Result<::lrpar::Lexeme<u32>, ::lrpar::Lexeme<u32>>,
                     mut __gt_arg_4: Result<Template<'input>,lrpar::Lexeme<u32>>,
                     mut __gt_arg_5: ::std::result::Result<::lrpar::Lexeme<u32>, ::lrpar::Lexeme<u32>>) 
->                  Result<Template<'input>,lrpar::Lexeme<u32>> {
Ok(Template::new(__gt_arg_2?, Span::new(__gt_lexer, None, __gt_span)?, TemplateBehavior::fallback(__gt_arg_4?)))
    }

    // TemplateVar
    #[allow(clippy::too_many_arguments)]
    fn __gt_action_4<'lexer, 'input: 'lexer>(__gt_ridx: ::cfgrammar::RIdx<u32>,
                     __gt_lexer: &'lexer dyn ::lrpar::NonStreamingLexer<'input, u32>,
                     __gt_span: ::lrpar::Span,
                     mut __gt_arg_1: ::std::result::Result<::lrpar::Lexeme<u32>, ::lrpar::Lexeme<u32>>,
                     mut __gt_arg_2: Result<Ident<'input>,lrpar::Lexeme<u32>>,
                     mut __gt_arg_3: ::std::result::Result<::lrpar::Lexeme<u32>, ::lrpar::Lexeme<u32>>,
                     mut __gt_arg_4: ::std::result::Result<::lrpar::Lexeme<u32>, ::lrpar::Lexeme<u32>>,
                     mut __gt_arg_5: ::std::result::Result<::lrpar::Lexeme<u32>, ::lrpar::Lexeme<u32>>) 
->                  Result<Template<'input>,lrpar::Lexeme<u32>> {
Ok(Template::new(__gt_arg_2?, Span::new(__gt_lexer, None, __gt_span)?, TemplateBehavior::fallback(Span::new(__gt_lexer, __gt_arg_4, None)?)))
    }

    // TemplateVar
    #[allow(clippy::too_many_arguments)]
    fn __gt_action_5<'lexer, 'input: 'lexer>(__gt_ridx: ::cfgrammar::RIdx<u32>,
                     __gt_lexer: &'lexer dyn ::lrpar::NonStreamingLexer<'input, u32>,
                     __gt_span: ::lrpar::Span,
                     mut __gt_arg_1: ::std::result::Result<::lrpar::Lexeme<u32>, ::lrpar::Lexeme<u32>>,
                     mut __gt_arg_2: Result<Ident<'input>,lrpar::Lexeme<u32>>,
                     mut __gt_arg_3: ::std::result::Result<::lrpar::Lexeme<u32>, ::lrpar::Lexeme<u32>>) 
->                  Result<Template<'input>,lrpar::Lexeme<u32>> {
Ok(Template::new(__gt_arg_2?, Span::new(__gt_lexer, None, __gt_span)?, None))
    }

}


/* CACHE INFORMATION
   Build time: "2020-05-17T20:41:52.173128659+00:00"
   Mod name: None
   Recoverer: None
   YaccKind: Some(Grmtools)
   Error on conflicts: true
   0 'IDENT'
   1 'TEMPLATE_START'
   2 'TEMPLATE_ASSIGN'
   3 'RBRACE'
   4 'NUM'
   5 'TEMPLATE_FALLBACK'
   6 <unknown>
*/
