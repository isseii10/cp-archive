package gopkg

type BinaryIndexedTree struct {
	bit []int
}

func NewBIT(n int) BinaryIndexedTree {
	// 1-indexedなのでn+1
	bit := make([]int, n+1)
	for i := 0; i < n+1; i++ {
		bit[i] = 0
	}
	return BinaryIndexedTree{bit: bit}
}
func (b *BinaryIndexedTree) Add(pos, v int) {
	pos++
	for pos < len(b.bit) {
		b.bit[pos] += v
		pos += pos & -pos
	}
}
func (b *BinaryIndexedTree) Get(pos int) int {
	pos++
	ret := 0
	for pos > 0 {
		ret += b.bit[pos]
		pos -= pos & -pos
	}
	return ret
}
