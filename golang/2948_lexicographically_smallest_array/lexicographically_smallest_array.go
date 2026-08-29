// Package main ...
package main

import (
	"fmt"
	"reflect"
	"sort"
)

func lexicographicallySmallestArray(nums []int, limit int) []int {
	n := len(nums)
	ans := make([]int, n)

	// 将元素值与原下标绑定
	arr := make([][2]int, n)
	for i, x := range nums {
		arr[i] = [2]int{x, i}
	}

	// 按元素值升序排序
	sort.Slice(arr, func(i, j int) bool {
		return arr[i][0] < arr[j][0]
	})

	values := make([]int, n)
	indices := make([]int, n)

	for i, p := range arr {
		values[i] = p[0]
		indices[i] = p[1]
	}

	i := 0
	for i < n {
		start := i

		// 当前连通块中的原下标
		groupIndices := []int{}

		// 当前连通块中的元素值
		groupValues := []int{}

		for i < n && (i == start || values[i]-values[i-1] <= limit) {
			groupIndices = append(groupIndices, indices[i])
			groupValues = append(groupValues, values[i])
			i++
		}

		// 由于元素值数组已经有序，这里不需要再排序
		sort.Ints(groupIndices)

		// 为得到字典序最小的结果，将较小元素放到较小下标处
		for k := 0; k < len(groupIndices); k++ {
			ans[groupIndices[k]] = groupValues[k]
		}
	}

	return ans
}

func main() {
	tests := []struct {
		nums  []int
		limit int
		ans   []int
	}{
		{[]int{1, 5, 3, 9, 8}, 2, []int{1, 3, 5, 8, 9}},
		{[]int{1, 7, 6, 18, 2, 1}, 3, []int{1, 6, 7, 18, 1, 2}},
		{[]int{1, 7, 28, 19, 10}, 3, []int{1, 7, 28, 19, 10}},
	}

	for _, test := range tests {
		if got := lexicographicallySmallestArray(test.nums, test.limit); !reflect.DeepEqual(got, test.ans) {
			fmt.Printf("expected %v, got %v\n", test.ans, got)
		}
	}
}
