package golib

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

func (h *Heap) Push(e interface{}) {
	*h = append(*h, e.(int))
}
func (h *Heap) Pop() interface{} {
	old := *h
	n := len(old)
	x := old[n-1]
	*h = old[:n-1]
	return x
}
