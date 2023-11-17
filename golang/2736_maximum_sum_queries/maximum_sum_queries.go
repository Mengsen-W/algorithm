/*
 * @Date: 2023-11-17
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-11-17
 * @FilePath: /algorithm/golang/2736_maximum_sum_queries/maximum_sum_queries.go
 */

// Package main ...
package main

import (
	"sort"
	"testing"

	"github.com/stretchr/testify/assert"
)

func maximumSumQueries(nums1 []int, nums2 []int, queries [][]int) []int {
	sortedNums := make([][]int, len(nums1))
	for i := 0; i < len(nums1); i++ {
		sortedNums[i] = []int{nums1[i], nums2[i]}
	}
	sort.Slice(sortedNums, func(i, j int) bool {
		return sortedNums[i][0] > sortedNums[j][0]
	})

	sortedQueries := make([][]int, len(queries))
	for i := 0; i < len(queries); i++ {
		sortedQueries[i] = []int{i, queries[i][0], queries[i][1]}
	}
	sort.Slice(sortedQueries, func(i, j int) bool {
		return sortedQueries[i][1] > sortedQueries[j][1]
	})

	stack := [][]int{}
	answer := make([]int, len(queries))
	for i := 0; i < len(queries); i++ {
		answer[i] = -1
	}
	j := 0
	for _, q := range sortedQueries {
		i, x, y := q[0], q[1], q[2]
		for j < len(sortedNums) && sortedNums[j][0] >= x {
			num1, num2 := sortedNums[j][0], sortedNums[j][1]
			for len(stack) > 0 && stack[len(stack)-1][1] <= num1+num2 {
				stack = stack[:len(stack)-1]
			}
			if len(stack) == 0 || stack[len(stack)-1][0] < num2 {
				stack = append(stack, []int{num2, num1 + num2})
			}
			j++
		}
		k := sort.Search(len(stack), func(i int) bool {
			return stack[i][0] >= y
		})

		if k < len(stack) {
			answer[i] = stack[k][1]
		}
	}
	return answer
}

func main() {
	tests := []struct {
		nums1   []int
		nums2   []int
		queries [][]int
		ans     []int
	}{
		{[]int{4, 3, 1, 2}, []int{2, 4, 9, 5}, [][]int{{4, 1}, {1, 3}, {2, 5}}, []int{6, 10, 7}},
		{[]int{3, 2, 5}, []int{2, 3, 4}, [][]int{{4, 4}, {3, 2}, {1, 1}}, []int{9, 9, 9}},
		{[]int{2, 1}, []int{2, 3}, [][]int{{3, 3}}, []int{-1}},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, maximumSumQueries(test.nums1, test.nums2, test.queries), index)
	}
}
