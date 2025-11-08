struct TarjanSCC {
    timestamp: usize,
    low: Vec<usize>,
    dfn: Vec<usize>,
    graph: Vec<Vec<usize>>,
    graph_id: Vec<Vec<usize>>,
    bridges: Vec<usize>,
}

impl TarjanSCC {
    fn new(graph: Vec<Vec<usize>>, graph_id: Vec<Vec<usize>>) -> Self {
        let n = graph.len();
        let mut t = Self {
            timestamp: 1,
            low: vec![0; n],
            dfn: vec![0; n],
            bridges: vec![],
            graph,
            graph_id,
        };
        t.find_bridges();
        t
    }

    fn find_bridges(&mut self) {
        for i in 0..self.graph.len() {
            self.dfs(i, usize::MAX);
        }
    }

    fn dfs(&mut self, cur: usize, cur_edge: usize) {
        self.dfn[cur] = self.timestamp;
        self.low[cur] = self.timestamp;
        self.timestamp += 1;

        for i in 0..self.graph[cur].len() {
            let nxt_edge = self.graph_id[cur][i];
            if nxt_edge == cur_edge {
                continue;
            }
            let nxt = self.graph[cur][i];
            if self.dfn[nxt] == 0 {
                self.dfs(nxt, nxt_edge);
                if self.dfn[cur] < self.low[nxt] {
                    self.bridges.push(nxt_edge);
                }
            }
            self.low[cur] = self.low[cur].min(self.low[nxt]);
        }
    }
}
