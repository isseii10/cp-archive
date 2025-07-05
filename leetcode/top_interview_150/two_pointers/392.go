package twopointers

import (
	"strings"
)

// s and t consist only of lowercase English letters.
func isSubsequence(s string, t string) bool {
	ss := strings.Split(s, "")
	tt := strings.Split(t, "")
	idxS := 0
	idxT := 0
	lenS := len(ss)
	lenT := len(tt)
	for idxS != lenS && idxT != lenT {
		if ss[idxS] == tt[idxT] {
			idxS++
		}
		idxT++
	}
	return idxS == lenS
}
