package graphgeneral

func solve(board [][]byte) {
	H := len(board)
	W := len(board[0])

	visited := make([][]int, H)
	// 0: 未訪問, 1:訪問済み, 2:書き換え済み
	for i := range visited {
		visited[i] = make([]int, W)
	}

	d := [][]int{{0, 1}, {0, -1}, {1, 0}, {-1, 0}}

	var dfs func(ph, pw int) bool
	dfs = func(ph, pw int) bool {
		visited[ph][pw] = 1
		ok := true
		for _, dhdw := range d {
			ch := ph + dhdw[0]
			cw := pw + dhdw[1]
			if !isInside(ch, cw, H, W) {
				ok = false
				continue
			}
			if visited[ch][cw] == 0 && board[ch][cw] == 'O' {
				// ok && dfsだとokがfalseの時にdfsは実行されない
				ok = dfs(ch, cw) && ok
			}
		}
		return ok
	}

	var writeX func(ph, pw int)
	writeX = func(ph, pw int) {
		board[ph][pw] = 'X'
		visited[ph][pw] = 2
		for _, dhdw := range d {
			ch := ph + dhdw[0]
			cw := pw + dhdw[1]
			if isInside(ch, cw, H, W) && visited[ch][cw] != 2 && board[ch][cw] == 'O' {
				writeX(ch, cw)
			}
		}
	}

	for i := range board {
		for j := range board[i] {
			if visited[i][j] == 0 && board[i][j] == 'O' {
				if ok := dfs(i, j); ok {
					writeX(i, j)
				}
			}
		}
	}
}
