package arraystring

import "strings"

func longestCommonPrefix(strs []string) string {
	minLen := 200
	for _, s := range strs {
		if minLen > len(s) {
			minLen = len(s)
		}
	}
	ss := make([][]string, len(strs))
	for i := range ss {
		ss[i] = strings.Split(strs[i], "")
	}
	ret := make([]string, minLen)

M:
	for i := 0; i < minLen; i++ {
		now := ss[0][i]
		for _, s := range ss {
			if s[i] != now {
				break M
			}
		}
		ret = append(ret, now)
	}

	return strings.Join(ret, "")
}
