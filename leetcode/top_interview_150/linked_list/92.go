package linkedlist

func reverseBetween(head *ListNode, left int, right int) *ListNode {
	nodes := make([]*ListNode, 0)
	node := head
	for node != nil {
		nodes = append(nodes, node)
		node = node.Next
	}
	reverse(nodes[left-1 : right])

	// 付け替え
	for i := 0; i < len(nodes)-1; i++ {
		nodes[i].Next = nodes[i+1]
	}
	nodes[len(nodes)-1].Next = nil
	return nodes[0]
}

func reverse[T any](elems []T) {
	n := len(elems)
	for i := 0; i < n/2; i++ {
		elems[i], elems[n-1-i] = elems[n-1-i], elems[i]
	}
}
