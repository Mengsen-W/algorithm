// Package main ...
package main

func minimizeSum(nums []int) int {
	mn1, mn2, mn3 := int(^uint(0)>>1), int(^uint(0)>>1), int(^uint(0)>>1)
	mx1, mx2, mx3 := -mn1-1, -mn1-1, -mn1-1

	for _, x := range nums {
		if x < mn1 {
			mn1, mn2, mn3 = x, mn1, mn2
		} else if x < mn2 {
			mn2, mn3 = x, mn2
		} else if x < mn3 {
			mn3 = x
		}

		if x > mx1 {
			mx1, mx2, mx3 = x, mx1, mx2
		} else if x > mx2 {
			mx2, mx3 = x, mx2
		} else if x > mx3 {
			mx3 = x
		}
	}

	min := func(a, b int) int {
		if a < b {
			return a
		}
		return b
	}

	return min(min(mx1-mn3, mx2-mn2), mx3-mn1)
}

func main() {
	tests := []struct {
		nums []int
		ans  int
	}{
		{[]int{1, 4, 3}, 0},
		{[]int{1, 4, 7, 8, 5}, 3},
	}

	for index, test := range tests {
		if test.ans != minimizeSum(test.nums) {
			panic(index)
		}
	}
}
