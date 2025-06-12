mod gen;

use sb_compiler_parse_ast::Program;
use sb_compiler_lirgen_ir::LirTopElem;
use sb_compiler_utils::collections::LayeredTable;

const ZERO_REG: u32 = 0;
const RET_REG: u32 = 10;
const FARG_REG_BASE: u32 = 10;

#[derive(Debug, Clone)]
pub(crate) struct GenContext<'a> {
    reserved_regs: u32,
    reserved_labels: u32,
    sym_table: LayeredTable<&'a str, u32>,
}

impl<'a> Default for GenContext<'a> {
    fn default() -> Self {
        Self {
            reserved_regs: 20,   // r0: ゼロレジスタ, r10 ~ r19: 引数レジスタ として確保済み
            reserved_labels: 0,
            sym_table: LayeredTable::default(),
        }
    }
}

impl<'a> GenContext<'a> {
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

    fn set_var_reg(&mut self, var: &'a str, reg: u32) {
        self.sym_table.insert(var, reg);
    }

    fn ref_var_reg(&self, var: &str) -> Option<u32> {
        self.sym_table.get(&var).map(|v| *v)
    }
}

pub fn lirgen(program: &Program) -> Vec<LirTopElem> {
    let mut ctx = GenContext::default();
    gen::lirgen_program(&mut ctx, program)
}
