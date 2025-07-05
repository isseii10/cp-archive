package gopkg

import "container/heap"

// 追加
// 最小値(最大値)の取り出し
// 削除
type PriorityQueue struct {
	q    *Heap
	dq   *Heap
	desc bool
}

func NewPriorityQueue(desc bool) PriorityQueue {
	return PriorityQueue{
		q:    NewHeap(),
		dq:   NewHeap(),
		desc: desc,
	}
}
func (pq *PriorityQueue) checkInput(v int) int {
	in := v
	if pq.desc {
		in = -v
	}
	return in
}
func (pq *PriorityQueue) Push(v int) {
	in := pq.checkInput(v)
	heap.Push(pq.q, in)
}
func (pq *PriorityQueue) Pop() (int, bool) {
	for !pq.q.IsEmpty() && !pq.dq.IsEmpty() {
		target := heap.Pop(pq.q).(int)
		deleted := heap.Pop(pq.dq).(int)
		if target == deleted {
			continue
		}
		heap.Push(pq.dq, deleted)
		return pq.checkInput(target), true
	}
	if pq.dq.IsEmpty() {
		target := heap.Pop(pq.q).(int)
		return pq.checkInput(target), true
	}
	return 0, false
}
func (pq *PriorityQueue) Erase(v int) {
	in := pq.checkInput(v)
	heap.Push(pq.dq, in)
}
func (pq *PriorityQueue) Size() int {
	return len(*pq.q) - len(*pq.dq)
}
func (pq *PriorityQueue) IsEmpty() bool {
	return pq.Size() == 0
}

// =====================================================================================
// heap
// =====================================================================================
type Heap []int

func NewHeap() *Heap {
	return &Heap{}
}
func (h Heap) IsEmpty() bool {
	return len(h) == 0
}

// heapインターフェースの実装 heap化はheap.Init(hq)
func (h Heap) Len() int           { return len(h) }
func (h Heap) Swap(i, j int)      { h[i], h[j] = h[j], h[i] }
func (h Heap) Less(i, j int) bool { return h[i] < h[j] }

func (h *Heap) Push(e any) {
	*h = append(*h, e.(int))
}
func (h *Heap) Pop() any {
	old := *h
	n := len(old)
	x := old[n-1]
	*h = old[:n-1]
	return x
}
