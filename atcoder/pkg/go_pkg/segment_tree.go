package gopkg

import "math/bits"

type S int

const e S = 0

func op(x, y S) S {
	return x + y
}

type SegmentTree struct {
	data   []S
	op     func(x, y S) S
	e      S
	size   int
	height int
}

func NewSegmentTree(n int, op func(x, y S) S, e S) SegmentTree {
	height := bits.Len64(uint64(n - 1))
	size := 1 << height
	data := make([]S, size*2)
	for i := 0; i < size*2; i++ {
		data[i] = e
	}
	return SegmentTree{data: data, op: op, e: e, size: size, height: height}
}
func (s *SegmentTree) Initialize(arr []S) {
	for i, a := range arr {
		idx := i + s.size
		s.data[idx] = a
	}
	for i := s.size - 1; i > 0; i-- {
		s.update(i)
	}
}

func (s *SegmentTree) update(i int) {
	s.data[i] = s.op(s.data[i<<1], s.data[i<<1|1])
}

func (s *SegmentTree) Update(pos int, x S) {
	pos += s.size
	s.data[pos] = x
	for i := 1; i <= s.height; i++ {
		s.update(pos >> i)
	}
}

func (s *SegmentTree) Get(pos int) S {
	return s.data[pos+s.size]
}

func (s *SegmentTree) Prod(l, r int) S {
	// op(data[l], data[l+1], ..., data[r-1])を返す
	left := s.e
	right := s.e
	l += s.size
	r += s.size

	for l < r {
		if l&1 == 1 {
			left = s.op(left, s.data[l])
			l += 1
		}
		if r&1 == 1 {
			r -= 1
			right = s.op(s.data[r], right)
		}
		l >>= 1
		r >>= 1
	}
	return s.op(left, right)
}

func (s *SegmentTree) AllProd() S {
	return s.data[1]
}
