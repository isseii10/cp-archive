package arraystring

func maxProfit2(prices []int) int {
	minV := prices[0]
	maxV := prices[0]
	profit := 0
	for _, v := range prices {
		if maxV <= v {
			maxV = v
			continue
		}
		profit += maxV - minV
		minV = v
		maxV = v
	}
	profit += maxV - minV
	return profit
}
