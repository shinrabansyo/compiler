use copager::ir::Tree;

use sb_compiler_parse_syntax::{SBLangDef, SBTokens};

use crate::utils::{unwrap_node, unwrap_leaf};
use super::Expr;

#[derive(Debug)]
pub enum InlineAsm {

}

impl From<(String, Tree<'_, SBLangDef>)> for InlineAsm {
    fn from((namespace, tree): (String, Tree<'_, SBLangDef>)) -> Self {
        todo!()
    }
}
