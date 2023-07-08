/*
 * @Date: 2023-07-08
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-07-08
 * @FilePath: /algorithm/golang/167_two_sum/two_sum.go
 */

// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func twoSum(numbers []int, target int) []int {
	low, high := 0, len(numbers)-1
	for low < high {
		sum := numbers[low] + numbers[high]
		if sum == target {
			return []int{low + 1, high + 1}
		} else if sum < target {
			low++
		} else {
			high--
		}
	}
	return []int{-1, -1}
}

func main() {
	testMap := []struct {
		number []int
		target int
		ans    []int
	}{
		{[]int{2, 7, 11, 15}, 9, []int{1, 2}},
		{[]int{2, 3, 4}, 6, []int{1, 3}},
		{[]int{-1, 0}, -1, []int{1, 2}},
	}

	for _, item := range testMap {
		assert.Empty(&testing.T{}, twoSum(item.number, item.target), item.ans)
	}
}
