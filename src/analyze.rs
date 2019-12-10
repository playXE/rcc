use crate::data::prelude::*;

type Error = Locatable<String>;

struct Analyzer<I: Iterator<Item = Stmt>> {
    stmts: I,
    errors: Vec<Error>,
}

pub(crate) fn analyze(decl: &Declaration) {
    unimplemented!();
}

// TODO: make this private
pub(crate) mod typecheck {
    use super::Error;
    use crate::data::prelude::*;
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
