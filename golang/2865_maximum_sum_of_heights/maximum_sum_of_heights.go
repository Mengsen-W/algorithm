/*
 * @Date: 2024-01-24
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-01-24
 * @FilePath: /algorithm/golang/2865_maximum_sum_of_heights/maximum_sum_of_heights.go
 */

// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func maximumSumOfHeights(maxHeights []int) int64 {
	n := len(maxHeights)
	prefix := make([]int, n)
	suffix := make([]int, n)
	stack1, stack2 := []int{}, []int{}

	for i := 0; i < n; i++ {
		for len(stack1) > 0 && maxHeights[i] < maxHeights[stack1[len(stack1)-1]] {
			stack1 = stack1[:len(stack1)-1]
		}
		if len(stack1) == 0 {
			prefix[i] = (i + 1) * maxHeights[i]
		} else {
			last := stack1[len(stack1)-1]
			prefix[i] = prefix[last] + (i-last)*maxHeights[i]
		}
		stack1 = append(stack1, i)
	}

	res := 0
	for i := n - 1; i >= 0; i-- {
		for len(stack2) > 0 && maxHeights[i] < maxHeights[stack2[len(stack2)-1]] {
			stack2 = stack2[:len(stack2)-1]
		}
		if len(stack2) == 0 {
			suffix[i] = (n - i) * maxHeights[i]
		} else {
			last := stack2[len(stack2)-1]
			suffix[i] = suffix[last] + (last-i)*maxHeights[i]
		}
		stack2 = append(stack2, i)
		res = max(res, prefix[i]+suffix[i]-maxHeights[i])
	}
	return int64(res)
}

func main() {
	tests := []struct {
		maxHeights []int
		ans        int64
	}{
		{[]int{5, 3, 4, 1, 1}, 13},
		{[]int{6, 5, 3, 9, 2, 7}, 22},
		{[]int{3, 2, 5, 5, 2, 3}, 18},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, maximumSumOfHeights(test.maxHeights), index)
	}
}
