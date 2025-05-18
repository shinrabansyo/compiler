mod gen;

use sb_compiler_parse_ast::Program;
use sb_compiler_lirgen_ir::LirTopElem;
use sb_compiler_utils::collections::LayeredTable;

const ZERO_REG: u32 = 0;
const RET_REG: u32 = 10;
const FARG_REG_BASE: u32 = 10;

#[derive(Debug, Clone)]
pub(crate) struct GenContext {
    reserved_regs: u32,
    reserved_labels: u32,
    fn_name: Option<(String, String)>,
    sym_table: LayeredTable<String, u32>,
}

impl Default for GenContext {
    fn default() -> Self {
        Self {
            reserved_regs: 20,   // r0: ゼロレジスタ, r10 ~ r19: 引数レジスタ として確保済み
            reserved_labels: 0,
            fn_name: None,
            sym_table: LayeredTable::default(),
        }
    }
}

impl GenContext {
    fn alloc_reg(&mut self) -> u32 {
        let allocated_reg = self.reserved_regs;
        self.reserved_regs += 1;
        allocated_reg
    }

    fn alloc_label(&mut self) -> u32 {
        let allocated_label = self.reserved_labels;
        self.reserved_labels += 1;
        allocated_label
    }

    fn set_fn_name(&mut self, namespace: String, name: String) {
        self.fn_name = Some((namespace, name));
    }

    fn reset_fn_name(&mut self) {
        self.fn_name = None;
    }

    fn get_fn_name(&self) -> Option<(String, String)> {
        self.fn_name.clone()
    }
}

pub fn lirgen<'ast>(program: &'ast Program) -> Vec<LirTopElem> {
    let mut ctx = GenContext::default();
    gen::lirgen_program(&mut ctx, program)
}
