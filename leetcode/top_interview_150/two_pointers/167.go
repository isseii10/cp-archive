package twopointers

func twoSum(numbers []int, target int) []int {
	for i := 0; i < len(numbers); i++ {
		complement := target - numbers[i]
		// if complement < numbers[i] {
		// 	// 解は必ずあるのでここには到達しない
		// 	break
		// }

		j := find(numbers[i+1:], complement)
		if j == -1 {
			continue
		}
		return []int{i + 1, i + j + 2}
	}
	return []int{}
}

func find(numbers []int, target int) int {
	left, right := -1, len(numbers)-1
	for right-left > 1 {
		mid := (left + right) / 2
		if numbers[mid] < target {
			left = mid
		} else {
			right = mid
		}
	}
	if numbers[right] == target {
		return right
	}
	return -1
}
