package arraystring

// nums1を書き換える
// nums1はすでにm+nのlength (m以上は0埋めされている)

func merge(nums1 []int, m int, nums2 []int, n int) {
	now := m + n - 1
	idx1 := m - 1
	idx2 := n - 1
	for now >= 0 {
		// nums2がなくなったら終わり
		if idx2 < 0 {
			break
		}
		// nums1がなくなったらnums2から入れるだけ
		if idx1 < 0 {
			nums1[now] = nums2[idx2]
			idx2--
		} else { // nums1, nums2どっちも残っているので比較
			if nums1[idx1] > nums2[idx2] {
				nums1[now] = nums1[idx1]
				idx1--
			} else {
				nums1[now] = nums2[idx2]
				idx2--
			}
		}
		now--
	}
}

// nums1 = [1,5,6,-1,-1,-1]
// nums2 = [1,3,6]
// tmp = -1

// i=0
// nums1 = [1,5,6,-1,-1,-1]
// nums2 = [1,3,6]
// tmp = -1

// i=1
// nums1 = [1,1,6,-1,-1,-1]
// nums2 = [-1,3,6]
// tmp = 5

// i=2
// nums1 = [1,1,3,-1,-1,-1]
// nums2 = [-1,5,6]
// tmp = 6
