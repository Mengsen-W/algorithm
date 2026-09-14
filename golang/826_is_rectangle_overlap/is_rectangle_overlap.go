// Package main ...
package main

import "fmt"

func isRectangleOverlap(rec1 []int, rec2 []int) bool {
	return min(rec1[2], rec2[2]) > max(rec1[0], rec2[0]) &&
		min(rec1[3], rec2[3]) > max(rec1[1], rec2[1])
}

func main() {
	tests := []struct {
		rec1 []int
		rec2 []int
		ans  bool
	}{
		{[]int{0, 0, 2, 2}, []int{1, 1, 3, 3}, true},
		{[]int{0, 0, 1, 1}, []int{1, 0, 2, 1}, false},
		{[]int{0, 0, 1, 1}, []int{2, 2, 3, 3}, false},
	}

	for index, test := range tests {
		if isRectangleOverlap(test.rec1, test.rec2) != test.ans {
			fmt.Println(index)
		}
	}
}
