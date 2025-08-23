pub struct SparseTable {
    tab: Vec<Vec<i32>>,
}

impl SparseTable {
    pub fn new(data: &[i32]) -> Self {
        let n = data.len();
        let d = Self::log2(n);
        let mut t = vec![vec![0; d + 1]; n];

        for i in 0..n {
            t[i][0] = data[i];
        }

        for e in 1..d + 1 {
            for i in 0..n {
                let j = i + Self::pow2(e);
                if j <= n {
                    // println!("{} {}", i, e);
                    t[i][e] = t[i][e - 1].min(t[(i + j) / 2][e - 1])
                }
            }
        }
        // println!("{:?}", t);
        Self { tab: t }
    }

    pub fn query(&self, i: usize, j: usize) -> i32 {
        let n = j - i;
        let d = Self::log2(n);
        let m = j - Self::pow2(d);
        return self.tab[i][d].min(self.tab[m][d]);
    }

    fn pow2(n: usize) -> usize {
        2_usize.pow(n as u32)
    }

    fn log2(n: usize) -> usize {
        (n as f32).log(2.0) as usize
    }
}
