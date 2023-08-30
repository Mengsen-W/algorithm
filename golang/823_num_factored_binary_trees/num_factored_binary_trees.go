/*
 * @Date: 2023-08-29
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-08-29
 * @FilePath: /algorithm/golang/823_num_factored_binary_trees/num_factored_binary_trees.go
 */

// Package main ...
package main

import (
	"sort"
	"testing"

	"github.com/stretchr/testify/assert"
)

func numFactoredBinaryTrees(arr []int) int {
	sort.Ints(arr)
	dp := make([]int64, len(arr))
	res, mod := int64(0), int64(1e9+7)
	for i := 0; i < len(arr); i++ {
		dp[i] = 1
		for left, right := 0, i-1; left <= right; left++ {
			for left <= right && int64(arr[left])*int64(arr[right]) > int64(arr[i]) {
				right--
			}
			if left <= right && int64(arr[left])*int64(arr[right]) == int64(arr[i]) {
				if left == right {
					dp[i] = (dp[i] + dp[left]*dp[right]) % mod
				} else {
					dp[i] = (dp[i] + dp[left]*dp[right]*2) % mod
				}
			}
		}
		res = (res + dp[i]) % mod
	}
	return int(res)
}

func main() {
	tests := []struct {
		arr []int
		ans int
	}{
		{[]int{2, 4}, 3},
		{[]int{2, 4, 5, 10}, 7},
	}

	for _, item := range tests {
		assert.Equal(&testing.T{}, item.ans, numFactoredBinaryTrees(item.arr))
	}
}
