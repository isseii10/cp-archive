package main

import (
	"bufio"
	"fmt"
	"math"
	"os"
	"strconv"
	"strings"

	"github.com/liyue201/gostl/algorithm/sort"
	"github.com/liyue201/gostl/ds/slice"
	"github.com/liyue201/gostl/utils/comparator"
)

const (
	inf           = math.MaxInt64
	mod1000000007 = 1000000007
	mod998244353  = 998244353
	mod           = mod1000000007
)

var (
	sc  = bufio.NewScanner(os.Stdin)
	wtr = bufio.NewWriter(os.Stdout)
)

func main() {
	defer flush()
	n, m := scanInt2()

	edges := make([][]bool, n)
	for i := 0; i < n; i++ {
		edges[i] = make([]bool, n)
		for j := 0; j < n; j++ {
			edges[i][j] = false
		}
	}

	for i := 0; i < m; i++ {
		a, b := scanInt2()
		a--
		b--
		edges[a][b] = true
		edges[b][a] = true
	}

	ns := make([]int, n+1)
	for i := 0; i < n; i++ {
		ns[i] = i
	}
	ns[n] = -1 // しきり

	ans := inf
	nodes := slice.NewSliceWrapper(ns)
	ok := true
	for ok {
		// edgeをコピーする
		copiedEdges := make([][]bool, n)
		for i := 0; i < n; i++ {
			copiedEdges[i] = make([]bool, n)
			for j := 0; j < n; j++ {
				copiedEdges[i][j] = edges[i][j]
			}
		}
		ans = min(ans, solve(n, nodes, copiedEdges))
		ok = sort.NextPermutation[int](nodes.Begin(), nodes.End(), comparator.IntComparator)
	}
	out(ans)
}

func solve(n int, nodes *slice.SliceWrapper[int], edges [][]bool) int {
	// 仕切りの場所探して分ける
	A := make([]int, 0, n)
	B := make([]int, 0, n)
	found := false
	for i := 0; i < nodes.Len(); i++ {
		if nodes.At(i) == -1 {
			found = true
			continue
		}
		if found {
			A = append(A, nodes.At(i))
		} else {
			B = append(B, nodes.At(i))
		}
	}

	if len(A) != 0 && len(A) < 3 {
		return inf
	}
	if len(B) != 0 && len(B) < 3 {
		return inf
	}

	ret := 0
	for _, nodeGroup := range [][]int{A, B} {
		if len(nodeGroup) == 0 {
			continue
		}
		for i := 0; i < len(nodeGroup); i++ {
			prev := nodeGroup[(i-1+len(nodeGroup))%len(nodeGroup)]
			now := nodeGroup[i]
			next := nodeGroup[(i+1)%len(nodeGroup)]
			for j := 0; j < n; j++ { // ここは全てのノードで回す
				if j == now {
					continue
				}

				if edges[now][j] {
					if j == prev || j == next {
						continue
					}
					// 余計なところに繋がっているので削除
					ret++
					edges[now][j] = false
					edges[j][now] = false
				} else {
					if j != prev && j != next {
						continue
					}
					// prev/nextに繋がっていないので追加
					ret++
					edges[now][j] = true
					edges[j][now] = true
				}
			}
		}
	}
	return ret
}

func init() {
	sc.Buffer([]byte{}, math.MaxInt64)
	sc.Split(bufio.ScanWords)

	if len(os.Args) > 1 && os.Args[1] == "i" {
		b, e := os.ReadFile("./input")
		if e != nil {
			panic(e)
		}
		sc = bufio.NewScanner(strings.NewReader(strings.Replace(string(b), " ", "\n", -1)))
	}
}

// ==================================================
// io
// ==================================================

func scanInt() int {
	sc.Scan()
	i, e := strconv.Atoi(sc.Text())
	if e != nil {
		panic(e)
	}
	return i
}

func scanInt2() (int, int) {
	return scanInt(), scanInt()
}

func scanInt3() (int, int, int) {
	return scanInt(), scanInt(), scanInt()
}

func scanInt4() (int, int, int, int) {
	return scanInt(), scanInt(), scanInt(), scanInt()
}

func scanIntSlice(n int) []int {
	a := make([]int, n)
	for i := 0; i < n; i++ {
		a[i] = scanInt()
	}
	return a
}

func scanIntSlice2(n int) ([]int, []int) {
	a := make([]int, n)
	b := make([]int, n)
	for i := 0; i < n; i++ {
		a[i], b[i] = scanInt2()
	}
	return a, b
}

func scanIntSlice3(n int) ([]int, []int, []int) {
	a := make([]int, n)
	b := make([]int, n)
	c := make([]int, n)
	for i := 0; i < n; i++ {
		a[i], b[i], c[i] = scanInt3()
	}
	return a, b, c
}

func scanIntSlice4(n int) ([]int, []int, []int, []int) {
	a := make([]int, n)
	b := make([]int, n)
	c := make([]int, n)
	d := make([]int, n)
	for i := 0; i < n; i++ {
		a[i], b[i], c[i], d[i] = scanInt4()
	}
	return a, b, c, d
}

func scanFloat() float64 {
	sc.Scan()
	f, e := strconv.ParseFloat(sc.Text(), 64)
	if e != nil {
		panic(e)
	}
	return f
}

func scanString() string {
	sc.Scan()
	return sc.Text()
}

func scani() int {
	var i int
	fmt.Scanf("%i", &i)
	return i
}

func scans() string {
	var s string
	fmt.Scanf("%s", &s)
	return s
}

func out(v ...any) {
	_, e := fmt.Fprintln(wtr, v...)
	if e != nil {
		panic(e)
	}
}

func outwoln(v ...any) {
	_, e := fmt.Fprint(wtr, v...)
	if e != nil {
		panic(e)
	}
}

func outIntSlice(sl []int) {
	r := make([]string, len(sl))
	for i, v := range sl {
		r[i] = itoa(v)
	}
	out(strings.Join(r, " "))
}

func flush() {
	e := wtr.Flush()
	if e != nil {
		panic(e)
	}
}

func atoi(s string) int {
	i, e := strconv.Atoi(s)
	if e != nil {
		panic(e)
	}
	return i
}

func itoa(i int) string {
	return strconv.Itoa(i)
}

func btoi(b byte) int {
	return atoi(string(b))
}

// ==================================================
// num
// ==================================================

func min(arr ...int) int {
	if len(arr) == 0 {
		panic("min error: len(arr) = 0")
	}
	res := inf
	for _, v := range arr {
		if res > v {
			res = v
		}
	}
	return res
}

func max(arr ...int) int {
	if len(arr) == 0 {
		panic("max error: len(arr) = 0")
	}
	res := -inf
	for _, v := range arr {
		if res < v {
			res = v
		}
	}
	return res
}

func abs(a int) int {
	if a > 0 {
		return a
	}
	return -a
}

func gcd(a, b int) int {
	if b == 0 {
		return a
	}
	return gcd(b, a%b)
}

// =====================================================================================
// mod
// =====================================================================================

// modを法としてa+b
func modAdd(a, b int) int {
	ret := a + b
	if ret < 0 {
		ret += mod
	}
	return ret % mod
}

func modSub(a, b int) int {
	ret := a - b
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
