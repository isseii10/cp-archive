package hashmap

func containsNearbyDuplicate(nums []int, k int) bool {
	counter := make(map[int]int)
	// init
	for i := 0; i < min(k+1, len(nums)); i++ {
		counter[nums[i]]++
		if counter[nums[i]] == 2 {
			return true
		}
	}
	for i := 0; i+k+1 < len(nums); i++ {
		counter[nums[i]]--
		counter[nums[i+k+1]]++
		if counter[nums[i+k+1]] == 2 {
			return true
		}
	}
	return false
}
