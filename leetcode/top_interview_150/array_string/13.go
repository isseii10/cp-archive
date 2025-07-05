package arraystring

import "strings"

func romanToInt(s string) int {
	romanMap := map[string]int{
		"I":  1,
		"V":  5,
		"X":  10,
		"L":  50,
		"C":  100,
		"D":  500,
		"M":  1000,
		"IV": 4,
		"IX": 9,
		"XL": 40,
		"XC": 90,
		"CD": 400,
		"CM": 900,
	}
	ret := 0
	sSlice := strings.Split(s, "")
	for i, v := range sSlice {
		if i != len(sSlice)-1 {
			n := sSlice[i+1]
			if _, ok := romanMap[v+n]; ok {
				ret -= romanMap[v]
				continue
			}
		}
		ret += romanMap[v]
	}
	return ret
}
