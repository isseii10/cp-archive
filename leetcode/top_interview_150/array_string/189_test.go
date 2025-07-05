package arraystring

import (
	"testing"

	"github.com/google/go-cmp/cmp"
)

func Test_rotate(t *testing.T) {
	type args struct {
		nums []int
		k    int
	}
	tests := []struct {
		name     string
		args     args
		wantNums []int
	}{
		{
			name:     "case 1",
			args:     args{nums: []int{1, 2, 3, 4, 5, 6, 7}, k: 3},
			wantNums: []int{5, 6, 7, 1, 2, 3, 4},
		},
		{
			name:     "case 2",
			args:     args{nums: []int{-1, -100, 3, 99}, k: 2},
			wantNums: []int{3, 99, -1, -100},
		},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			rotate(tt.args.nums, tt.args.k)
			if diff := cmp.Diff(tt.args.nums, tt.wantNums); diff != "" {
				t.Errorf("-got, +want\n%s", diff)
			}
		})
	}
}
