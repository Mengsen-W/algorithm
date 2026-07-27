// Package main ...
package main

import "fmt"

func maxProduct(nums []int) int {
	a, b := nums[0], nums[1]
	if a < b {
		a, b = b, a
	}
	for _, num := range nums[2:] {
		if num > a {
			a, b = num, a
		} else if num > b {
			b = num
		}
	}
	return (a - 1) * (b - 1)
}

func main() {
	tests := []struct {
		nums []int
		ans  int
	}{
		{[]int{3, 4, 5, 2}, 12},
		{[]int{1, 5, 4, 5}, 16},
		{[]int{3, 7}, 12},
	}

	for index, test := range tests {
		if maxProduct(test.nums) != test.ans {
			fmt.Println(index)
		}
	}
}
