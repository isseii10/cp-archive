package bitmanipulation

import "slices"

// 短い方に番兵入れると綺麗やけどちょっと遅い
func addBinary(a string, b string) string {
	if len(a) < len(b) {
		a, b = b, a
	}
	aa := []rune(a)
	bb := []rune(b)
	slices.Reverse(aa)
	slices.Reverse(bb)
	n := len(aa)
	m := len(bb)
	for i := 0; i < n-m; i++ {
		bb = append(bb, '0')
	}
	ret := []rune{}
	carry := false
	for i := 0; i < n; i++ {
		if aa[i]-'0' == 1 && bb[i]-'0' == 1 {
			if carry {
				ret = append(ret, '1')
			} else {
				ret = append(ret, '0')
			}
			carry = true
		} else if aa[i]-'0' == 0 && bb[i]-'0' == 0 {
			if carry {
				ret = append(ret, '1')
			} else {
				ret = append(ret, '0')
			}
			carry = false
		} else {
			if carry {
				ret = append(ret, '0')
			} else {
				ret = append(ret, '1')
			}
		}
	}

	if carry {
		ret = append(ret, '1')
	}
	slices.Reverse(ret)
	return string(ret)
}

func addBinary2(a string, b string) string {
	if len(a) < len(b) {
		a, b = b, a
	}
	aa := []rune(a)
	bb := []rune(b)
	slices.Reverse(aa)
	slices.Reverse(bb)

	ret := []rune{}
	carry := false
	for i := 0; i < len(aa); i++ {
		if aa[i] == '1' && i < len(bb) && bb[i] == '1' {
			if carry {
				ret = append(ret, '1')
			} else {
				ret = append(ret, '0')
			}
			carry = true
		} else if aa[i] == '0' && i < len(bb) && bb[i] == '1' {
			if carry {
				ret = append(ret, '0')
			} else {
				ret = append(ret, '1')
			}
		} else {
			if carry && aa[i] == '1' {
				ret = append(ret, '0')
			} else if !carry && aa[i] == '0' {
				ret = append(ret, '0')
			} else {
				ret = append(ret, '1')
				carry = false
			}
		}
	}

	if carry {
		ret = append(ret, '1')
	}
	slices.Reverse(ret)
	return string(ret)
}
