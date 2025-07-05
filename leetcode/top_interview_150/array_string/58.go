package arraystring

func lengthOfLastWord(s string) int {
	count := 0
	counting := false
	for i := len(s) - 1; i >= 0; i-- {
		if counting && string(s[i]) == " " {
			break
		}
		if string(s[i]) != " " {
			if !counting {
				counting = true
			}
			count++
		}
	}
	return count
}
