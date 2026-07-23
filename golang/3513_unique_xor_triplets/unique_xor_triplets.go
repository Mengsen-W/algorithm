// Package main ...
package main

import "fmt"

func uniqueXorTriplets(nums []int) int {
	n := len(nums)
	if n <= 2 {
		return n
	}
	ans := 1
	for ans <= n {
		ans <<= 1
	}
	return ans
}

func main() {
	tests := []struct {
		nums []int
		ans  int
	}{
		{[]int{1, 2}, 2},
		{[]int{3, 1, 2}, 4},
	}

	for index, test := range tests {
		result := uniqueXorTriplets(test.nums)
		if result != test.ans {
			fmt.Printf("Test %d failed: expected %d, got %d\n", index+1, test.ans, result)
		}
	}
}
