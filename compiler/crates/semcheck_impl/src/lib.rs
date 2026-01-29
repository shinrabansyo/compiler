use sb_compiler_semcheck_impl_var::VarContext;
use sb_compiler_semcheck_impl_type::TypeContext;
use sb_compiler_utils::str::PartedString;

#[derive(Clone)]
pub struct SemCheckContext<'src> {
    pub name: PartedString,
    pub var: VarContext<'src>,
    pub r#type: TypeContext<'src>,
}

impl<'src> SemCheckContext<'src> {
    pub fn new() -> Self {
        SemCheckContext {
            name: PartedString::new::<64>(),
            var: VarContext::new(),
            r#type: TypeContext::new(),
        }
    }
}
