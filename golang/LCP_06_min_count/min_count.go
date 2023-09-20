/*
 * @Date: 2023-09-20
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-09-20
 * @FilePath: /algorithm/golang/LCP_06_min_count/min_count.go
 */

// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func minCount(coins []int) int {
	sum := 0
	for _, i := range coins {
		sum += (i + 1) / 2
	}
	return sum
}

func main() {
	tests := []struct {
		coins []int
		ans   int
	}{
		{[]int{4, 2, 1}, 4},
		{[]int{2, 3, 10}, 8},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, minCount(test.coins), index)
	}
}
