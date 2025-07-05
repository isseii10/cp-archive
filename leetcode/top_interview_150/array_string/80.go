package arraystring

func removeDuplicates2(nums []int) int {
	slow, fast := 0, 0
	count := 0
	for fast < len(nums) {
		if nums[slow] < nums[fast] {
			slow++
			nums[slow] = nums[fast]
			count = 1
		} else {
			count++
			if count == 2 {
				slow++
				nums[slow] = nums[fast]
			}
		}
		fast++
	}
	nums[slow] = nums[fast-1]
	return slow + 1
}
