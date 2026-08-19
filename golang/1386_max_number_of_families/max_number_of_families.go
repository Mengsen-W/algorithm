// Package main ...
package main

import "fmt"

func maxNumberOfFamilies(n int, reservedSeats [][]int) int {
	left := 0b11110000
	middle := 0b11000011
	right := 0b00001111

	occupied := make(map[int]int)
	for _, seat := range reservedSeats {
		if seat[1] >= 2 && seat[1] <= 9 {
			occupied[seat[0]] |= (1 << (seat[1] - 2))
		}
	}

	ans := (n - len(occupied)) * 2
	for _, bitmask := range occupied {
		if (bitmask|left) == left ||
			(bitmask|middle) == middle ||
			(bitmask|right) == right {
			ans++
		}
	}
	return ans
}

func main() {
	tests := []struct {
		n             int
		reservedSeats [][]int
		ans           int
	}{
		{3, [][]int{{1, 2}, {1, 3}, {1, 8}, {2, 6}, {3, 1}, {3, 10}}, 4},
		{2, [][]int{{2, 1}, {1, 8}, {2, 6}}, 2},
		{4, [][]int{{4, 3}, {1, 4}, {4, 6}, {1, 7}}, 4},
	}

	for _, test := range tests {
		if got := maxNumberOfFamilies(test.n, test.reservedSeats); got != test.ans {
			panic(fmt.Sprintf("test %v: got %v, want %v", test, got, test.ans))
		}
	}
}
