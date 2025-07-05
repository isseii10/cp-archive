package gopkg

import (
	"testing"

	"github.com/google/go-cmp/cmp"
)

func TestDivisor(t *testing.T) {
	tests := []struct {
		name string
		n    int
		want []int
	}{
		{
			name: "10",
			n:    10,
			want: []int{1, 2, 5, 10},
		},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			got := divisor(tt.n)
			if diff := cmp.Diff(got, tt.want); diff != "" {
				t.Errorf("divisor() mismatch (-got +want):\n%s", diff)
			}
		})
	}
}
