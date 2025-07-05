package arraystring

import (
	"testing"

	"github.com/google/go-cmp/cmp"
)

func Test_removeDuplicates(t *testing.T) {
	type args struct {
		nums []int
	}
	tests := []struct {
		name string
		args args
		want int
	}{
		// TODO: Add test cases.
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			if got := removeDuplicates(tt.args.nums); got != tt.want {
				t.Errorf("removeDuplicates() = %v, want %v", got, tt.want)
			}
		})
	}
}

func Test_removeDuplicates_2(t *testing.T) {
	type args struct {
		nums []int
	}
	tests := []struct {
		name      string
		args      args
		want      int
		wantArray []int
	}{
		{
			name:      "case2",
			args:      args{nums: []int{0, 0, 1, 1, 1, 2, 2, 3, 3, 4}},
			want:      5,
			wantArray: []int{0, 1, 2, 3, 4},
		},
		{
			name:      "wrong case1",
			args:      args{nums: []int{1, 2}},
			want:      2,
			wantArray: []int{1, 2},
		},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			got := removeDuplicates_2(tt.args.nums)
			if got != tt.want {
				t.Errorf("removeDuplicates_2() = %v, want %v", got, tt.want)
			}
			if diff := cmp.Diff(tt.args.nums[:got], tt.wantArray); diff != "" {
				t.Errorf("-got, +want\n%s", diff)
			}
		})
	}
}
