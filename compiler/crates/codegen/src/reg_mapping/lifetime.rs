use std::collections::HashSet;

use sb_compiler_lirgen_ir::LirBlock;

pub fn analyze_lifetime(lir_block: &LirBlock) -> impl Iterator<Item = (u32, Vec<u32>)> {
    // 1. 単純に寿命を求める
    let simple_lifetime = SimpleLifetimeAnalyzer::analyze(lir_block);

    // 2. 制御構造を考慮して，より正確に寿命を求める
    LifetimeAnalyzer::analyze(lir_block, simple_lifetime)
}

struct SimpleLifetimeAnalyzer {
    already_ended: HashSet<u32>,
    end_point: Vec<Vec<u32>>,
}

impl SimpleLifetimeAnalyzer {
    fn analyze(lir_block: &LirBlock) -> impl Iterator<Item = Vec<u32>> {
        let mut analyzer = SimpleLifetimeAnalyzer {
            already_ended: HashSet::new(),
            end_point: vec![],
        };
        analyzer.find_end_point(lir_block);
        analyzer.end_point.push(vec![]);
        analyzer.end_point.into_iter().rev()
    }

    fn find_end_point(&mut self, lir_block: &LirBlock) {
        match lir_block {
            LirBlock::Single { lirs, .. } => {
                for lir_block in lirs.iter().rev() {
                    self.find_end_point(lir_block);
                }
            }
            LirBlock::Multiple { lirs } => {
                for lir_block in lirs.iter().rev() {
                    self.find_end_point(lir_block);
                }
            }
            LirBlock::Inst { src1, src2, .. } => {
                // 1. 既に消失していない かつ ゼロレジスタでない レジスタを記録対象とする
                let do_mark_src1 = !self.already_ended.contains(src1) && *src1 > 0;
                let do_mark_src2 = !self.already_ended.contains(src2) && *src2 > 0;

                // 2. src に含まれるレジスタを消失済みレジスタとして記録
                let end_regs = match (do_mark_src1, do_mark_src2) {
                    (true, true) => vec![*src1, *src2],
                    (true, false) => vec![*src1],
                    (false, true) => vec![*src2],
                    (false, false) => vec![],
                };
                self.end_point.push(end_regs);
                self.already_ended.insert(*src1);
                self.already_ended.insert(*src2);
            }
            _ => {}
        }
    }
}

struct LifetimeAnalyzer<T>
where
    T: Iterator<Item = Vec<u32>>,
{
    // 結果
    begin_point: Vec<u32>,
    end_point: Vec<Vec<u32>>,

    // 解析中に動的に使用する変数 (analyze_tree 内で使用)
    layered_alive_regs: Vec<HashSet<u32>>,
    will_be_destroyed_regs: Vec<u32>,
    end_point_iter: T,
}

impl<T> LifetimeAnalyzer<T>
where
    T: Iterator<Item = Vec<u32>>,
{
    fn analyze(lir_block: &LirBlock, end_point_iter: T) -> impl Iterator<Item = (u32, Vec<u32>)> {
        // 1. 変数準備
        let mut analyzer = LifetimeAnalyzer {
            begin_point: vec![0],
            end_point: vec![vec![]],
            layered_alive_regs: vec![HashSet::new()],
            will_be_destroyed_regs: vec![],
            end_point_iter,
        };

        // 2. 解析 (end_point_iter はプログラム開始前の情報を持っているので，最初の要素をスキップ)
        analyzer.end_point_iter.next();
        analyzer.analyze_tree(lir_block);

        // 3. 誕生と消失をペアにしたイテレータを構成
        analyzer.begin_point
            .into_iter()
            .zip(analyzer.end_point.into_iter())
    }

    fn analyze_tree(&mut self, lir_block: &LirBlock) {
        match lir_block {
            LirBlock::Single { lirs, .. } => {
                for lir_block in lirs {
                    self.analyze_tree(lir_block);
                }
            }
            LirBlock::Multiple { lirs } => {
                // 1. Multiple スコープを開始
                self.layered_alive_regs.push(HashSet::new());

                // 2. 子ノードを解析
                for lir_block in lirs {
                    self.analyze_tree(lir_block);
                }

                // 3. Multiple スコープを終了
                self.layered_alive_regs.pop();

                // 4. 終了記録を延期したレジスタを記録
                self.end_point
                    .last_mut()
                    .unwrap()
                    .extend(self.will_be_destroyed_regs.drain(..));
            }
            LirBlock::Inst { dst, .. } => {
                // 1. 誕生したレジスタとして dst を記録
                self.layered_alive_regs.last_mut().unwrap().insert(*dst);
                self.begin_point.push(*dst);

                // 2. より上位の Multiple スコープで使用されているレジスタ以外を，消失したレジスタとして記録
                //    記録できなかった場合は，現在の Multiple スコープ終了時に破棄するレジスタとして記録
                let mut truly_end_regs = vec![];
                'outer: for end_reg in self.end_point_iter.next().unwrap() {
                    // 2.1. より上位の Multiple スコープを対象にイテレータを準備
                    let mut layered_alive_regs_iter = self.layered_alive_regs.iter().rev();
                    layered_alive_regs_iter.next();

                    // 2.2. 今消失できるレジスタか確認
                    for alive_regs in layered_alive_regs_iter {
                        if alive_regs.contains(&end_reg) {
                            self.will_be_destroyed_regs.push(end_reg);
                            continue 'outer;
                        }
                    }
                    truly_end_regs.push(end_reg);
                }
                self.end_point.push(truly_end_regs);
            }
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use sb_compiler_lirgen_ir::{lir, LirBlock, Add, Addi, Ble, Jmp, Li};

    use super::analyze_lifetime;

    #[test]
    fn test_lifetime_1() {
        let lir = LirBlock::Single {
            result_reg: 0,
            lirs: vec![
                lir!(Li(10) 20),       // li t0 = 10
                lir!(Li(20) 21),       // li t1 = 20
                lir!(Add 22, 20, 21),  // add  t22 = t20 + t21
            ],
        };

        let mut lifetime_tracker = analyze_lifetime(&lir);
        assert_eq!(lifetime_tracker.next(), Some((0, vec![])));
        assert_eq!(lifetime_tracker.next(), Some((20, vec![])));
        assert_eq!(lifetime_tracker.next(), Some((21, vec![])));
        assert_eq!(lifetime_tracker.next(), Some((22, vec![20, 21])));
        assert_eq!(lifetime_tracker.next(), None);
    }

    #[test]
    fn test_lifetime_2() {
        let lir = LirBlock::Single {
            result_reg: 0,
            lirs: vec![
                lir!(Li(0) 20),  // li t20 = 0 (cnt)
                lir!(Li(0) 21),  // li t21 = 0 (sum)
                lir!(Li(10) 22), // li t22 = 10
                LirBlock::Multiple {
                    lirs: vec![
                        lir!(Ble(12) 0, 22, 20), // ble r0, (t22 <= t20) -> 24
                        lir!(Add 21, 0, 20),     // add t21 = t0 + t20
                        lir!(Addi(1) 20, 0),     // addi t20 = t0 + 1
                        lir!(Jmp(-18)),          // jmp -18
                    ],
                },
            ],
        };

        let mut lifetime_tracker = analyze_lifetime(&lir);
        assert_eq!(lifetime_tracker.next(), Some((0, vec![])));
        assert_eq!(lifetime_tracker.next(), Some((20, vec![])));
        assert_eq!(lifetime_tracker.next(), Some((21, vec![])));
        assert_eq!(lifetime_tracker.next(), Some((22, vec![])));
        assert_eq!(lifetime_tracker.next(), Some((0, vec![])));
        assert_eq!(lifetime_tracker.next(), Some((21, vec![])));
        assert_eq!(lifetime_tracker.next(), Some((20, vec![])));
        assert_eq!(lifetime_tracker.next(), Some((0, vec![22, 20])));
        assert_eq!(lifetime_tracker.next(), None);
    }
}
