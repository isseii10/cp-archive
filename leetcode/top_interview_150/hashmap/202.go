package hashmap

func isHappy(n int) bool {
	memo := make(map[int]int)
	for n != 1 {
		if _, ok := memo[n]; ok {
			// すでに計算済みだったら、無限ループ突入
			return false
		}
		// calculate
		now := n
		res := 0
		for now != 0 {
			d := now % 10
			res += d * d
			now /= 10
		}
		memo[n] = res
		n = res
	}
	return true
}
