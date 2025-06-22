mod gen;

use string_interner::symbol::SymbolU32;

use sb_compiler_lirgen_ir::LirTopElem;
use sb_compiler_semcheck_hir::Program;
use sb_compiler_utils::collections::LayeredTable;

const ZERO_REG: u32 = 0;
const RET_REG: u32 = 10;
const FARG_REG_BASE: u32 = 10;

#[derive(Debug, Clone)]
pub(crate) struct GenContext {
    reserved_regs: u32,
    reserved_labels: u32,
    var_table: LayeredTable<SymbolU32, u32>,
}

impl Default for GenContext{
    fn default() -> Self {
        GenContext {
            reserved_regs: 20,   // r0: ゼロレジスタ, r10 ~ r19: 引数レジスタ として確保済み
            reserved_labels: 0,
            var_table: LayeredTable::default(),
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

    fn set_var_reg(&mut self, symbol: SymbolU32, reg: u32) {
        self.var_table.insert(symbol, reg);
    }

    fn ref_var_reg(&self, symbol: &SymbolU32) -> Option<u32> {
        self.var_table.get(symbol).map(|v| *v)
    }
}

pub fn lirgen(program: &Program) -> Vec<LirTopElem> {
    let mut ctx = GenContext::default();
    gen::lirgen_program(&mut ctx, program)
}
