package golib

import (
	"reflect"
	"testing"
)

func Test_divisor(t *testing.T) {
	type args struct {
		n int
	}
	tests := []struct {
		name string
		args args
		want []int
	}{
		{
			name: "10",
			args: args{n: 10},
			want: []int{1, 2, 5, 10},
		},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			if got := divisor(tt.args.n); !reflect.DeepEqual(got, tt.want) {
				t.Errorf("divisor() = %v, want %v", got, tt.want)
			}
		})
	}
}
