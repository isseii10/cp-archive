package arraystring

// valと等しくない要素の数を返す
func removeElement(nums []int, val int) int {
	k := 0
	n := len(nums)
	for i, num := range nums {
		for nums[n-1-k] == val {
			k++
			if k == n {
				return 0
			}
		}
		if i >= n-1-k {
			return n - k
		}

		if num == val {
			nums[i], nums[n-1-k] = nums[n-1-k], nums[i]
		}
	}
	return n - k
}
