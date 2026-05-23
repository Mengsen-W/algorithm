// Package main ...
package main

func check(nums []int) bool {
	n := len(nums)
	x := 0
	for i := 1; i < n; i++ {
		if nums[i] < nums[i-1] {
			x = i
			break
		}
	}
	if x == 0 {
		return true
	}
	for i := x + 1; i < n; i++ {
		if nums[i] < nums[i-1] {
			return false
		}
	}
	return nums[0] >= nums[n-1]
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}

	tests := []struct {
		nums []int
		ans  bool
	}{
		{[]int{3, 4, 5, 1, 2}, true},
		{[]int{2, 1, 3, 4}, false},
		{[]int{1, 2, 3}, true},
	}

	for _, test := range tests {
		assert(check(test.nums) == test.ans)
	}
}
