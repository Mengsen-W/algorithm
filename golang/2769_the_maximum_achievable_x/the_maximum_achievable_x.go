/*
 * @Date: 2024-05-21
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-05-21
 * @FilePath: /algorithm/golang/2769_the_maximum_achievable_x/the_maximum_achievable_x.go
 */

// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func theMaximumAchievableX(num int, t int) int {
	return num + 2*t
}

func main() {
	tests := []struct {
		num int
		t   int
		ans int
	}{
		{4, 1, 6},
		{3, 2, 7},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, theMaximumAchievableX(test.num, test.t), index)
	}
}
