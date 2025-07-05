package linkedlist

import (
	"reflect"
	"testing"
)

func Test_mergeTwoLists(t *testing.T) {
	type args struct {
		list1 *ListNode
		list2 *ListNode
	}
	tests := []struct {
		name string
		args args
		want *ListNode
	}{
		{
			name: "case 1",
			args: func() args {
				l13 := &ListNode{Val: 4}
				l12 := &ListNode{Val: 2, Next: l13}
				l11 := &ListNode{Val: 1, Next: l12}

				l23 := &ListNode{Val: 4}
				l22 := &ListNode{Val: 3, Next: l23}
				l21 := &ListNode{Val: 1, Next: l22}
				return args{list1: l11, list2: l21}
			}(),
			want: func() *ListNode {
				l6 := &ListNode{Val: 4}
				l5 := &ListNode{Val: 4, Next: l6}
				l4 := &ListNode{Val: 3, Next: l5}
				l3 := &ListNode{Val: 2, Next: l4}
				l2 := &ListNode{Val: 1, Next: l3}
				l1 := &ListNode{Val: 1, Next: l2}
				return l1
			}(),
		},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			if got := mergeTwoLists(tt.args.list1, tt.args.list2); !reflect.DeepEqual(got, tt.want) {
				t.Errorf("mergeTwoLists() = %v, want %v", got, tt.want)
			}
		})
	}
}

func Test_mergeTwoLists2(t *testing.T) {
	type args struct {
		list1 *ListNode
		list2 *ListNode
	}
	tests := []struct {
		name string
		args args
		want *ListNode
	}{
		{
			name: "case 1",
			args: func() args {
				l13 := &ListNode{Val: 4}
				l12 := &ListNode{Val: 2, Next: l13}
				l11 := &ListNode{Val: 1, Next: l12}

				l23 := &ListNode{Val: 4}
				l22 := &ListNode{Val: 3, Next: l23}
				l21 := &ListNode{Val: 1, Next: l22}
				return args{list1: l11, list2: l21}
			}(),
			want: func() *ListNode {
				l6 := &ListNode{Val: 4}
				l5 := &ListNode{Val: 4, Next: l6}
				l4 := &ListNode{Val: 3, Next: l5}
				l3 := &ListNode{Val: 2, Next: l4}
				l2 := &ListNode{Val: 1, Next: l3}
				l1 := &ListNode{Val: 1, Next: l2}
				return l1
			}(),
		},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			if got := mergeTwoLists2(tt.args.list1, tt.args.list2); !reflect.DeepEqual(got, tt.want) {
				t.Errorf("mergeTwoLists2() = %v, want %v", got, tt.want)
			}
		})
	}
}
