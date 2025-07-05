package arraystring

// len(nums)/2 以上現れる数字を返す

func majorityElement(nums []int) int {
	m := len(nums) / 2
	if len(nums)%2 != 0 {
		m++
	}
	counter := make(map[int]int)
	for _, v := range nums {
		counter[v]++
		if counter[v] >= m {
			return v
		}
	}
	return 0
}

// テクニカルな回答(そんな速くなかった)
// 一番出現回数が配列の過半数を占めるってことを用いる。
// 過半数を占めない場合は使えない。
// 使えない例: [1,1,2,2,3,3,3,1,1] want=1, got=3

// 過半数を占める
// -> 出現回数が最大の数字をxとして、それ以外を全てyに置き換えても問題ない。(yは最大でも過半数を超えない)
// 例: 1が3個,2が1個,3が1個からなる配列を、xが3個,yが2個からなる配列に変換
// [1,1,1,2,3] -> [x,x,x,y,y]
// -> 出てくる数字が2つになったので、出現回数が多い方を答えれば良い
// ただ、どれをxにするかは分からないので、それまで見てきた配列で一番多い数をcandidateとして更新して行けば、最終的にcandidateはxと一致する
func majorityElement2(nums []int) int {
	var candidate int
	var count int
	for _, v := range nums {
		if count == 0 {
			candidate = v
		}
		if candidate == v {
			count++
		} else {
			count--
		}
	}
	return candidate
}
