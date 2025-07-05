package linkedlist

type Node struct {
	Val    int
	Next   *Node
	Random *Node
}

func copyRandomList(head *Node) *Node {
	if head == nil {
		return nil
	}
	// prepare
	nodeList := make([]*Node, 0)
	nodeList = append(nodeList, head)
	idxMap := make(map[*Node]int)
	idxMap[head] = 0
	next := head.Next
	idx := 1
	for next != nil {
		nodeList = append(nodeList, next)
		idxMap[next] = idx
		next = next.Next
		idx++
	}

	// build
	copyNodeList := make([]*Node, len(nodeList)+1)
	// 先にpointer作成
	for i := 0; i < len(nodeList); i++ {
		copyNodeList[i] = &Node{}
	}
	// 埋めていく
	for i, n := range nodeList {
		copyNodeList[i].Val = n.Val
		copyNodeList[i].Next = copyNodeList[i+1]
		if n.Random != nil {
			copyNodeList[i].Random = copyNodeList[idxMap[n.Random]]
		}
	}
	return copyNodeList[0]
}

func copyRandomList_2(head *Node) *Node {
	if head == nil {
		return nil
	}

	// 先にコピー先の箱を用意しておく。
	copyByOrigin := make(map[*Node]*Node)
	node := head
	for node != nil {
		copyByOrigin[node] = &Node{}
		node = node.Next
	}

	// build
	node = head
	for node != nil {
		c := copyByOrigin[node]
		c.Val = node.Val
		c.Next = copyByOrigin[node.Next]
		c.Random = copyByOrigin[node.Random]
		node = node.Next
	}

	return copyByOrigin[head]
}
