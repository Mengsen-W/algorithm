// Package main ...
package main

import "fmt"

func uniformArray(nums1 []int) bool {
	mn := nums1[0]
	hasOdd := false
	for _, v := range nums1 {
		if v < mn {
			mn = v
		}
		if v%2 == 1 {
			hasOdd = true
		}
	}
	if mn%2 == 1 {
		return true
	}
	return !hasOdd
}

func main() {
	tests := []struct {
		nums1 []int
		ans   bool
	}{
		{[]int{1, 4, 7}, true},
		{[]int{2, 3}, false},
		{[]int{4, 6}, true},
	}

	for index, test := range tests {
		if uniformArray(test.nums1) != test.ans {
			fmt.Printf("Test %d failed\n", index)
		}
	}
}
