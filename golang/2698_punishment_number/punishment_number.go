/*
 * @Date: 2023-10-26
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-10-26
 * @FilePath: /algorithm/golang/2698_punishment_number/punishment_number.go
 */
// Package main ...
package main

import (
	"strconv"
	"testing"

	"github.com/stretchr/testify/assert"
)

func punishmentNumber(n int) int {
	var dfs func(string, int, int, int) bool
	dfs = func(s string, pos int, tot int, target int) bool {
		if pos == len(s) {
			return tot == target
		}
		sum := 0
		for i := pos; i < len(s); i++ {
			sum = sum*10 + int(s[i]-'0')
			if sum+tot > target {
				break
			}
			if dfs(s, i+1, sum+tot, target) {
				return true
			}
		}
		return false
	}
	res := 0
	for i := 1; i <= n; i++ {
		if dfs(strconv.Itoa(i*i), 0, 0, i) {
			res += i * i
		}
	}
	return res
}

func main() {
	tests := []struct {
		n   int
		ans int
	}{
		{10, 182},
		{37, 1478},
	}
	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, punishmentNumber(test.n), index)
	}
}
