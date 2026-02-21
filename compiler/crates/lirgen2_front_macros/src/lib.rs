#[macro_export]
macro_rules! lir {
    // <let> ::= "let" <ident> "=" <inst> ;
    (let $dst:ident = $rhs:expr ; $($remain:tt)*) => {{
        let ($dst, stmt) = Assign::new($rhs);
        let next = lir!($($remain)*);
        Lir::wrap(stmt).chain(next)
    }};

    // <stmt> ::= <inst> ;
    ($stmt:expr ; $($remain:tt)*) => {{
        let (_, stmt) = Assign::new($stmt);
        let next = lir!($($remain)*);
        Lir::wrap(stmt).chain(next)
    }};

    // EOF
    () => { Terminal };
}
