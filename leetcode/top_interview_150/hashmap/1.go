package hashmap

func twoSum(nums []int, target int) []int {
	seen := make(map[int]int)
	for i, v := range nums {
		w := target - v
		if _, ok := seen[w]; !ok {
			seen[v] = i
			continue
		}
		return []int{seen[w], i}
	}
	return []int{}
}
