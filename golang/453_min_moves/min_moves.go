// Package main ...
package main

func minMoves(nums []int) (ans int) {
	min := nums[0]
	for _, num := range nums[1:] {
		if num < min {
			min = num
		}
	}
	for _, num := range nums {
		ans += num - min
	}
	return
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}

	for _, test := range []struct {
		nums []int
		ans  int
	}{
		{[]int{1, 2, 3}, 2},
		{[]int{1, 1, 1}, 0},
	} {
		assert(minMoves(test.nums) == test.ans)
	}
}
