package math

func isPalindrome(x int) bool {
	if x < 0 {
		return false
	}

	digits := make([]int, 0)
	for x > 0 {
		digits = append(digits, x%10)
		x /= 10
	}
	n := len(digits)
	mid := n / 2
	for i := 0; i < mid; i++ {
		if digits[i] != digits[n-1-i] {
			return false
		}
	}
	return true
}

// 反対にするには、足して10倍する
func isPalindrome2(x int) bool {
	if x < 0 {
		return false
	}
	num := x
	rev := 0
	for num > 0 {
		rev *= 10
		rev += num % 10
		num /= 10
	}
	return x == rev
}
