use cargo_snippet::snippet;

type Mint = ac_library::ModInt998244353;

/// 階乗とその逆元を前計算して nCr, nPr, H を求める構造体
#[snippet]
pub struct CombFactorial {
    fac: Vec<Mint>,
    fac_inv: Vec<Mint>,
}

impl CombFactorial {
    /// n までの階乗と逆元を前計算
    pub fn new(n: usize) -> Self {
        let mut fac = vec![Mint::new(1); n];
        let mut fac_inv = vec![Mint::new(1); n];

        for i in 1..n {
            fac[i] = fac[i - 1] * Mint::new(i as u32);
            fac_inv[i] = fac[i].inv();
        }

        Self { fac, fac_inv }
    }

    /// n!
    pub fn factorial(&self, n: usize) -> Mint {
        self.fac[n]
    }

    /// nCr
    pub fn combination(&self, n: usize, r: usize) -> Mint {
        if r > n {
            return Mint::new(0);
        }
        self.fac[n] * self.fac_inv[r] * self.fac_inv[n - r]
    }

    /// nPr
    pub fn permutation(&self, n: usize, r: usize) -> Mint {
        if r > n {
            return Mint::new(0);
        }
        self.fac[n] * self.fac_inv[n - r]
    }

    /// 重複組合せ nHr
    pub fn homogeneous_product(&self, n: usize, r: usize) -> Mint {
        self.combination(n - 1 + r, r)
    }
}

/// パスカルの三角形で nCr を求める構造体
/// mが合成数の場合(ConbFactorialが使えない場合)に使用する
#[snippet]
pub struct CombPascal {
    pascal: Vec<Vec<Mint>>,
}

impl CombPascal {
    /// n までの nCr を前計算
    pub fn new(n: usize) -> Self {
        let mut pascal = vec![vec![Mint::new(0); n + 1]; n + 1];
        pascal[0][0] = Mint::new(1);
        for i in 0..n {
            for j in 0..=i {
                pascal[i + 1][j] = pascal[i + 1][j] + pascal[i][j];
                pascal[i + 1][j + 1] = pascal[i + 1][j + 1] + pascal[i][j];
            }
        }
        Self { pascal }
    }
    /// nCr
    pub fn combination(&self, n: usize, r: usize) -> Mint {
        if r > n {
            return Mint::new(0);
        }
        self.pascal[n][r]
    }
}
