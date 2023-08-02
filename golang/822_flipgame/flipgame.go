/*
 * @Date: 2023-08-02
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-08-02
 * @FilePath: /algorithm/golang/822_flipgame/flipgame.go
 */

// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func flipgame(fronts, backs []int) int {
	same := make(map[int]bool)
	for i := range fronts {
		if fronts[i] == backs[i] {
			same[fronts[i]] = true
		}
	}
	res := 3000
	for _, x := range fronts {
		if x < res && !same[x] {
			res = x
		}
	}
	for _, x := range backs {
		if x < res && !same[x] {
			res = x
		}
	}
	return res % 3000
}

func main() {
	tests := []struct {
		fronts []int
		backs  []int
		ans    int
	}{
		{[]int{1, 2, 4, 4, 7}, []int{1, 3, 4, 1, 3}, 2},
	}

	for _, item := range tests {
		assert.Equal(&testing.T{}, item.ans, flipgame(item.fronts, item.backs))
	}
}
