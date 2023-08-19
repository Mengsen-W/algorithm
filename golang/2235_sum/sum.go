/*
 * @Date: 2023-08-19
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-08-19
 * @FilePath: /algorithm/golang/2235_sum/sum.go
 */

// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func sum(num1 int, num2 int) int {
	return num1 + num2
}

func main() {
	tests := []struct {
		num1 int
		num2 int
		ans  int
	}{
		{12, 5, 17},
		{-10, 4, -6},
	}

	for _, item := range tests {
		assert.Equal(&testing.T{}, item.ans, sum(item.num1, item.num2))
	}
}
