package twopointers

import "testing"

func Test_isPalindrome(t *testing.T) {
	type args struct {
		s string
	}
	tests := []struct {
		name string
		args args
		want bool
	}{
		{
			name: "case 1",
			args: args{s: "A man, a plan, a canal: Panama"},
			want: true,
		},
		{
			name: "case 2",
			args: args{s: "race a car"},
			want: false,
		},
		{
			name: "case 3",
			args: args{s: " "},
			want: true,
		},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			if got := isPalindrome(tt.args.s); got != tt.want {
				t.Errorf("isPalindrome() = %v, want %v", got, tt.want)
			}
		})
	}
}

func Test_isAlphanumeric(t *testing.T) {
	type args struct {
		r rune
	}
	tests := []struct {
		name string
		args args
		want bool
	}{
		// TODO: Add test cases.
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			if got := isAlphanumeric(tt.args.r); got != tt.want {
				t.Errorf("isAlphanumeric() = %v, want %v", got, tt.want)
			}
		})
	}
}
