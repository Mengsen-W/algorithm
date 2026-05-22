// Package main ...
package main

func search(nums []int, target int) int {
	last := nums[len(nums)-1]
	left, right := -1, len(nums)-1 // 开区间 (-1, n-1)
	for left+1 < right {           // 开区间不为空
		mid := left + (right-left)/2
		x := nums[mid]
		if target > last && x <= last { // target 在第一段，x 在第二段
			right = mid // 下轮循环去左边找
		} else if x > last && target <= last { // x 在第一段，target 在第二段
			left = mid // 下轮循环去右边找
		} else if x >= target { // 否则，x 和 target 在同一段，这就和方法一的 lowerBound 一样了
			right = mid
		} else {
			left = mid
		}
	}
	if nums[right] != target {
		return -1
	}
	return right
}

func main() {
	tests := []struct {
		nums   []int
		target int
		ans    int
	}{
		{[]int{4, 5, 6, 7, 0, 1, 2}, 0, 4},
		{[]int{4, 5, 6, 7, 0, 1, 2}, 3, -1},
		{[]int{1}, 0, -1},
	}

	for _, test := range tests {
		if search(test.nums, test.target) != test.ans {
			panic("error")
		}
	}
}
