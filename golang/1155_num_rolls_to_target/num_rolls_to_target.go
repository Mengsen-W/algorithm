/*
 * @Date: 2023-10-24
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-10-24
 * @FilePath: /algorithm/golang/1155_num_rolls_to_target/num_rolls_to_target.go
 */

// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func numRollsToTarget(n int, k int, target int) int {
	mod := int(1e9 + 7)
	f := make([]int, target+1)
	f[0] = 1
	for i := 1; i <= n; i++ {
		for j := target; j >= 0; j-- {
			f[j] = 0
			for x := 1; x <= k; x++ {
				if j-x >= 0 {
					f[j] = (f[j] + f[j-x]) % mod
				}
			}
		}
	}
	return f[target]
}

func main() {
	tests := []struct {
		n      int
		k      int
		target int
		ans    int
	}{
		{1, 6, 3, 1},
		{2, 6, 7, 6},
		{30, 30, 500, 222616187},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, numRollsToTarget(test.n, test.k, test.target), index)
	}
}
