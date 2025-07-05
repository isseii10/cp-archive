package slidingwindow

func lengthOfLongestSubstring(s string) int {
	runes := []rune(s)
	counter := make(map[rune]int)
	r := 0
	n := len(runes)
	ans := 0
	for l := 0; l < n; l++ {
		for r < n && counter[runes[r]] == 0 {
			counter[runes[r]]++
			r++
		}

		ans = max(ans, r-l)

		counter[runes[l]]--
		if l == r {
			r++
		}
	}
	return ans
}
