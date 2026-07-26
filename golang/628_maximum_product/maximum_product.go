// Package main ...
package main

import (
	"fmt"
	"math"
)

func maximumProduct(nums []int) int {
	max := func(a, b int) int {
		if a > b {
			return a
		}
		return b
	}
	// 最小的和第二小的
	min1, min2 := math.MaxInt64, math.MaxInt64
	// 最大的、第二大的和第三大的
	max1, max2, max3 := math.MinInt64, math.MinInt64, math.MinInt64

	for _, x := range nums {
		if x < min1 {
			min2 = min1
			min1 = x
		} else if x < min2 {
			min2 = x
		}

		if x > max1 {
			max3 = max2
			max2 = max1
			max1 = x
		} else if x > max2 {
			max3 = max2
			max2 = x
		} else if x > max3 {
			max3 = x
		}
	}

	return max(min1*min2*max1, max1*max2*max3)
}

func main() {
	tests := []struct {
		nums []int
		ans  int
	}{
		{[]int{1, 2, 3}, 6},
		{[]int{1, 2, 3, 4}, 24},
		{[]int{-1, -2, -3}, -6},
	}

	for index, test := range tests {
		if maximumProduct(test.nums) != test.ans {
			fmt.Println(index)
		}
	}
}
