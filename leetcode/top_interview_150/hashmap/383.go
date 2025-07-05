package hashmap

import "strings"

// Constraints:
// 1 <= ransomNote.length, magazine.length <= 105
// ransomNote and magazine consist of lowercase English letters.

func canConstruct(ransomNote string, magazine string) bool {
	m := make(map[string]int)
	ss := strings.Split(magazine, "")
	for _, s := range ss {
		m[s]++
	}
	for _, s := range strings.Split(ransomNote, "") {
		m[s]--
		if m[s] < 0 {
			return false
		}
	}
	return true
}

// 小文字アルファベットはたかだか26文字なことを利用する
func canConstruct2(ransomNote string, magazine string) bool {
	count := make([]int, 26)
	for _, r := range magazine {
		count[r-'a']++
	}
	for _, r := range ransomNote {
		count[r-'a']--
		if count[r-'a'] < 0 {
			return false
		}
	}
	return true
}
