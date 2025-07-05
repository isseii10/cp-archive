package bitmanipulation

import "slices"

func reverseBits(num uint32) uint32 {
	rs := make([]rune, 32)
	for i := 0; i < 32; i++ {
		if num>>i&1 == 1 {
			rs[i] = '1'
		} else {
			rs[i] = '0'
		}
	}
	slices.Reverse(rs)
	var res uint32 = 0
	for i, v := range rs {
		if v == '1' {
			res += 1 << i
		}
	}
	return res
}
