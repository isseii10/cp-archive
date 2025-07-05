package arraystring

func maxProfit(prices []int) int {
	buy := 100001 // 入力の最大+1
	profit := 0
	for _, v := range prices {
		if buy > v {
			buy = v
		}
		if profit < v-buy {
			profit = v - buy
		}
	}
	return profit
}
