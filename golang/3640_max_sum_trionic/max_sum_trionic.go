// Package main ...
package main

import (
	"math"
	"testing"

	"github.com/stretchr/testify/assert"
)

func maxSumTrionic(nums []int) int64 {
	n := len(nums)
	var p, q, j int
	var max_sum, sum, res int64
	ans := int64(math.MinInt64)
	for i := 0; i < n; i++ {
		j = i + 1
		res = 0
		// 第一段
		for ; j < n && nums[j-1] < nums[j]; j++ {
		}
		p = j - 1
		if p == i {
			continue
		}
		// 第二段
		res += int64(nums[p] + nums[p-1])
		for ; j < n && nums[j-1] > nums[j]; j++ {
			res += int64(nums[j])
		}
		q = j - 1
		if q == p || q == n-1 || (j < n && nums[j] <= nums[q]) {
			i = q
			continue
		}
		// 第三段
		res += int64(nums[q+1])
		// 第三段求累加最大值
		max_sum = 0
		sum = 0
		for k := q + 2; k < n && nums[k] > nums[k-1]; k++ {
			sum += int64(nums[k])
			if sum > max_sum {
				max_sum = sum
			}
		}
		res += max_sum
		// 第一段求累加最大值
		max_sum = 0
		sum = 0
		for k := p - 2; k >= i; k-- {
			sum += int64(nums[k])
			if sum > max_sum {
				max_sum = sum
			}
		}
		res += max_sum
		// 更新答案
		if res > ans {
			ans = res
		}
		i = q - 1
	}
	return ans
}

func main() {
	tests := []struct {
		nums []int
		ans  int64
	}{
		{[]int{0, -2, -1, -3, 0, 2, -1}, -4},
		{[]int{1, 4, 2, 7}, 14},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, maxSumTrionic(test.nums), index)
	}
}
