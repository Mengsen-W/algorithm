// Package main ...
package main

import "reflect"

func pivotArray(nums []int, pivot int) []int {
	reverse := func(arr []int) {
		for i, j := 0, len(arr)-1; i < j; i, j = i+1, j-1 {
			arr[i], arr[j] = arr[j], arr[i]
		}
	}
	n := len(nums)
	ans := make([]int, n)
	for i := range ans {
		ans[i] = pivot
	}
	left, right := 0, n-1
	for i := 0; i < n; i++ {
		if nums[i] < pivot {
			ans[left] = nums[i]
			left++
		} else if nums[i] > pivot {
			ans[right] = nums[i]
			right--
		}
	}
	reverse(ans[right+1:])
	return ans
}

func main() {
	tests := []struct {
		nums  []int
		pivot int
		ans   []int
	}{
		{[]int{9, 12, 5, 10, 14, 3, 10}, 10, []int{9, 5, 3, 10, 10, 12, 14}},
		{[]int{-3, 4, 3, 2}, 2, []int{-3, 2, 4, 3}},
	}

	for _, test := range tests {
		if ans := pivotArray(test.nums, test.pivot); !reflect.DeepEqual(ans, test.ans) {
			println("ans: ", ans, " expect: ", test.ans)
		}
	}
}
