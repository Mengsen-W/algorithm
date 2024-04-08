/*
 * @Date: 2024-04-08
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-04-08
 * @FilePath: /algorithm/golang/2009_min_operations/min_operations.go
 */

// Package main ...
package main

import (
	"sort"
	"testing"

	"github.com/stretchr/testify/assert"
)

func minOperations(nums []int) int {
	n := len(nums)
	cnt := make(map[int]bool)
	for _, num := range nums {
		cnt[num] = true
	}
	sortedUniqueNums := []int{}
	for num := range cnt {
		sortedUniqueNums = append(sortedUniqueNums, num)
	}
	sort.Ints(sortedUniqueNums)
	res := n
	j := 0
	for i, left := range sortedUniqueNums {
		right := left + n - 1
		for j < len(sortedUniqueNums) && sortedUniqueNums[j] <= right {
			res = min(res, n-(j-i+1))
			j++
		}
	}
	return res
}

func main() {
	tests := []struct {
		nums []int
		ans  int
	}{
		{[]int{4, 2, 5, 3}, 0},
		{[]int{1, 2, 3, 5, 6}, 1},
		{[]int{1, 10, 100, 1000}, 3},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, minOperations(test.nums), index)
	}
}
