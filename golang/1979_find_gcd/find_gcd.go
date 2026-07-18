// Package main ...
package main

import "fmt"

func findGCD(nums []int) int {
	gcd := func(a, b int) int {
		for b != 0 {
			a, b = b, a%b
		}
		return a
	}

	mx, mn := nums[0], nums[0]
	for _, num := range nums {
		mn = min(mn, num)
		mx = max(mx, num)
	}
	return gcd(mx, mn)
}

func main() {
	tests := []struct {
		nums []int
		ans  int
	}{
		{[]int{2, 5, 6, 9, 10}, 2},
		{[]int{7, 5, 6, 8, 3}, 1},
		{[]int{3, 3}, 3},
	}

	for _, test := range tests {
		got := findGCD(test.nums)
		if got != test.ans {
			fmt.Printf("findGCD(%v) = %d, want %d\n", test.nums, got, test.ans)
		}
	}
}
