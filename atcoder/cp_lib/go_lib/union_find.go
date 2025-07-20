package golib

type UnionFind struct {
	n    int
	root []int
}

func NewUnionFind(n int) UnionFind {
	root := make([]int, n)
	for i := 0; i < n; i++ {
		root[i] = -1
	}
	uf := UnionFind{n: n, root: root}
	return uf
}
func (u *UnionFind) Find(x int) int {
	if u.root[x] < 0 {
		return x
	}
	u.root[x] = u.Find(u.root[x])
	return u.root[x]
}
func (u *UnionFind) Union(x, y int) {
	x = u.Find(x)
	y = u.Find(y)
	if x == y {
		return
	}
	if -u.root[x] < -u.root[y] {
		x, y = y, x
	} // xの方がサイズ大きいように
	u.root[x] += u.root[y]
	u.root[y] = x
}
func (u *UnionFind) IsSame(x, y int) bool {
	return u.Find(x) == u.Find(y)
}
func (u *UnionFind) Size(x int) int {
	return -u.root[u.Find(x)]
}
