// Package main ...
package main

import (
	"fmt"
	"math"
	"sort"
)

func gcd(a, b int) int {
	for b != 0 {
		a, b = b, a%b
	}
	return a
}

func gcdSum(nums []int) int64 {
	n := len(nums)
	mx := make([]int, n)
	prefixMax := math.MinInt32

	for i, x := range nums {
		if x > prefixMax {
			prefixMax = x
		}
		mx[i] = prefixMax
	}

	prefixGcd := make([]int, n)
	for i := 0; i < n; i++ {
		prefixGcd[i] = gcd(nums[i], mx[i])
	}
	sort.Ints(prefixGcd)

	var ans int64 = 0
	left, right := 0, n-1
	for left < right {
		ans += int64(gcd(prefixGcd[left], prefixGcd[right]))
		left++
		right--
	}

	return ans
}

func main() {
	tests := []struct {
		nums []int
		ans  int64
	}{
		{[]int{2, 6, 4}, 2},
		{[]int{3, 6, 2, 8}, 5},
	}

	for _, test := range tests {
		got := gcdSum(test.nums)
		if got != test.ans {
			fmt.Printf("gcdSum(%v) = %d, want %d\n", test.nums, got, test.ans)
		}
	}
}
