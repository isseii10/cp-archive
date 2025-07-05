package linkedlist

// 消すやつの一個前と一個後がわかれば良い
func removeNthFromEnd(head *ListNode, n int) *ListNode {
	node := head
	length := 0
	for node != nil {
		length++
		node = node.Next
	}

	targetIdx := length - n

	// 消すやつの一個前と一個後を特定する
	var before *ListNode
	var after *ListNode
	node = head
	for i := 0; i < length; i++ {
		if i == targetIdx-1 {
			before = node
		}
		if i == targetIdx+1 {
			after = node
		}
		node = node.Next
	}
	if before == nil {
		// 消すのはhead
		return head.Next
	}
	before.Next = after
	return head
}
