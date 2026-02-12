#[macro_export]
macro_rules! lir {
    // <let> ::= "let" <ident> "=" <inst> ;
    (let $dst:ident = $syntax:expr ; $($remain:tt)*) => {{
        let ($dst, stmt) = Assign::new($syntax);
        let next = lir!($($remain)*);
        LirSyntaxNode::wrap(stmt).chain(next)
    }};

    // <stmt> ::= <inst> ;
    ($syntax:expr ; $($remain:tt)*) => {{
        let (_, stmt) = Assign::new($syntax);
        let next = lir!($($remain)*);
        LirSyntaxNode::wrap(stmt).chain(next)
    }};

    // EOF
    () => { Terminal };
}
