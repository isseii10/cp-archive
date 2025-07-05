package binarytreegeneral

func hasPathSum(root *TreeNode, targetSum int) bool {
	if root == nil {
		return false
	}
	if root.Left == nil && root.Right == nil {
		// leaf
		return root.Val == targetSum
	}
	nextSum := targetSum - root.Val
	return hasPathSum(root.Left, nextSum) || hasPathSum(root.Right, nextSum)
}
