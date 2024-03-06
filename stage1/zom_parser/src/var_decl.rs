//! Module responsible for parsing expression.
use crate::{expr::Expression, prelude::*, types::Type};

#[repr(u8)]
#[derive(Debug)]
pub enum VarType {
    ConstVar,
    VariableVar,
}

#[derive(Debug)]
pub struct VarDecl {
    pub var_type: VarType,
    pub name: String,
    pub ty: Option<Type>,
    pub expr: Option<Expression>,
    pub span: Range<usize>,
}

impl Parse for VarDecl {
    type Output = Self;

    fn parse(parser: &mut Parser) -> ParsingResult<Self::Output> {
        let mut parsed_tokens = Vec::new();

        let var_type = expect_token!(parser => [T::Const, VarType::ConstVar; T::Var, VarType::VariableVar], [Const, Var], parsed_tokens);
        let start = span_toks!(start parsed_tokens);

        let name = expect_token!(parser => [T::Ident(name), name.clone()], Ident, parsed_tokens);

        let ty = if token_parteq!(parser.last(), T::Colon) {
            expect_token!(parser => [T::Colon, ()], Colon, parsed_tokens);
            Some(parse_try!(parser => Type, parsed_tokens))
        } else {
            None
        };

        let expr = if token_parteq!(parser.last(), T::Oper(Operator::Equal)) {
            expect_token!(parser => [T::Oper(Operator::Equal), ()], T::Oper(Operator::Equal), parsed_tokens);
            Some(parse_try!(parser => Expression, parsed_tokens))
        } else {
            None
        };

        let end = span_toks!(end parsed_tokens);

        Good(
            VarDecl {
                var_type,
                name,
                ty,
                expr,
                span: start..end,
            },
            parsed_tokens,
        )
    }
}
