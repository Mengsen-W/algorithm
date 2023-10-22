/*
 * @Date: 2023-10-22
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-10-22
 * @FilePath: /algorithm/golang/1402_max_satisfaction/max_satisfaction.go
 */

// Package main ...
package main

import (
	"sort"
	"testing"

	"github.com/stretchr/testify/assert"
)

func maxSatisfaction(satisfaction []int) int {
	sort.Slice(satisfaction, func(i, j int) bool {
		return satisfaction[i] > satisfaction[j]
	})
	presum, ans := 0, 0
	for _, si := range satisfaction {
		if presum+si > 0 {
			presum += si
			ans += presum
		} else {
			break
		}
	}
	return ans
}

func main() {
	tests := []struct {
		satisfaction []int
		ans          int
	}{
		{[]int{-1, -8, 0, 5, -9}, 14},
		{[]int{4, 3, 2}, 20},
		{[]int{-1, -4, -5}, 0},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, maxSatisfaction(test.satisfaction), index)
	}
}
