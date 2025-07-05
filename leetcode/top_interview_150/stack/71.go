package stack

import "strings"

func simplifyPath(path string) string {
	p := strings.Split(path, "/")
	stack := make([]string, len(p))
	top := 0
	for _, s := range p {
		if s == "" || s == "." {
			continue
		}
		if s == ".." {
			if top > 0 {
				top--
			}
			continue
		}
		stack[top] = s
		top++
	}
	return "/" + strings.Join(stack[:top], "/")
}
