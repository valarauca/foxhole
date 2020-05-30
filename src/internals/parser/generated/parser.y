%start Expr
%%

Expr -> Result<Expression<'input>,lrpar::Lexeme<u32>>:
      Func { Expression::invoke($1?, Span::into($lexer, $span)) }
    | Num { Expression::num($1?, Span::into($lexer, $span)) }
    | Identifier { Expression::var($1?,Span::into($lexer, $span)) }
    | TemplateVar { Expression::template($1?, Span::into($lexer,$span)) };

Num -> Result<Span<'input>,lrpar::Lexeme<u32>>:
    'NUM' { Ok(Span::new($lexer, None, $span)?) };

Assignment -> Result<Assign<'input>,lrpar::Lexeme<u32>>:
    'LET' Identifier 'EQ' Expr { Assign::new($2?, $4?, Span::into($lexer,$span)) };

Func -> Result<Invoke<'input>,lrpar::Lexeme<u32>>:
    Identifier FuncArgs { Invoke::new($1?, $2?, Span::into($lexer,$span)) };

FuncArgs -> Result<Vec<Expression<'input>>,lrpar::Lexeme<u32>>:
      'LPAR' 'RPAR' { Ok(Vec::new()) }
    | 'LPAR' ArgList 'RPAR' { Ok($2?) };

ArgList -> Result<Vec<Expression<'input>>,lrpar::Lexeme<u32>>:
      ArgList 'COMMA' Expr { let mut v = $1?; v.push($3?); Ok(v) }
    | Expr { Ok( vec![$1?] ) };

Identifier -> Result<Ident<'input>,lrpar::Lexeme<u32>>:
      'IDENT' { Ok( Ident::new( Span::new($lexer, None, $span)? ) ) };

TemplateVar -> Result<Template<'input>,lrpar::Lexeme<u32>>:
      'TEMPLATE_START' Identifier 'TEMPLATE_ASSIGN' TemplateVar 'RBRACE' { Ok(Template::new($2?, Span::new($lexer, None, $span)?, TemplateBehavior::assign($4?))) }
    | 'TEMPLATE_START' Identifier 'TEMPLATE_ASSIGN' 'NUM' 'RBRACE' { Ok(Template::new($2?, Span::new($lexer, None, $span)?, TemplateBehavior::assign(Span::new($lexer, $4, None)?))) }
    | 'TEMPLATE_START' Identifier 'TEMPLATE_FALLBACK' TemplateVar 'RBRACE' { Ok(Template::new($2?, Span::new($lexer, None, $span)?, TemplateBehavior::fallback($4?))) }
    | 'TEMPLATE_START' Identifier 'TEMPLATE_FALLBACK' 'NUM' 'RBRACE' { Ok(Template::new($2?, Span::new($lexer, None, $span)?, TemplateBehavior::fallback(Span::new($lexer, $4, None)?))) }
    | 'TEMPLATE_START' Identifier 'RBRACE' { Ok(Template::new($2?, Span::new($lexer, None, $span)?, None)) };

%%

use crate::internals::parser::span::{Span};
use crate::internals::parser::ast::ident::{Ident};
use crate::internals::parser::ast::invoke::{Invoke};
use crate::internals::parser::ast::expr::{Expression};
use crate::internals::parser::ast::template::{Template,TemplateBehavior,TemplateFallback};
use crate::internals::parser::ast::assign::{Assign};
