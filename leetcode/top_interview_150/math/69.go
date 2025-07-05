package math

func mySqrt(x int) int {
	for i := 0; i*i <= x; i++ {
		if i*i <= x && x < (i+1)*(i+1) {
			return i
		}
	}
	return 0
}

func mySqrt2(x int) int {
	if x == 1 {
		return 1
	}
	ok, ng := 0, x
	for ng-ok > 1 {
		mid := (ok + ng) / 2
		if mid*mid <= x {
			ok = mid
		} else {
			ng = mid
		}
	}
	return ok
}
