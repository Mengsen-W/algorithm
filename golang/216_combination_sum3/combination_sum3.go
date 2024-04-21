/*
 * @Date: 2024-04-21
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-04-21
 * @FilePath: /algorithm/golang/216_combination_sum3/combination_sum3.go
 */

// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func combinationSum3(k int, n int) (ans [][]int) {
	var temp []int
	var dfs func(cur, rest int)
	dfs = func(cur, rest int) {
		// 找到一个答案
		if len(temp) == k && rest == 0 {
			ans = append(ans, append([]int(nil), temp...))
			return
		}
		// 剪枝：跳过的数字过多，后面已经无法选到 k 个数字
		if len(temp)+10-cur < k || rest < 0 {
			return
		}
		// 跳过当前数字
		dfs(cur+1, rest)
		// 选当前数字
		temp = append(temp, cur)
		dfs(cur+1, rest-cur)
		temp = temp[:len(temp)-1]
	}
	dfs(1, n)
	return
}

func main() {
	tests := []struct {
		k, n int
		ans  [][]int
	}{
		{3, 7, [][]int{{1, 2, 4}}},
		{3, 9, [][]int{{1, 2, 6}, {1, 3, 5}, {2, 3, 4}}},
		{4, 1, [][]int{}},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, combinationSum3(test.k, test.n), index)
	}
}
