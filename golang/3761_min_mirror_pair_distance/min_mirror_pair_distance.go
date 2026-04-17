// Package main ...
package main

import "fmt"

func minMirrorPairDistance(nums []int) int {
	reverseNum := func(x int) int {
		y := 0
		for x > 0 {
			y = y*10 + x%10
			x /= 10
		}
		return y
	}

	prev := make(map[int]int)
	n := len(nums)
	ans := n + 1

	for i, x := range nums {
		if idx, exists := prev[x]; exists {
			if i-idx < ans {
				ans = i - idx
			}
		}
		prev[reverseNum(x)] = i
	}

	if ans == n+1 {
		return -1
	}
	return ans
}

func main() {
	tests := []struct {
		nums []int
		ans  int
	}{
		{[]int{12, 21, 45, 33, 54}, 1},
		{[]int{120, 21}, 1},
		{[]int{21, 120}, -1},
	}

	for index, test := range tests {
		if minMirrorPairDistance(test.nums) != test.ans {
			fmt.Errorf("%d", index)
		}
	}
}
