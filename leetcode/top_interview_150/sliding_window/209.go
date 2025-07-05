package slidingwindow

func minSubArrayLen(target int, nums []int) int {
	r := 0
	minLen := 100001
	sum := 0
	for l := 0; l < len(nums); l++ {
		for r < len(nums) && sum < target {
			sum += nums[r]
			r++
		}
		if sum >= target {
			minLen = min(minLen, r-l)
		}

		sum -= nums[l]
		if l == r {
			r++
		}
	}
	if minLen == 100001 {
		return 0
	}
	return minLen
}
