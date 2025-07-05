package gopkg

type Deque[T any] []T

func NewDeque[T any]() *Deque[T] {
	return &Deque[T]{}
}
func (d *Deque[T]) PushBack(v T) {
	*d = append(*d, v)
}
func (d *Deque[T]) PushFront(v T) {
	*d = append([]T{v}, *d...)
}
func (d *Deque[T]) PopFront() T {
	old := *d
	x := old[0]
	*d = old[1:]
	return x
}
func (d *Deque[T]) PopBack() T {
	old := *d
	n := len(old)
	x := old[n-1]
	*d = old[:n-1]
	return x
}
func (d *Deque[T]) IsEmpty() bool {
	return len(*d) == 0
}
