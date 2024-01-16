/*
 * @Date: 2024-01-16
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-01-16
 * @FilePath: /algorithm/golang/2719_count/count.go
 */

// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

const (
	MOD = 1000000007
	N   = 23
	M   = 401
)

func count(num1 string, num2 string, minSum int, maxSum int) int {
	d := make([][]int, N)
	for i := range d {
		d[i] = make([]int, M)
		for j := range d[i] {
			d[i][j] = -1
		}
	}

	var dfs func(num string, i int, j int, limit bool) int
	dfs = func(num string, i int, j int, limit bool) int {
		if j > maxSum {
			return 0
		}
		if i == -1 {
			if j >= minSum {
				return 1
			}
			return 0
		}
		if !limit && d[i][j] != -1 {
			return d[i][j]
		}

		res := 0
		var up int
		if limit {
			up = int(num[i] - '0')
		} else {
			up = 9
		}

		for x := 0; x <= up; x++ {
			res = (res + dfs(num, i-1, j+x, limit && x == up)) % MOD
		}

		if !limit {
			d[i][j] = res
		}
		return res
	}

	get := func(num string) int {
		num = reverse(num)
		return dfs(num, len(num)-1, 0, true)
	}
	// 求解 num - 1，先把最后一个非 0 字符减去 1，再把后面的 0 字符变为 9
	sub := func(num string) string {
		i := len(num) - 1
		arr := []byte(num)
		for arr[i] == '0' {
			i--
		}
		arr[i]--
		i++
		for ; i < len(num); i++ {
			arr[i] = '9'
		}
		return string(arr)
	}

	return (get(num2) - get(sub(num1)) + MOD) % MOD
}

func reverse(s string) string {
	runes := []rune(s)
	for i, j := 0, len(runes)-1; i < j; i, j = i+1, j-1 {
		runes[i], runes[j] = runes[j], runes[i]
	}
	return string(runes)
}

func main() {
	tests := []struct {
		num1   string
		num2   string
		minSum int
		maxSum int
		ans    int
	}{
		{"1", "12", 1, 8, 11},
		{"1", "5", 1, 5, 5},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, count(test.num1, test.num2, test.minSum, test.maxSum), index)
	}
}
