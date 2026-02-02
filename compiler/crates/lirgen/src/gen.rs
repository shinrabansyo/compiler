// トップ要素
mod program;           pub use program::lirgen_program;
mod top;               pub use top::lirgen_top;

// 定義
mod func_def;          pub use func_def::lirgen_func_def;

// 文
mod stmt;              pub use stmt::lirgen_stmt;
mod block;             pub use block::lirgen_block;
mod var_decl;          pub use var_decl::lirgen_var_decl;
mod r#return;          pub use r#return::lirgen_return;
mod r#if;              pub use r#if::lirgen_if;
mod r#while;           pub use r#while::lirgen_while;
mod r#for;             pub use r#for::lirgen_for;
mod inasm;             pub use inasm::lirgen_inline_asm;

// 式
mod expr;              pub use expr::lirgen_expr;
mod assign;            pub use assign::lirgen_assign;
mod value_l;           pub use value_l::lirgen_value_l;
mod logic_or;          pub use logic_or::lirgen_logic_or;
mod logic_and;         pub use logic_and::lirgen_logic_and;
mod bit_or;            pub use bit_or::lirgen_bit_or;
mod bit_xor;           pub use bit_xor::lirgen_bit_xor;
mod bit_and;           pub use bit_and::lirgen_bit_and;
mod cond;              pub use cond::lirgen_cond;
mod bit_shift;         pub use bit_shift::lirgen_bit_shift;
mod add;               pub use add::lirgen_add;
mod mul;               pub use mul::lirgen_mul;
mod cast;              pub use cast::lirgen_cast;
mod unary;             pub use unary::lirgen_unary;
mod value_r;           pub use value_r::lirgen_value_r;
mod struct_init;       pub use struct_init::lirgen_struct_init;
mod struct_field_init; pub use struct_field_init::lirgen_struct_field_init;
mod struct_access;     pub use struct_access::lirgen_struct_access;

// HIR -> AST 用
use sb_compiler_semcheck_impl_var::Var;
use sb_compiler_utils::collections::LayeredTable;

const ZERO_REG: u32 = 0;
const ADDR_REG: u32 = 7;
const RET_REG: u32 = 10;
const FARG_REG_BASE: u32 = 10;

#[derive(Debug, Clone)]
pub(crate) struct GenContext<'src> {
    reserved_regs: u32,
    reserved_labels: u32,
    var_table: LayeredTable<Var<'src>, u32>,
}

impl<'src> Default for GenContext<'src> {
    fn default() -> Self {
        GenContext {
            reserved_regs: 20,   // r0: ゼロレジスタ, r10 ~ r19: 引数レジスタ として確保済み
            reserved_labels: 0,
            var_table: LayeredTable::default(),
        }
    }
}

impl<'src> GenContext<'src> {
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

    fn set_var_reg(&mut self, var: Var<'src>, reg: u32) {
        self.var_table.insert(var, reg);
    }

    fn ref_var_reg(&self, var: &Var<'src>) -> Option<u32> {
        self.var_table.get(var).map(|v| *v)
    }
}
