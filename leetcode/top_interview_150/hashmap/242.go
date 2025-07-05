package hashmap

func isAnagram(s string, t string) bool {
	if len(s) != len(t) {
		return false
	}
	counter := make([]int, 26)
	for i := 0; i < len(s); i++ {
		counter[s[i]-'a']++
		counter[t[i]-'a']--
	}
	for _, v := range counter {
		if v != 0 {
			return false
		}
	}
	return true
}
