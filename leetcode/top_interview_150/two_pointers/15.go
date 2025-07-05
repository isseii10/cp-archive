package twopointers

import "sort"

func threeSum(nums []int) [][]int {
	sort.Ints(nums)
	ans := make([][]int, 0)
	seen := make(map[[3]int]bool)
	for i := 0; i < len(nums)-2; i++ {
		j := i + 1
		k := len(nums) - 1
		for j < k {
			sum := nums[i] + nums[j] + nums[k]
			if sum == 0 {
				if !seen[[3]int{nums[i], nums[j], nums[k]}] {
					ans = append(ans, []int{nums[i], nums[j], nums[k]})
					seen[[3]int{nums[i], nums[j], nums[k]}] = true
				}
				j++
				k--
			} else if sum > 0 {
				k--
			} else {
				j++
			}
		}
	}
	return ans
}
