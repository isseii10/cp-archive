package main

import (
	"bufio"
	"fmt"
	"math"
	"os"
	"sort"
	"strconv"
	"strings"
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
	T := scanInt()
	for t := 0; t < T; t++ {
		n := scanInt()
		A := scanIntSlice(n)
		if solve(n, A) {
			out("Yes")
		} else {
			out("No")
		}
	}
}

func solve(n int, A []int) bool {
	a0 := A[0]
	// 公比が1の場合
	rIsOne := true
	for _, a := range A {
		if a != a0 {
			rIsOne = false
			break
		}
	}
	if rIsOne {
		return true
	}
	// 公比が-1の場合
	rIsMinusOne := true
	pos := 0
	neg := 0
	for _, a := range A {
		if a == a0 {
			pos++
		}
		if a == -a0 {
			neg++
		}
	}
	if pos+neg != n || abs(pos-neg) > 1 {
		rIsMinusOne = false
	}
	if rIsMinusOne {
		return true
	}

	// 公比の絶対値が1より大きい場合
	sort.Slice(A, func(i, j int) bool { return abs(A[i]) < abs(A[j]) })
	ok := true
	for i := 1; i < n-1; i++ {
		if A[i]*A[i] != A[i-1]*A[i+1] {
			ok = false
			break
		}
	}
	return ok
}

func init() {
	sc.Buffer([]byte{}, math.MaxInt64)
	sc.Split(bufio.ScanWords)

	// inputあれば使う
	if f, err := os.Open("./input"); err == nil {
		sc = bufio.NewScanner(f)
		sc.Split(bufio.ScanWords)
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

// =====================================================================================
// heap
// =====================================================================================
type Heap []int

func NewHeap() *Heap {
	return &Heap{}
}
func (h Heap) IsEmpty() bool {
	return len(h) == 0
}

// heapインターフェースの実装 heap化はheap.Init(hq)
func (h Heap) Len() int           { return len(h) }
func (h Heap) Swap(i, j int)      { h[i], h[j] = h[j], h[i] }
func (h Heap) Less(i, j int) bool { return h[i] < h[j] }

func (h *Heap) Push(e interface{}) {
	*h = append(*h, e.(int))
}
func (h *Heap) Pop() interface{} {
	old := *h
	n := len(old)
	x := old[n-1]
	*h = old[:n-1]
	return x
}
