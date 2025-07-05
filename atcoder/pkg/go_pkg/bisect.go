package gopkg

func bisectLeft(x int, arr []int) int {
	l := 0
	r := len(arr)
	for l < r {
		mid := (l + r) / 2
		if arr[mid] < x {
			l = mid + 1
		} else {
			r = mid
		}
	}
	return l
}

func bisectRight(x int, arr []int) int {
	l := 0
	r := len(arr)
	for l < r {
		mid := (r + l) / 2
		if arr[mid] <= x {
			l = mid + 1
		} else {
			r = mid
		}
	}
	return l
}
