use std::collections::HashMap;

use sb_compiler_parse_ast::{AST, ConstDecl};

#[derive(Debug)]
pub struct Define<'ast> {
    pub local_id: u32,
    pub ty: &'ast String,
}

#[derive(Debug)]
pub struct DefineTable<'ast> {
    defines: HashMap<String, HashMap<String, Define<'ast>>>,
}

// AST からの変換処理
impl<'ast> DefineTable<'ast> {
    pub fn from(ast: &'ast AST) -> anyhow::Result<Self> {
        match ast {
            AST::ConstDecl { const_decl, .. } => {
                let mut def_table = DefineTable {
                    defines: HashMap::new(),
                };
                def_table.from_const_decl(const_decl)?;
                Ok(def_table)
            }
        }
    }

    fn from_const_decl(&mut self, const_decl: &'ast ConstDecl) -> anyhow::Result<()> {
        if self.find_once(&const_decl.namespace, &const_decl.ident).is_some() {
            return Err(anyhow::anyhow!("{} is already defined", const_decl.ident));
        }

        let local_id = if let Some(defines) = self.defines.get(&const_decl.namespace) {
            defines.len() as u32
        } else {
            0
        };
        self.defines
            .entry(const_decl.namespace.clone())
            .or_insert_with(HashMap::new)
            .entry(const_decl.ident.clone())
            .insert_entry(Define { local_id, ty: &const_decl.ty });

        Ok(())
    }
}

// 検索クエリ解決処理
impl<'ast> DefineTable<'ast> {
    pub fn find(&self, namespace: &str, name: &str) -> Option<&Define<'ast>> {
        if let Some(defines) = self.defines.get(namespace) {
            if let Some(define) = defines.get(name) {
                return Some(define);
            }
        }

        let dot_pos = namespace.find('.');
        match dot_pos {
            Some(pos) => self.find(&namespace[pos+1..], name),
            None => None,
        }
    }

    pub fn find_once(&self, namespace: &str, name: &str) -> Option<&Define<'ast>> {
        self.defines
            .get(namespace)?
            .get(name)
    }
}
