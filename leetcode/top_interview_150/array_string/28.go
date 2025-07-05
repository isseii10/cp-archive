package arraystring

import "strings"

func strStr(haystack string, needle string) int {
	return strings.Index(haystack, needle)
}

func strStr2(haystack string, needle string) int {
	n := len(needle)
	res := -1
	for i := 0; i+n <= len(haystack); i++ {
		if haystack[i:i+n] == needle {
			res = i
			break
		}
	}
	return res
}
