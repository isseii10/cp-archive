package binarytreegeneral

// 103,226を使った回答
// 元のtreeを破壊しているのであんまりよくない
func isSymmetric(root *TreeNode) bool {
	leftInv, right := invertTree(root.Left), root.Right
	return isSameTree(leftInv, right)
}

func isSymmetric2(root *TreeNode) bool {
	if root == nil {
		return true
	}

	return checkSymmetric(root.Left, root.Right)
}

func checkSymmetric(left, right *TreeNode) bool {
	if left == nil && right == nil {
		return true
	}
	if left != nil && right != nil {
		if left.Val == right.Val {
			return checkSymmetric(left.Left, right.Right) && checkSymmetric(left.Right, right.Left)
		}
	}
	return false
}
