package stack

func isValid(s string) bool {
	st := make([]rune, len(s))
	top := -1
	for _, v := range s {
		// push to stack
		if v == '(' || v == '{' || v == '[' {
			top++
			st[top] = v
			continue
		}
		// pop from stack
		if top < 0 {
			return false
		}
		switch v {
		case ')':
			if st[top] != '(' {
				return false
			}
		case '}':
			if st[top] != '{' {
				return false
			}
		case ']':
			if st[top] != '[' {
				return false
			}
		}
		top--
	}
	// is empty?
	return top == -1
}
