package golib

import (
	"sort"
)

// 約数列挙
func divisor(n int) []int {
	ret := make([]int, 0)
	for i := 1; i*i <= n; i++ {
		if n%i == 0 {
			ret = append(ret, i)
			if i != n/i {
				ret = append(ret, n/i)
			}
		}
	}
	sort.Ints(ret)
	return ret
}

// 素因数分解
func primeFactor(n int) []int {
	ret := make([]int, 0)
	for i := 2; i*i <= n; i++ {
		for n%i == 0 {
			ret = append(ret, i)
			n /= i
		}
	}
	if n != 1 {
		ret = append(ret, n)
	}
	return ret
}

// 素因数分解 return map
func primeFactorMap(n int) map[int]int {
	ret := make(map[int]int)
	for i := 2; i*i <= n; i++ {
		for n%i == 0 {
			_, ok := ret[i]
			if !ok {
				ret[i] = 0
			}
			ret[i]++
			n /= i
		}
	}
	if n != 1 {
		_, ok := ret[n]
		if !ok {
			ret[n] = 0
		}
		ret[n]++
	}
	return ret
}

// 素数判定
func isPrime(n int) bool {
	if n == 1 {
		return false
	}
	for i := 2; i*i <= n; i++ {
		if n%i == 0 {
			return false
		}
	}
	return true
}
