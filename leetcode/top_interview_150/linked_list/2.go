package linkedlist

/**
 * Definition for singly-linked list.
 * type ListNode struct {
 *     Val int
 *     Next *ListNode
 * }
 */
func addTwoNumbers(l1 *ListNode, l2 *ListNode) *ListNode {
	root := &ListNode{}
	now := root
	carry := 0
	for now != nil {
		val1, val2 := 0, 0
		if l1 != nil {
			val1 = l1.Val
			l1 = l1.Next
		}
		if l2 != nil {
			val2 = l2.Val
			l2 = l2.Next
		}
		val := val1 + val2 + carry
		if val >= 10 {
			carry = 1
		} else {
			carry = 0
		}

		// set val
		now.Val = val % 10
		// set next
		if l1 == nil && l2 == nil && carry == 0 {
			now.Next = nil
		} else {
			now.Next = &ListNode{}
		}
		// next loop
		now = now.Next
	}
	return root
}
