pub struct DSU {
    par: Vec<usize>,
    sz: Vec<usize>,
}

impl DSU {
    pub fn new(n: usize) -> Self {
        Self {
            par: (0..n).collect(),
            sz: vec![1; n],
        }
    }

    pub fn find(&mut self, x: usize) -> usize {
        if self.par[x] != x {
            self.par[x] = self.find(self.par[x]);
        }
        self.par[x]
    }

    pub fn unite(&mut self, x: usize, y: usize) -> bool {
        let (mut px, mut py) = (self.find(x), self.find(y));
        if px != py {
            if self.sz[px] > self.sz[py] {
                std::mem::swap(&mut px, &mut py);
            }
            self.par[px] = self.par[py];
            self.sz[py] += self.sz[px];
            true
        } else {
            false
        }
    }

    pub fn is_connected(&mut self, x: usize, y: usize) -> bool {
        self.find(x) == self.find(y)
    }

    pub fn component_num(&self) -> usize {
        self.par
            .iter()
            .enumerate()
            .filter(|&(x, p)| x == *p)
            .count()
    }
}


struct UninitializedDSU {
    par: Vec<usize>,
    sz: Vec<usize>,
}

impl UninitializedDSU {
    fn new(n: usize) -> Self {
        Self {
            par: (0..n).collect(),
            sz: vec![0; n],
        }
    }

    fn len(&self) -> usize {
        let mut ans = 0;
        for (i, p) in self.par.iter().enumerate() {
            if i == *p {
                ans += self.sz[i];
            }
        }
        ans
    }

    fn find(&mut self, x: usize) -> usize {
        if self.par[x] != x {
            self.par[x] = self.find(self.par[x]);
        }
        self.par[x]
    }

    fn insert(&mut self, x: usize) {
        if self.sz[x] == 0 {
            self.sz[x] = 1
        }
    }

    fn unite(&mut self, x: usize, y: usize) -> bool {
        self.insert(x);
        self.insert(y);
        let (mut px, mut py) = (self.find(x), self.find(y));
        if px != py {
            if self.sz[px] > self.sz[py] {
                std::mem::swap(&mut px, &mut py);
            }
            self.par[px] = self.par[py];
            self.sz[py] += self.sz[px];
            true
        } else {
            false
        }
    }

    fn connected(&mut self, x: usize, y: usize) -> bool {
        self.find(x) == self.find(y)
    }

    fn component_num(&self) -> usize {
        self.par
            .iter()
            .enumerate()
            .filter(|&(x, p)| self.sz[x] > 0 && x == *p)
            .count()
    }
}
