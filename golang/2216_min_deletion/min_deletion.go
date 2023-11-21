/*
 * @Date: 2023-11-21
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-11-21
 * @FilePath: /algorithm/golang/2216_min_deletion/min_deletion.go
 */

// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func minDeletion(nums []int) int {
	n, ans, check := len(nums), 0, true
	for i := 0; i+1 < n; i++ {
		if nums[i] == nums[i+1] && check {
			ans++
		} else {
			check = !check
		}
	}
	if (n-ans)%2 != 0 {
		ans++
	}
	return ans
}

func main() {
	tests := []struct {
		nums []int
		ans  int
	}{
		{[]int{1, 1, 2, 3, 5}, 1},
		{[]int{1, 1, 2, 2, 3, 3}, 2},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, minDeletion(test.nums), index)
	}
}
