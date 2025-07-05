package hashmap

import "strings"

func isIsomorphic(s string, t string) bool {
	ss := strings.Split(s, "")
	tt := strings.Split(t, "")
	n := len(ss)
	to := make(map[string]string)
	for i := 0; i < n; i++ {
		a := ss[i]
		b := tt[i]
		if _, ok := to[a]; !ok {
			to[a] = b
		}
		if _, ok := to[b]; !ok {
			to[b] = a
		}
		if a != to[b] || b != to[a] {
			return false
		}
	}
	return true
}
