package arraystring

import (
	"testing"

	"github.com/google/go-cmp/cmp"
)

func Test_removeDuplicates2(t *testing.T) {
	type args struct {
		nums []int
	}
	tests := []struct {
		name     string
		args     args
		want     int
		wantNums []int
	}{
		{
			name:     "case 1",
			args:     args{nums: []int{1, 1, 1, 2, 2, 3}},
			want:     5,
			wantNums: []int{1, 1, 2, 2, 3},
		},
		{
			name:     "case 2",
			args:     args{nums: []int{0, 0, 1, 1, 1, 1, 2, 3, 3}},
			want:     7,
			wantNums: []int{0, 0, 1, 1, 2, 3, 3},
		},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			nums := tt.args.nums
			got := removeDuplicates2(nums)
			if got != tt.want {
				t.Errorf("removeDuplicates2() = %v, want %v", got, tt.want)
			}
			if diff := cmp.Diff(nums[:got], tt.wantNums); diff != "" {
				t.Errorf("+got, -want\n%s", diff)
			}
		})
	}
}
