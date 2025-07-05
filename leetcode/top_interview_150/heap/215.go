package heap

import "container/heap"

func findKthLargest(nums []int, k int) int {
	minusNums := make([]int, len(nums))
	for i, v := range nums {
		minusNums[i] = -v
	}
	h := intHeap(minusNums)
	heap.Init(&h)
	for i := 0; i < k-1; i++ {
		heap.Pop(&h)
	}
	return -h[0]
}

type intHeap []int

func (h intHeap) Len() int           { return len(h) }
func (h intHeap) Less(i, j int) bool { return h[i] < h[j] }
func (h intHeap) Swap(i, j int)      { h[i], h[j] = h[j], h[i] }
func (h *intHeap) Push(x interface{}) {
	*h = append(*h, x.(int))
}

func (h *intHeap) Pop() interface{} {
	old := *h
	n := len(old)
	x := old[n-1]
	*h = old[0 : n-1]
	return x
}
