package linkedlist

/**
 * Definition for singly-linked list.
 * type ListNode struct {
 *     Val int
 *     Next *ListNode
 * }
 */

func mergeTwoLists(list1 *ListNode, list2 *ListNode) *ListNode {
	sorted := make([]*ListNode, 101)
	idx := 0
	// enqueue
	for list1 != nil || list2 != nil {
		if list1 != nil && list2 != nil {
			if list1.Val <= list2.Val {
				sorted[idx] = &ListNode{Val: list1.Val, Next: list1.Next}
				list1 = list1.Next
			} else {
				sorted[idx] = &ListNode{Val: list2.Val, Next: list2.Next}
				list2 = list2.Next
			}
			idx++
		} else if list1 != nil && list2 == nil {
			sorted[idx] = &ListNode{Val: list1.Val, Next: list1.Next}
			list1 = list1.Next
			idx++
		} else if list1 == nil && list2 != nil {
			sorted[idx] = &ListNode{Val: list2.Val, Next: list2.Next}
			list2 = list2.Next
			idx++
		}
	}

	// build
	idx2 := 0
	for sorted[idx2] != nil {
		sorted[idx2].Next = sorted[idx2+1]
		idx2++
	}
	return sorted[0]
}

func mergeTwoLists2(list1 *ListNode, list2 *ListNode) *ListNode {
	result := &ListNode{}
	current := result

	for list1 != nil && list2 != nil {
		if list1.Val < list2.Val {
			// ポインタ型への代入は、実体のListNodeへのaddressがコピーされる
			current.Next = list1 // Nextが参照する実体のListNodeをlist1が参照している実体のListNode(Aとする)にする
			list1 = list1.Next   // list1の参照をAの次にする
		} else {
			current.Next = list2
			list2 = list2.Next
		}
		current = current.Next
	}

	if list1 == nil {
		current.Next = list2
	} else {
		current.Next = list1
	}

	return result.Next
}
