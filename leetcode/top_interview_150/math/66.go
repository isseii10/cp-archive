package math

func plusOne(digits []int) []int {
	carry := 1
	for i := len(digits) - 1; i >= 0; i-- {
		if digits[i]+carry == 10 {
			digits[i] = 0
			carry = 1
			continue
		}
		digits[i] += carry
		carry = 0
	}
	if carry == 1 {
		return append([]int{1}, digits...)
	}
	return digits
}
