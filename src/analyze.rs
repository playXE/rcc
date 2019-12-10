use crate::data::prelude::*;

type Error = Locatable<String>;

struct Analyzer<I: Iterator<Item = Stmt>> {
    stmts: I,
    errors: Vec<Error>,
}

pub(crate) fn analyze(decl: &Declaration) -> Result<(), Error> {
    unimplemented!();
}

// TODO: make this private
pub(crate) mod typecheck {
    use super::Error;
    use crate::data::prelude::*;

    pub(crate) fn typeck(expr: Expr) -> RecoverableResult<Expr, Error> {
        match expr.expr {
            _ => Ok(expr),
        }
    }

    pub(crate) fn integer_binary_op(
        left: Expr,
        right: Expr,
        token: &Locatable<Token>,
    ) -> RecoverableResult<(Expr, Expr), Error> {
        if let Err(err) = ensure_integer(&left, &right, &token.data, token.location) {
            return Err((err, (left, right)));
        }
        Expr::binary_promote(left, right)
    }

    // TODO: make this private
    pub(crate) fn ensure_integer(
        left: &Expr,
        right: &Expr,
        token: &Token,
        location: Location,
    ) -> Result<(), Error> {
        let err = |ctype| {
            Err(Locatable::new(
                format!(
                    "expected integer on both sides of '{}', got '{}'",
                    token, ctype
                ),
                location,
            ))
        };
        match (left.ctype.is_integral(), right.ctype.is_integral()) {
            (false, _) => err(&left.ctype),
            (_, false) => err(&right.ctype),
            (true, true) => Ok(()),
        }
    }
}
