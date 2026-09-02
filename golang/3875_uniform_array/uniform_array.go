// Package main ...
package main

import "fmt"

func uniformArray(nums1 []int) bool {
	return true
}

func main() {
	tests := []struct {
		nums1 []int
		ans   bool
	}{
		{[]int{2, 3}, true},
		{[]int{4, 6}, true},
	}

	for index, test := range tests {
		if uniformArray(test.nums1) != test.ans {
			fmt.Printf("Test %d failed\n", index)
		}
	}
}
