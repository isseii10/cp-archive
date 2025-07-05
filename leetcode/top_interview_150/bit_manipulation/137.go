package bitmanipulation

func singleNumber2(nums []int) int {
	counter := make(map[int]int, len(nums))
	for _, v := range nums {
		counter[v]++
	}
	for k, v := range counter {
		if v == 1 {
			return k
		}
	}
	return 0
}

func singleNumber2_2(nums []int) int {
	ones := 0
	twos := 0

	for _, v := range nums {
		ones = (ones ^ v) & ^twos
		twos = (twos ^ v) & ^ones
	}
	return ones
}
