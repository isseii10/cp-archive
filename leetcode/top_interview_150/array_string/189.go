package arraystring

func rotate(nums []int, k int) {
	n := len(nums)
	memo := make([]int, n)
	for i := range memo {
		memo[i] = nums[i]
	}
	for from := range nums {
		to := (from + k + n) % n
		nums[to] = memo[from]
	}
}

// reverse 3回
func rotate_2(nums []int, k int) {
	n := len(nums)
	k = (k + n) % n
	// reverse left
	reverse(nums[:n-k])
	// reverse right
	reverse(nums[n-k:])
	// all
	reverse(nums)
}

func reverse(nums []int) {
	n := len(nums)
	for i := 0; i < n/2; i++ {
		nums[i], nums[n-1-i] = nums[n-1-i], nums[i]
	}
}
