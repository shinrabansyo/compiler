use petgraph::graph::{DiGraph, NodeIndex};

#[derive(Debug, Hash, PartialEq, Eq)]
enum Node<T> {
    DiBegin,
    UnBegin,
    Internal(T),
}

pub struct LayeredGraph<T> {
    graph: DiGraph<Node<T>, ()>,
}

// ユーザによる初期化用
impl<T> LayeredGraph<T> {
    pub fn new_with_directed() -> (LayeredGraph<T>, NodeIndex) {
        LayeredGraph::new(Node::DiBegin)
    }

    pub fn new_with_undirected() -> (LayeredGraph<T>, NodeIndex) {
        LayeredGraph::new(Node::UnBegin)
    }

    fn new(begin: Node<T>) -> (LayeredGraph<T>, NodeIndex) {
        let mut graph = DiGraph::new();
        let cur_node = graph.add_node(begin);
        let graph = LayeredGraph { graph };

        (graph, cur_node)
    }
}

// ユーザからのクエリへの回答
impl<T> LayeredGraph<T> {
    pub fn add_directed_layer(&mut self, cur_node: NodeIndex) -> NodeIndex {
        let dibegin_node = self.graph.add_node(Node::DiBegin);
        self.graph.add_edge(dibegin_node, cur_node, ());

        dibegin_node
    }

    pub fn add_undirected_layer(&mut self, cur_node: NodeIndex) -> NodeIndex {
        let unbegin_node = self.graph.add_node(Node::UnBegin);
        self.graph.add_edge(unbegin_node, cur_node, ());

        unbegin_node
    }

    pub fn add_node(&mut self, cur_node: NodeIndex, new_node: T) -> NodeIndex {
        let new_node = self.graph.add_node(Node::Internal(new_node));
        self.graph.add_edge(new_node, cur_node, ());
        if self.is_undirected_layer(cur_node) {
            self.graph.add_edge(cur_node, new_node, ());
        }

        new_node
    }

    pub fn find(&self, cur_node: NodeIndex, is_goal: impl Fn(&T) -> bool) -> Option<&T> {
        let is_goal = |nidx: NodeIndex| -> bool {
            match &self.graph[nidx] {
                Node::Internal(t) => is_goal(t),
                _ => false,
            }
        };
        self.find_nearest(cur_node, is_goal)
            .map(|nidx| self.graph.node_weight(nidx).unwrap())
            .map(|node| match node {
                Node::Internal(t) => t,
                _ => unreachable!(),
            })
    }
}

// 内部クエリへの回答
impl<T> LayeredGraph<T> {
    fn find_nearest(
        &self,
        start: NodeIndex,
        is_goal: impl Fn(NodeIndex) -> bool,
    ) -> Option<NodeIndex> {
        use petgraph::algo::astar as petgraph_aster;

        let result = petgraph_aster(
            &self.graph,
            start,
            is_goal,
            |_| 0,
            |_| 0,
        );
        match result {
            Some((_, path)) => path.last().map(|&nidx| nidx),
            None => None,
        }
    }

    fn is_undirected_layer(&self, cur_node: NodeIndex) -> bool {
        let is_begin = |nidx: NodeIndex| -> bool {
            match &self.graph[nidx] {
                Node::DiBegin => true,
                Node::UnBegin => true,
                _ => false,
            }
        };
        match self.find_nearest(cur_node, is_begin) {
            Some(nidx) => match &self.graph[nidx] {
                Node::DiBegin => false,
                Node::UnBegin => true,
                _ => unreachable!(),
            },
            None => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use petgraph::graph::NodeIndex;

    use super::LayeredGraph;

    impl LayeredGraph<i32> {
        fn check_visibility(
            &self,
            cur_node: NodeIndex,
            visible: &[i32],
            invisible: &[i32],
        ) {
            for v in visible {
                assert_eq!(
                    self.find(cur_node, |t| t == v).is_some(),
                    true,
                    "Node '{:?}' should be visible from node {:?}",
                    v,
                    self.graph.node_weight(cur_node).unwrap(),
                );
            }
            for iv in invisible {
                assert_eq!(
                    self.find(cur_node, |t| t == iv).is_some(),
                    false,
                    "Node '{:?}' should be invisible from node {:?}",
                    iv,
                    self.graph.node_weight(cur_node).unwrap(),
                );
            }
        }
    }

    mod directed {
        use super::LayeredGraph;

        #[test]
        fn single_layer() {
            // Laayer0 : 1 <-- 2 <-- 3
            let (mut graph, cur_0) = LayeredGraph::<i32>::new_with_directed();
            let cur_1 = graph.add_node(cur_0, 1);
            let cur_2 = graph.add_node(cur_1, 2);
            let cur_3 = graph.add_node(cur_2, 3);

            // テスト本体
            graph.check_visibility(
                cur_1,
                &[1],
                &[2, 3],
            );
            graph.check_visibility(
                cur_2,
                &[1, 2],
                &[3],
            );
            graph.check_visibility(
                cur_3,
                &[1, 2, 3],
                &[],
            );
        }

        #[test]
        fn multiple_layers() {
            // Layer0 :  1  <-- 2 <-- 3
            let (mut graph, cur_0) = LayeredGraph::<i32>::new_with_directed();
            let cur_1 = graph.add_node(cur_0, 1);
            let cur_2 = graph.add_node(cur_1, 2);
            let cur_3 = graph.add_node(cur_2, 3);

            // Layer1 : (2) <-- 4
            let cur_0 = graph.add_directed_layer(cur_2);
            let cur_4 = graph.add_node(cur_0, 4);

            // Layer2 : (3) <-- 5
            let cur_0 = graph.add_directed_layer(cur_3);
            let cur_5 = graph.add_node(cur_0, 5);

            // テスト本体
            graph.check_visibility(
                cur_1,
                &[1],
                &[2, 3, 4, 5],
            );
            graph.check_visibility(
                cur_2,
                &[1, 2],
                &[3, 4, 5],
            );
            graph.check_visibility(
                cur_3,
                &[1, 2, 3],
                &[4, 5],
            );
            graph.check_visibility(
                cur_4,
                &[1, 2, 4],
                &[3, 5],
            );
            graph.check_visibility(
                cur_5,
                &[1, 2, 3, 5],
                &[4],
            );
        }
    }

    mod undirected {
        use super::LayeredGraph;

        #[test]
        fn single_layer() {
            // Layer0 : 1 <-> 2 <-> 3
            let (mut graph, cur_0) = LayeredGraph::<i32>::new_with_undirected();
            let cur_1 = graph.add_node(cur_0, 1);
            let cur_2 = graph.add_node(cur_1, 2);
            let cur_3 = graph.add_node(cur_2, 3);

            // テスト本体
            graph.check_visibility(
                cur_1,
                &[1, 2, 3],
                &[],
            );
            graph.check_visibility(
                cur_2,
                &[1, 2, 3],
                &[],
            );
            graph.check_visibility(
                cur_3,
                &[1, 2, 3],
                &[],
            );
        }

        #[test]
        fn multiple_layers() {
            // Layer0 :  1  <-> 2 <-> 3
            let (mut graph, cur_0) = LayeredGraph::<i32>::new_with_undirected();
            let cur_1 = graph.add_node(cur_0, 1);
            let cur_2 = graph.add_node(cur_1, 2);
            let cur_3 = graph.add_node(cur_2, 3);

            // Layer1 : (2) <-> 4
            let cur_0 = graph.add_undirected_layer(cur_2);
            let cur_4 = graph.add_node(cur_0, 4);

            // Layer2 : (3) <-> 5
            let cur_0 = graph.add_undirected_layer(cur_3);
            let cur_5 = graph.add_node(cur_0, 5);

            // テスト本体
            graph.check_visibility(
                cur_1,
                &[1, 2, 3],
                &[4, 5],
            );
            graph.check_visibility(
                cur_2,
                &[1, 2, 3],
                &[4, 5],
            );
            graph.check_visibility(
                cur_3,
                &[1, 2, 3],
                &[4, 5],
            );
            graph.check_visibility(
                cur_4,
                &[1, 2, 3, 4],
                &[5],
            );
            graph.check_visibility(
                cur_5,
                &[1, 2, 3, 5],
                &[4],
            );
        }
    }

    mod mixed {
        use super::LayeredGraph;

        #[test]
        fn multiple_layers() {
            // Layer0(undirected) :  1  <-> 2 <-> 3
            let (mut graph, cur_0) = LayeredGraph::<i32>::new_with_undirected();
            let cur_1 = graph.add_node(cur_0, 1);
            let cur_2 = graph.add_node(cur_1, 2);
            let cur_3 = graph.add_node(cur_2, 3);

            // Layer1(directed)   : (2) <-- 4 <-- 5
            let cur_0 = graph.add_directed_layer(cur_2);
            let cur_4 = graph.add_node(cur_0, 4);
            let cur_5 = graph.add_node(cur_4, 5);

            // Layer2(undirected) : (3) <-> 6 <-> 7
            let cur_0 = graph.add_undirected_layer(cur_3);
            let cur_6 = graph.add_node(cur_0, 6);
            let cur_7 = graph.add_node(cur_6, 7);

            // テスト本体
            graph.check_visibility(
                cur_1,
                &[1, 2, 3],
                &[4, 5, 6, 7],
            );
            graph.check_visibility(
                cur_2,
                &[1, 2, 3],
                &[4, 5, 6, 7],
            );
            graph.check_visibility(
                cur_3,
                &[1, 2, 3],
                &[4, 5, 6, 7],
            );
            graph.check_visibility(
                cur_4,
                &[1, 2, 3, 4],
                &[5, 6, 7],
            );
            graph.check_visibility(
                cur_5,
                &[1, 2, 3, 4, 5],
                &[6, 7],
            );
            graph.check_visibility(
                cur_6,
                &[1, 2, 3, 6, 7],
                &[4, 5],
            );
            graph.check_visibility(
                cur_7,
                &[1, 2, 3, 6, 7],
                &[4, 5],
            );
        }
    }
}

