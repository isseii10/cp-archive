package binarytreegeneral

/**
 * Definition for a binary tree node.
 * type TreeNode struct {
 *     Val int
 *     Left *TreeNode
 *     Right *TreeNode
 * }
 */

// preorder and inorder consist of unique values.
// -> 値でノードの同一判定をする
func buildTree(preorder []int, inorder []int) *TreeNode {
	if len(preorder) == 0 || len(inorder) == 0 {
		return nil
	}
	rootVal := preorder[0]
	root := &TreeNode{
		Val: rootVal,
	}
	rootIdx := 0
	for i, v := range inorder {
		if v == rootVal {
			rootIdx = i
		}
	}

	// inorderはrootValから見て左側は左の部分木の構成要素、右側は右の部分木の構成要素になっている
	// preorderはroot、左部分木、右部分木の順に訪れるので、
	root.Left = buildTree(preorder[1:], inorder[:rootIdx])
	root.Right = buildTree(preorder[1+rootIdx:], inorder[rootIdx+1:])
	return root
}
