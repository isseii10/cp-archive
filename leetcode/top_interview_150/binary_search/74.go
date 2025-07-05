package binarysearch

func searchMatrix(matrix [][]int, target int) bool {
	for _, row := range matrix {
		if target < row[0] || row[len(row)-1] < target {
			continue
		}
		return search(row, target)
	}
	return false
}

func search(nums []int, target int) bool {
	le := 0         // lt is less than or equal to
	gt := len(nums) // gt is greater than
	for gt-le > 1 {
		mid := (le + gt) / 2
		if nums[mid] <= target {
			le = mid
		} else {
			gt = mid
		}
	}
	return nums[le] == target
}
