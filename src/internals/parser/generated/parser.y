%start Body
%left 'ADD'
%left 'SUB'
%left 'MUL'
%left 'DIV'
%left 'EQ'
%left 'NE'
%left 'GT'
%left 'LT'
%left 'GE'
%left 'LE'
%left 'AND'
%left 'OR'
%left 'XOR'
%%

Body -> Result<Vec<Statement<'input>>,lrpar::Lexeme<u32>>:
      Sttmnts Term { {let mut v = $1?; v.push($2?); Ok(v)} }
    | Term { Ok(vec![$1?]) };

Sttmnts -> Result<Vec<Statement<'input>>,lrpar::Lexeme<u32>>:
      Sttmnts Sttmnt { {let mut v = $1?; v.push($2?); Ok(v) } }
    | Sttmnt { Ok(vec![$1?]) };

Sttmnt -> Result<Statement<'input>,lrpar::Lexeme<u32>>:
    Assignment 'SEMI' { Statement::new($1?, Span::into($lexer,$span)) };

Term -> Result<Statement<'input>,lrpar::Lexeme<u32>>:
    Expr { Statement::new($1?, Span::into($lexer,$span)) };

Assignment -> Result<Assign<'input>,lrpar::Lexeme<u32>>:
    'LET' Identifier 'ASSIGN' Expr { Assign::new($2?, $4?, Span::into($lexer,$span)) };

Expr -> Result<Expression<'input>,lrpar::Lexeme<u32>>:
      Expr 'ADD' Expr    { Expression::new(Operation::new($1?,Op::ADD,$3?,Span::into($lexer,$span))?, Span::into($lexer,$span)) }
    | Expr 'SUB' Expr    { Expression::new(Operation::new($1?,Op::SUB,$3?,Span::into($lexer,$span))?, Span::into($lexer,$span)) }
    | Expr 'MUL' Expr    { Expression::new(Operation::new($1?,Op::MUL,$3?,Span::into($lexer,$span))?, Span::into($lexer,$span)) }
    | Expr 'DIV' Expr    { Expression::new(Operation::new($1?,Op::DIV,$3?,Span::into($lexer,$span))?, Span::into($lexer,$span)) }
    | Expr 'EQ' Expr     { Expression::new(Operation::new($1?,Op::EQ ,$3?,Span::into($lexer,$span))?, Span::into($lexer,$span)) }
    | Expr 'NE' Expr     { Expression::new(Operation::new($1?,Op::NE ,$3?,Span::into($lexer,$span))?, Span::into($lexer,$span)) }
    | Expr 'GT' Expr     { Expression::new(Operation::new($1?,Op::GT ,$3?,Span::into($lexer,$span))?, Span::into($lexer,$span)) }
    | Expr 'LT' Expr     { Expression::new(Operation::new($1?,Op::LT ,$3?,Span::into($lexer,$span))?, Span::into($lexer,$span)) }
    | Expr 'GE' Expr     { Expression::new(Operation::new($1?,Op::GE ,$3?,Span::into($lexer,$span))?, Span::into($lexer,$span)) }
    | Expr 'LE' Expr     { Expression::new(Operation::new($1?,Op::LE ,$3?,Span::into($lexer,$span))?, Span::into($lexer,$span)) }
    | Expr 'AND' Expr    { Expression::new(Operation::new($1?,Op::AND,$3?,Span::into($lexer,$span))?, Span::into($lexer,$span)) }
    | Expr 'OR' Expr     { Expression::new(Operation::new($1?,Op::OR ,$3?,Span::into($lexer,$span))?, Span::into($lexer,$span)) }
    | Expr 'XOR' Expr    { Expression::new(Operation::new($1?,Op::XOR,$3?,Span::into($lexer,$span))?, Span::into($lexer,$span)) }
    | 'LPAR' Expr 'RPAR' { Expression::new($2?, Span::into($lexer,$span)) }
    | Func               { Expression::new($1?, Span::into($lexer,$span)) }
    | Num                { Expression::new($1?, Span::into($lexer,$span)) }
    | Identifier         { Expression::new($1?, Span::into($lexer,$span)) }
    | TemplateVar        { Expression::new($1?, Span::into($lexer,$span)) };

Num -> Result<Span<'input>,lrpar::Lexeme<u32>>:
    'NUM' { Ok(Span::new($lexer, None, $span)?) };

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
use crate::internals::parser::ast::template::{Template,TemplateBehavior};
use crate::internals::parser::ast::assign::{Assign};
use crate::internals::parser::ast::op::{Op,Operation};
use crate::internals::parser::ast::statement::{Statement};
