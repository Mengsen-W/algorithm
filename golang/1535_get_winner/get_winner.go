/*
 * @Date: 2024-05-19
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-05-19
 * @FilePath: /algorithm/golang/1535_get_winner/get_winner.go
 */

// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func getWinner(arr []int, k int) int {
	prev := max(arr[0], arr[1])
	if k == 1 {
		return prev
	}
	consecutive := 1
	maxNum := prev
	for i := 2; i < len(arr); i++ {
		curr := arr[i]
		if prev > curr {
			consecutive++
			if consecutive == k {
				return prev
			}
		} else {
			prev = curr
			consecutive = 1
		}
		maxNum = max(maxNum, curr)
	}
	return maxNum
}

func main() {
	tests := []struct {
		arr []int
		k   int
		ans int
	}{
		{[]int{2, 1, 3, 5, 4, 6, 7}, 2, 5},
		{[]int{3, 2, 1}, 10, 3},
		{[]int{1, 9, 8, 2, 3, 7, 6, 4, 5}, 7, 9},
		{[]int{1, 11, 22, 33, 44, 55, 66, 77, 88, 99}, 1000000000, 99},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, getWinner(test.arr, test.k), index)
	}
}
