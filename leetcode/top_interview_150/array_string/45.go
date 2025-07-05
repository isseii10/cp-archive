package arraystring

const inf = 10001

func jump(nums []int) int {
	n := len(nums)
	dp := make([]int, n)
	// dp[i]はiに到達するために必要な最小ジャンプ回数
	for i := range dp {
		dp[i] = inf
	}
	dp[0] = 0
	for i := 0; i < n; i++ {
		m := nums[i]
		for j := 1; j <= m; j++ {
			if i+j < n {
				dp[i+j] = min(dp[i+j], dp[i]+1)
			}
		}
	}
	return dp[n-1]
}

func jump_2(nums []int) int {
	reachable := 0
	now := 0
	jumps := 0

	for i, v := range nums {
		if now >= len(nums)-1 {
			break
		}
		reachable = max(reachable, i+v)
		if i == now {
			jumps++
			now = reachable
		}
	}
	return jumps
}
