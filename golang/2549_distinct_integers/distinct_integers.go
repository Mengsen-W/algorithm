/*
 * @Date: 2024-03-23
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-03-23
 * @FilePath: /algorithm/golang/2549_distinct_integers/distinct_integers.go
 */

// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func distinctIntegers(n int) int {
	return max(n-1, 1)
}

func main() {
	tests := []struct {
		n   int
		ans int
	}{
		{5, 4},
		{3, 2},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, distinctIntegers(test.n), index)
	}
}
