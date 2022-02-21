%start ProgramParser
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

ProgramParser -> Result<Body,lrpar::Lexeme<u32>>:
      SttmntsColl { Body::new($1?, Span::into($lexer,$span)) }; 

/*
 * Statement handling
 *
 */ 

SttmntsColl -> Result<Vec<Statement>,lrpar::Lexeme<u32>>:
      Sttmnts Term { {let mut v = $1?; v.push($2?); Ok(v)} }
    | Term         { Ok(vec![$1?]) };

Sttmnts -> Result<Vec<Statement>,lrpar::Lexeme<u32>>:
      Sttmnts Sttmnt { {let mut v = $1?; v.push($2?); Ok(v) } }
    | Sttmnt         { Ok(vec![$1?]) };

Sttmnt -> Result<Statement,lrpar::Lexeme<u32>>:
      Assignment 'SEMI' { Statement::new($1?, Span::into($lexer,$span)) }
    | DecCmp 'SEMI'     { Statement::new($1?, Span::into($lexer,$span)) }
    | DecFunc           { Statement::new($1?, Span::into($lexer,$span)) };

Term -> Result<Statement,lrpar::Lexeme<u32>>:
    Expr { Statement::new($1?, Span::into($lexer,$span)) };

/*
 * Expressions
 *
 */

Expr -> Result<Expression,lrpar::Lexeme<u32>>:
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
    | Cond               { Expression::new($1?, Span::into($lexer,$span)) }
    | Func               { Expression::new($1?, Span::into($lexer,$span)) }
    | Num                { Expression::new($1?, Span::into($lexer,$span)) }
    | Identifier         { Expression::new($1?, Span::into($lexer,$span)) }
    | TemplateVar        { Expression::new($1?, Span::into($lexer,$span)) };

/*
 * Conditionals
 *
 */
Cond -> Result<Conditional,lrpar::Lexeme<u32>>:
    'IF' Expr 'RBRACE' Expr 'LBRACE' 'ELSE' 'RBRACE' Expr 'LBRACE' { Conditional::new($2?, $4?, $8?, Span::into($lexer, $span)) };

/*
 * Variable Assignments
 *
 */

Assignment -> Result<Assign,lrpar::Lexeme<u32>>:
      'CONST' Identifier 'COLON' TypeInfo 'EQ' Expr { Assign::new($2?, true, $4?, $6?, Span::into($lexer,$span)) }
    | 'CONST' Identifier 'ASSIGN' Expr              { Assign::new($2?, true, None, $4?, Span::into($lexer,$span)) } 
    | 'LET' Identifier 'COLON' TypeInfo 'EQ' Expr   { Assign::new($2?, false, $4?, $6?, Span::into($lexer,$span)) }
    | 'LET' Identifier 'ASSIGN' Expr                { Assign::new($2?, false, None, $4?, Span::into($lexer,$span)) };

/*
 * Declaring Functions
 *
 */

DecFuncArg -> Result<FunctionArg,lrpar::Lexeme<u32>>:
    Identifier 'COLON' TypeInfo { FunctionArg::new($1?,$3?,Span::into($lexer,$span)) };

FuncArgDecList -> Result<Vec<FunctionArg>,lrpar::Lexeme<u32>>:
      FuncArgDecList 'COMMA' DecFuncArg { let mut v = $1?; v.push($3?); Ok(v) }
    | DecFuncArg                        { Ok(vec![$1?]) };

DecFuncArgs -> Result<Vec<FunctionArg>,lrpar::Lexeme<u32>>:
      'LPAR' 'RPAR' { Ok(Vec::new()) }
    | 'LPAR' FuncArgDecList 'RPAR' { Ok($2?) };

DecFunc -> Result<FunctionDec,lrpar::Lexeme<u32>>:
    'FN' Identifier DecFuncArgs TypeInfo 'RBRACE' SttmntsColl 'LBRACE' { FunctionDec::new($2?,$3?,$6?,$4?,Span::into($lexer,$span)) };

/*
 * Invoking Functions
 *
 */

Func -> Result<Invoke,lrpar::Lexeme<u32>>:
    Identifier FuncArgs { Invoke::new($1?, $2?, Span::into($lexer,$span)) };

FuncArgs -> Result<Vec<Expression>,lrpar::Lexeme<u32>>:
      'LPAR' 'RPAR' { Ok(Vec::new()) }
    | 'LPAR' ArgList 'RPAR' { Ok($2?) };

ArgList -> Result<Vec<Expression>,lrpar::Lexeme<u32>>:
      ArgList 'COMMA' Expr { let mut v = $1?; v.push($3?); Ok(v) }
    | Expr { Ok( vec![$1?] ) };


/*
 * Compositional Functions (homomorphisms & monads)
 *
 */

DecCmp -> Result<CompositionalFunction,lrpar::Lexeme<u32>>:
    'COMP' Identifier 'LPAR' CompArg 'COMMA' CompArg 'COMMA' CompArg 'RPAR' TypeInfo { CompositionalFunction::new($2?,$4?,$6?,$8?,$10?,Span::into($lexer,$span)) };

CompArg -> Result<CompositionalFunctionArg,lrpar::Lexeme<u32>>:
      Bool        { CompositionalFunctionArg::new($1?,Span::into($lexer,$span)) }
    | Num         { CompositionalFunctionArg::new($1?,Span::into($lexer,$span)) }
    | TemplateVar { CompositionalFunctionArg::new($1?,Span::into($lexer,$span)) }
    | Identifier  { CompositionalFunctionArg::new($1?,Span::into($lexer,$span)) }
    | 'ADD'       { CompositionalFunctionArg::new(Op::ADD,Span::into($lexer,$span)) }
    | 'SUB'       { CompositionalFunctionArg::new(Op::SUB,Span::into($lexer,$span)) }
    | 'MUL'       { CompositionalFunctionArg::new(Op::MUL,Span::into($lexer,$span)) }
    | 'AND'       { CompositionalFunctionArg::new(Op::AND,Span::into($lexer,$span)) }
    | 'OR'        { CompositionalFunctionArg::new(Op::OR,Span::into($lexer,$span))  }
    | 'XOR'       { CompositionalFunctionArg::new(Op::XOR,Span::into($lexer,$span)) };

/*
 * Primatives. Numbers, Booleans, Templates, Identifiers, and Types.
 * Very simple things
 *
 */

Num -> Result<Span,lrpar::Lexeme<u32>>:
    'NUM' { Ok(Span::new($lexer, None, $span)?) };

Bool -> Result<Span,lrpar::Lexeme<u32>>:
      'TRUE'  { Ok(Span::new($lexer,None,$span)?) }
    | 'FALSE' { Ok(Span::new($lexer,None,$span)?) };

Identifier -> Result<Ident,lrpar::Lexeme<u32>>:
      'IDENT' { Ok( Ident::new( Span::new($lexer, None, $span)? ) ) };

TemplateVar -> Result<Template,lrpar::Lexeme<u32>>:
      'TEMPLATE_START' Identifier 'TEMPLATE_ASSIGN' TemplateVar 'RBRACE' { Ok(Template::new($2?, Span::new($lexer, None, $span)?, TemplateBehavior::assign($4?))) }
    | 'TEMPLATE_START' Identifier 'TEMPLATE_ASSIGN' 'NUM' 'RBRACE' { Ok(Template::new($2?, Span::new($lexer, None, $span)?, TemplateBehavior::assign(Span::new($lexer, $4, None)?))) }
    | 'TEMPLATE_START' Identifier 'TEMPLATE_FALLBACK' TemplateVar 'RBRACE' { Ok(Template::new($2?, Span::new($lexer, None, $span)?, TemplateBehavior::fallback($4?))) }
    | 'TEMPLATE_START' Identifier 'TEMPLATE_FALLBACK' 'NUM' 'RBRACE' { Ok(Template::new($2?, Span::new($lexer, None, $span)?, TemplateBehavior::fallback(Span::new($lexer, $4, None)?))) }
    | 'TEMPLATE_START' Identifier 'RBRACE' { Ok(Template::new($2?, Span::new($lexer, None, $span)?, None)) };

TypeInfo -> Result<Kind,lrpar::Lexeme<u32>>:
      'INT'      { Ok(Kind::Int) }
    | 'BOOL'     { Ok(Kind::Bool) } 
    | 'VEC_INT'  { Ok(Kind::CollOfInt) }
    | 'VEC_BOOL' { Ok(Kind::CollOfBool) };

%%

use crate::internals::parser::span::{Span};
use crate::internals::parser::ast::kind::{Kind};
use crate::internals::parser::ast::args::{FunctionArg};
use crate::internals::parser::ast::func::{FunctionDec};
use crate::internals::parser::ast::comparg::{CompositionalFunctionArg,CompositionalFunction};
use crate::internals::parser::ast::ident::{Ident};
use crate::internals::parser::ast::invoke::{Invoke};
use crate::internals::parser::ast::expr::{Expression};
use crate::internals::parser::ast::template::{Template,TemplateBehavior};
use crate::internals::parser::ast::assign::{Assign};
use crate::internals::parser::ast::op::{Op,Operation};
use crate::internals::parser::ast::statement::{Statement,Body};
use crate::internals::parser::ast::condition::{Conditional};
