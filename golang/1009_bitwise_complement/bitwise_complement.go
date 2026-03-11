// Package main ...
package main

func bitwiseComplement(n int) int {
	highBit := 0
	for i := 1; i <= 30; i++ {
		if n < 1<<i {
			break
		}
		highBit = i
	}
	mask := 1<<(highBit+1) - 1
	return n ^ mask
}

func main() {
	tests := []struct {
		n    int
		want int
	}{
		{5, 2},
		{7, 0},
		{10, 5},
	}
	for _, tt := range tests {
		got := bitwiseComplement(tt.n)
		if got != tt.want {
			panic("fail")
		}
	}
}
