package graphgeneral

func numIslands(grid [][]byte) int {
	H := len(grid)
	W := len(grid[0])
	visited := make([][]bool, H)
	for i := range visited {
		visited[i] = make([]bool, W)
	}

	d := [][]int{{0, 1}, {0, -1}, {1, 0}, {-1, 0}}

	var dfs func(ph, pw int)
	dfs = func(ph, pw int) {
		visited[ph][pw] = true
		for _, dhdw := range d {
			ch := ph + dhdw[0]
			cw := pw + dhdw[1]
			if isInside(ch, cw, H, W) && grid[ch][cw] == '1' && !visited[ch][cw] {
				dfs(ch, cw)
			}
		}
	}
	ans := 0
	for i := range grid {
		for j := range grid[i] {
			if grid[i][j] == '1' && !visited[i][j] {
				dfs(i, j)
				ans++
			}
		}
	}

	return ans
}

func isInside(h, w, H, W int) bool {
	return 0 <= h && h < H && 0 <= w && w < W
}
