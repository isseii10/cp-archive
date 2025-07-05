package gopkg

const mod = 100000007

type combFactorial struct {
	fac    []int
	facInv []int
}

func newCombFactorial(n int) *combFactorial {

	fac := make([]int, n)
	facInv := make([]int, n)
	fac[0] = 1
	facInv[0] = modInvFermat(1, mod)

	for i := 1; i < n; i++ {
		fac[i] = modMulti(i, fac[i-1])
		facInv[i] = modInvFermat(fac[i], mod)
	}

	return &combFactorial{
		fac:    fac,
		facInv: facInv,
	}
}

func (c *combFactorial) Factorial(n int) int {
	return c.fac[n]
}

func (c *combFactorial) Combination(n, r int) int {
	if r > n {
		return 0
	}
	return modMulti(modMulti(c.fac[n], c.facInv[r]), c.facInv[n-r])
}

func (c *combFactorial) Permutation(n, r int) int {
	if r > n {
		return 0
	}
	return modMulti(c.fac[n], c.facInv[n-r])
}

func (c *combFactorial) HomogeousProduct(n, r int) int {
	return c.Combination(n-1+r, r)
}

// ==================================================
// mod
// ==================================================

// modを法としてa+b
func modAdd(a, b int) int {
	ret := a + b
	if ret < 0 {
		ret += mod
	}
	return ret % mod
}

// modを法としてa*b
func modMulti(a, b int) int {
	return a * b % mod
}

// modを法としてa/b
func modDiv(a, b int) int {
	a %= mod
	return a * modInvFermat(b, mod) % mod
}

// mを法としてa^nを求める
func modPow(a, n, m int) int {
	if m == 1 {
		return 0
	}
	r := 1
	for n > 0 {
		if n&1 == 1 {
			r = r * a % m
		}
		a, n = a*a%m, n>>1
	}
	return r
}

// mを法としてaの逆元を求める
func modInv(a, m int) int {
	p, x, u := m, 1, 0
	for p != 0 {
		t := a / p
		a, p = p, a-t*p
		x, u = u, x-t*u
	}
	x %= m
	if x < 0 {
		x += m
	}
	return x
}

// フェルマーの小定理を用いてaの逆元を求める
// mは素数
func modInvFermat(a, m int) int {
	return modPow(a, m-2, mod)
}
