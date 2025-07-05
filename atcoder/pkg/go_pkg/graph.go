package gopkg

type edge struct {
	to   int
	cost int
}
type Edges []edge
type Graph []Edges

func NewGraph(n int) Graph {
	return make([]Edges, n)
}

// Edgesのcostでのsortを可能にするためのsort.interfaceを実装
func (e Edges) Len() int           { return len(e) }
func (e Edges) Swap(i, j int)      { e[i], e[j] = e[j], e[i] }
func (e Edges) Less(i, j int) bool { return e[i].cost < e[j].cost }

// asc: sort.Sort(graph[i])
// desc: sort.Sort(sort.Reverse(graph[i]))
