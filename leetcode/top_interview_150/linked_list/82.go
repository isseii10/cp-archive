package linkedlist

func deleteDuplicates(head *ListNode) *ListNode {
	if head == nil {
		return nil
	}
	before := &ListNode{Next: head}
	slow := before
	fast := head.Next
	duplicated := false
	for fast != nil {
		if slow.Next.Val == fast.Val {
			// 重複を検知したのでマークして、fastにはさらなる重複があるか探索してもらう
			duplicated = true
			fast = fast.Next
			continue
		}

		if duplicated {
			slow.Next = fast
			fast = fast.Next
			duplicated = false
		} else {
			slow = slow.Next
			fast = fast.Next
		}
	}
	if duplicated {
		slow.Next = nil
	}
	return before.Next
}
