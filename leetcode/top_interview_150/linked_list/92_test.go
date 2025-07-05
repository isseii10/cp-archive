package linkedlist

import (
	"testing"

	"github.com/google/go-cmp/cmp"
)

func Test_reverseBetween(t *testing.T) {
	type args struct {
		head  *ListNode
		left  int
		right int
	}
	tests := []struct {
		name string
		args args
		want *ListNode
	}{
		{
			name: "case 1",
			args: args{
				left:  1,
				right: 2,
				head:  &ListNode{Val: 3, Next: &ListNode{Val: 5}},
			},
			want: &ListNode{Val: 5, Next: &ListNode{Val: 3}},
		},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			got := reverseBetween(tt.args.head, tt.args.left, tt.args.right)
			if d := cmp.Diff(got, tt.want); d != "" {
				t.Errorf("-got, +want\n%s", d)
			}
		})
	}
}
