// Package main ...
package main

import (
	"fmt"
	"reflect"
)

func targetIndices(nums []int, target int) []int {
	cnt1 := 0 // 小于 target 的元素数量
	cnt2 := 0 // 等于 target 的元素数量

	for _, num := range nums {
		if num < target {
			cnt1++
		} else if num == target {
			cnt2++
		}
	}

	res := make([]int, cnt2)
	for i := 0; i < cnt2; i++ {
		res[i] = cnt1 + i
	}
	return res
}

func main() {
	tests := []struct {
		nums   []int
		target int
		ans    []int
	}{
		{[]int{1, 2, 5, 2, 3}, 2, []int{1, 2}},
		{[]int{1, 2, 5, 2, 3}, 3, []int{3}},
		{[]int{1, 2, 5, 2, 3}, 5, []int{4}},
		{[]int{1, 2, 5, 2, 3}, 4, []int{}},
	}

	for index, test := range tests {
		if got := targetIndices(test.nums, test.target); !reflect.DeepEqual(got, test.ans) {
			fmt.Printf("test %d failed: got %v, want %v", index, got, test.ans)
		}
	}
}
