mod gen;

use sb_compiler_lirgen_ir::LirTopElem;
use sb_compiler_semcheck_hir::Program;
use sb_compiler_semcheck_impl_vardecl::Var;
use sb_compiler_utils::collections::LayeredTable;

const ZERO_REG: u32 = 0;
const RET_REG: u32 = 10;
const FARG_REG_BASE: u32 = 10;

#[derive(Debug, Clone)]
pub(crate) struct GenContext<'input> {
    reserved_regs: u32,
    reserved_labels: u32,
    var_table: LayeredTable<Var<'input>, u32>,
}

impl<'input> Default for GenContext<'input> {
    fn default() -> Self {
        GenContext {
            reserved_regs: 20,   // r0: ゼロレジスタ, r10 ~ r19: 引数レジスタ として確保済み
            reserved_labels: 0,
            var_table: LayeredTable::default(),
        }
    }
}

impl<'input> GenContext<'input> {
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

    fn set_var_reg(&mut self, var: Var<'input>, reg: u32) {
        self.var_table.insert(var, reg);
    }

    fn ref_var_reg(&self, var: &Var) -> Option<u32> {
        self.var_table.get(&var).map(|v| *v)
    }
}

pub fn lirgen(program: &Program) -> Vec<LirTopElem> {
    let mut ctx = GenContext::default();
    gen::lirgen_program(&mut ctx, program)
}
