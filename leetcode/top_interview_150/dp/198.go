package dp

func rob(nums []int) int {
	if len(nums) == 1 {
		return nums[0]
	}
	// dp[i]はi番目の家までで、最大の金額
	dp := make([]int, len(nums))
	dp[0] = nums[0]
	dp[1] = max(dp[0], nums[1])
	for i := 2; i < len(nums); i++ {
		// i番目を強盗する場合は、i-2番目から足す
		dp[i] = max(dp[i], dp[i-2]+nums[i])
		// i番目を強盗しない場合はi-1番目がそのまま
		dp[i] = max(dp[i], dp[i-1])
	}
	return dp[len(nums)-1]
}
