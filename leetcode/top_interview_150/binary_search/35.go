package binarysearch

func searchInsert(nums []int, target int) int {
	ok, ng := 0, len(nums)
	for ng-ok > 1 {
		mid := (ng + ok) / 2

		if nums[mid] < target {
			ok = mid
		} else {
			ng = mid
		}
	}
	if nums[ok] >= target {
		return ok
	}
	return ok + 1
}
