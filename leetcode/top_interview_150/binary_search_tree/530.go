package binarysearchtree

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func getMinimumDifference(root *TreeNode) int {
	if root == nil {
		return 0
	}
	ret := 1000000
	if root.Left != nil {
		ret = min(ret, root.Val-getMax(root.Left), getMinimumDifference(root.Left))
	}
	if root.Right != nil {
		ret = min(ret, getMin(root.Right)-root.Val, getMinimumDifference(root.Right))
	}

	return ret
}

// root is not nil
func getMin(root *TreeNode) int {
	for root.Left != nil {
		root = root.Left
	}
	return root.Val
}

// root is not nil
func getMax(root *TreeNode) int {
	for root.Right != nil {
		root = root.Right
	}
	return root.Val
}
