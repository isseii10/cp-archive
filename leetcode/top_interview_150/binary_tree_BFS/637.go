package binarytreebfs

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func averageOfLevels(root *TreeNode) []float64 {
	q := make([]*TreeNode, 0)
	q = append(q, root)
	idx := 0
	ret := make([]float64, 0)
	var isNothing bool
	for len(q[idx:]) > 0 {
		isNothing = true
		nodes := q[idx:]
		idx += len(q[idx:])
		var sum, count float64 = 0, 0
		for _, v := range nodes {
			if v == nil {
				continue
			}
			if isNothing {
				isNothing = false
			}
			sum += float64(v.Val)
			count++
			q = append(q, v.Left, v.Right)
		}
		if !isNothing {
			ret = append(ret, sum/count)
		}
	}
	return ret
}

func averageOfLevels2(root *TreeNode) []float64 {
	if root == nil {
		return []float64{}
	}
	q := []*TreeNode{root}
	aves := make([]float64, 0)
	for len(q) > 0 {
		nextQ := make([]*TreeNode, 0)
		var sum float64 = 0
		for _, v := range q {
			sum += float64(v.Val)
			if v.Left != nil {
				nextQ = append(nextQ, v.Left)
			}
			if v.Right != nil {
				nextQ = append(nextQ, v.Right)
			}
		}
		aves = append(aves, sum/float64(len(q)))
		q = nextQ
	}
	return aves
}
