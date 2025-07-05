package bitmanipulation

// 1 <= n <= 2^31 - 1
func hammingWeight(n int) int {
	popCount := 0
	for i := 0; i < 32; i++ {
		if n>>i&1 == 1 {
			popCount++
		}
	}
	return popCount
}
