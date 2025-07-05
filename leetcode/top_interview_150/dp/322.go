package dp

const inf = 10001

func coinChange(coins []int, amount int) int {
	// dp[i]はi円を表せる最小の硬貨数
	dp := make([]int, amount+1)
	for i := range dp {
		dp[i] = inf
	}
	dp[0] = 0
	for i := 1; i <= amount; i++ {
		for _, c := range coins {
			if i-c >= 0 {
				dp[i] = min(dp[i], dp[i-c]+1)
			}
		}
	}
	if dp[amount] == inf {
		return -1
	}
	return dp[amount]
}
