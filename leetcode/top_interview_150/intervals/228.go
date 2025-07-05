package intervals

import (
	"strconv"
)

func summaryRanges(nums []int) []string {
	summary := make([]string, 0, len(nums))

	left := 0
	for left < len(nums) {
		right := left
		start := nums[left]
		end := nums[right]

		for right+1 < len(nums) {
			if nums[right]+1 != nums[right+1] {
				break
			}
			right++
			end = nums[right]
		}

		summary = append(summary, makeSummary(start, end))

		left = right + 1
	}
	return summary
}

func makeSummary(start, end int) string {
	r := strconv.Itoa(start)
	if end > start {
		r += "->" + strconv.Itoa(end)
	}
	return r
}
