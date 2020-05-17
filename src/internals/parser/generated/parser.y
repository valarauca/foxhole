%start TemplateVar
%%

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
use crate::internals::parser::ast::template::{Template,TemplateBehavior,TemplateFallback};
