package intervals

import "sort"

func merge(intervals [][]int) [][]int {
	sort.Slice(intervals, func(i, j int) bool { return intervals[i][0] < intervals[j][0] })
	res := make([][]int, 0, len(intervals))
	// top means len(res) -1
	top := -1
	for i, v := range intervals {
		if i == 0 {
			res = append(res, v)
			top = 0
			continue
		}
		if v[0] <= res[top][1] && res[top][1] < v[1] {
			// merge
			res[top][1] = v[1]
		}
		if res[top][1] < v[0] {
			// cannot merge
			res = append(res, v)
			top++
		}
	}
	return res
}
