package twopointers

func maxArea(height []int) int {
	left := 0
	right := len(height) - 1
	ans := 0
	for left < right {
		ans = max(ans, min(height[left], height[right])*(right-left))
		if height[left] < height[right] {
			left++
		} else {
			right--
		}
	}
	return ans
}
