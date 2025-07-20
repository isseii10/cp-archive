package golib

type Queue[T any] []T

func NewQueue[T any]() *Queue[T] {
	return &Queue[T]{}
}

func (q *Queue[T]) Push(v T) {
	*q = append(*q, v)
}

func (q *Queue[T]) Pop() T {
	old := *q
	x := old[0]
	*q = old[1:]
	return x
}

func (q *Queue[T]) IsEmpty() bool {
	return len(*q) == 0
}
