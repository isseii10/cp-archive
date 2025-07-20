package golib

import "math/bits"

type SS struct {
	Zero, One, Inv int
}

func ope(x, y SS) SS {
	return SS{
		Zero: x.Zero + y.Zero,
		One:  x.One + y.One,
		Inv:  x.Inv + y.Inv + x.One*y.Zero,
	}
}

var ee = SS{One: 0, Zero: 0, Inv: 0}

type F bool

func composition(f, g F) F {
	return (f && !g) || (!f && g)
}

var id F = false

func mapping(f F, x SS) SS {
	if !f {
		return x
	}
	return SS{
		Zero: x.One,
		One:  x.Zero,
		Inv:  x.One*x.Zero - x.Inv,
	}
}

type LazySegmentTree struct {
	data        []SS
	op          func(x, y SS) SS
	e           SS
	size        int
	height      int
	composition func(f, g F) F
	mapping     func(f F, x SS) SS
	id          F
	lazy        []F
}

func NewLazySegmentTree(
	n int,
	op func(x, y SS) SS,
	e SS,
	composition func(f, g F) F,
	mapping func(f F, x SS) SS,
	id F,
) LazySegmentTree {
	height := bits.Len64(uint64(n - 1))
	size := 1 << height
	data := make([]SS, size*2)
	for i := 0; i < size*2; i++ {
		data[i] = e
	}
	lazy := make([]F, size)
	for i := 0; i < size; i++ {
		lazy[i] = id
	}
	return LazySegmentTree{
		data:        data,
		op:          op,
		e:           e,
		size:        size,
		height:      height,
		composition: composition,
		mapping:     mapping,
		id:          id,
		lazy:        lazy,
	}
}
func (l *LazySegmentTree) Initialize(arr []SS) {
	for i, a := range arr {
		idx := i + l.size
		l.data[idx] = a
	}
	for i := l.size - 1; i > 0; i-- {
		l.update(i)
	}
}

func (l *LazySegmentTree) Set(pos int, x SS) {
	pos += l.size
	l.data[pos] = x
	for i := l.height; i > 0; i-- {
		l.push(pos >> i)
	}
	l.data[pos] = x
	for i := 1; i <= l.height; i++ {
		l.update(pos >> i)
	}
}

func (l *LazySegmentTree) allApply(i int, f F) {
	// 内部関数
	// data[i]を更新済みにして、lazyに更新内容保存
	l.data[i] = l.mapping(f, l.data[i])
	if i < l.size {
		l.lazy[i] = l.composition(f, l.lazy[i])
	}
}

func (l *LazySegmentTree) push(i int) {
	// 内部関数
	// 子を更新して,lazy伝搬させる
	ii := i << 1
	iii := i<<1 | 1
	_ = ii
	_ = iii
	l.allApply(int(i<<1), l.lazy[i])
	l.allApply(int(i<<1|1), l.lazy[i])
	l.lazy[i] = l.id
}

func (l *LazySegmentTree) update(i int) {
	l.data[i] = l.op(l.data[i<<1], l.data[i<<1|1])
}
func (l *LazySegmentTree) Update(pos int, x SS) {
	// data[pos] = x にする
	pos += l.size
	// bottom to top
	for i := l.height; i > 0; i-- {
		l.push(pos >> i)
	}
	l.data[pos] = x
	// top to bottom
	for i := 1; i <= l.height; i++ {
		l.update(pos >> i)
	}
}
func (l *LazySegmentTree) Apply(pos int, f F) {
	// data[p]にfを作用させる
	pos += l.size
	for i := l.height; i > 0; i-- {
		l.push(pos >> i)
	}
	l.data[pos] = l.mapping(f, l.data[pos])
	for i := 1; i <= l.height; i++ {
		l.update(pos >> i)
	}
}

func (l *LazySegmentTree) RangeApply(left, right int, f F) {
	// l, l+1, ..., r-1にfを作用させる
	if left == right {
		return
	}

	left += l.size
	right += l.size
	for i := l.height; i > 0; i-- {
		if ((left >> i) << i) != left {
			l.push(left >> i)
		}
		if ((right >> i) << i) != right {
			l.push((right - 1) >> i)
		}
	}
	left2, right2 := left, right
	for left < right {
		if left&1 == 1 {
			l.allApply(left, f)
			left++
		}
		if right&1 == 1 {
			right--
			l.allApply(right, f)
		}
		left >>= 1
		right >>= 1
	}
	left, right = left2, right2
	for i := 1; i <= l.height; i++ {
		if ((left >> i) << i) != left {
			l.update(left >> i)
		}
		if ((right >> i) << i) != right {
			l.update((right - 1) >> i)
		}
	}
}

func (l *LazySegmentTree) Get(pos int) SS {
	pos += l.size
	for i := l.height; i > 0; i-- {
		l.push(pos >> i)
	}
	return l.data[pos]
}

func (l *LazySegmentTree) Prod(left, right int) SS {
	if left == right {
		return l.e
	}
	left += l.size
	right += l.size
	for i := l.height; i > 0; i-- {
		if ((left >> i) << i) != left {
			l.push(left >> i)
		}
		if ((right >> i) << i) != right {
			l.push(right >> i)
			// l.push((right-1) >> i)
		}
	}
	sml := l.e
	smr := l.e
	for left < right {
		if left&1 == 1 {
			sml = l.op(sml, l.data[left])
			left++
		}
		if right&1 == 1 {
			right--
			smr = l.op(l.data[right], smr)
		}
		left >>= 1
		right >>= 1
	}
	return l.op(sml, smr)
}

func (l *LazySegmentTree) AllProd() SS {
	return l.data[1]
}
