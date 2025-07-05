package arraystring

func canJump(nums []int) bool {
	reachable := 0
	for i, v := range nums {
		if i > reachable {
			return false
		}
		if reachable < v+i {
			reachable = v + i
		}
	}
	return true
}
