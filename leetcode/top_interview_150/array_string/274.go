package arraystring

import "sort"

func hIndex(citations []int) int {
	papersByCitations := make([]int, 5001)
	for _, v := range citations {
		papersByCitations[v]++
	}
	h := 0
	for i := len(papersByCitations) - 1; i > 0; i-- {
		if papersByCitations[i] >= i {
			h = i
			break
		}
		papersByCitations[i-1] += papersByCitations[i]
	}
	return h
}

// 降順ソートして見ていく
func hIndex_2(citations []int) int {
	hIndex := 0
	sort.Slice(citations, func(i, j int) bool {
		return citations[i] > citations[j]
	})

	for i := 0; i < len(citations); i++ {
		if i+1 <= citations[i] {
			hIndex = i + 1
		} else {
			return hIndex
		}
	}
	return hIndex
}

// 1-indexedで考える
// 1番目が1以上ということは、降順ソートしているので[1以上, 1以下, 1以下, ...]になる。この時1回引用されているものは少なくとも1つ存在する。
// 2番目が2以上ということは、[2以上, 2以上, 2以下, ...]になる。
