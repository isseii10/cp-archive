package hashmap

import "strings"

func wordPattern(pattern string, s string) bool {
	toS := make(map[string]string, 26)
	toP := make(map[string]string, 26)
	ss := strings.Split(s, " ")
	pp := strings.Split(pattern, "")
	if len(ss) != len(pp) {
		return false
	}

	for i, v := range pp {
		if _, ok := toS[v]; !ok {
			toS[v] = ss[i]
		}
		if _, ok := toP[ss[i]]; !ok {
			toP[ss[i]] = v
		}
		if toP[ss[i]] != v || toS[v] != ss[i] {
			return false
		}
	}
	return true
}
