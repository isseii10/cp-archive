package twopointers

// s consists only of printable ASCII characters.
func isPalindrome(s string) bool {
	runes := []rune(s)
	l := 0
	r := len(runes) - 1
	ok := true
	for r-l > 0 {
		rOk := isAlphanumeric(runes[r])
		lOk := isAlphanumeric(runes[l])

		if rOk && lOk {
			if toLowerIfUpper(runes[r]) != toLowerIfUpper(runes[l]) {
				ok = false
				break
			}
			r--
			l++
			continue
		}

		if !rOk {
			r--
		}
		if !lOk {
			l++
		}
	}
	return ok
}

func isAlphanumeric(r rune) bool {
	if 'a' <= r && r <= 'z' || 'A' <= r && r <= 'Z' || '0' <= r && r <= '9' {
		return true
	}
	return false
}

func toLowerIfUpper(r rune) rune {
	if 'A' <= r && r <= 'Z' {
		return rune(r - 'A' + 'a')
	}
	return r
}
