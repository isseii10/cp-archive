package dp

import "strings"

func wordBreak(s string, wordDict []string) bool {
	ss := strings.Split(s, "")
	n := len(ss)
	wordMap := make(map[string]bool)
	for _, s := range wordDict {
		wordMap[s] = true
	}
	// dp[i]はi番目までで条件を満たすか
	dp := make([]bool, n+1)
	dp[0] = true
	for i := 1; i <= n; i++ {
		for j := 0; j < i; j++ {
			if dp[j] && wordMap[s[j:i]] {
				dp[i] = true
			}
		}
	}
	return dp[n]
}
