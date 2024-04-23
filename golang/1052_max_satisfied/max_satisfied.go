/*
 * @Date: 2024-04-23
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-04-23
 * @FilePath: /algorithm/golang/1052_max_satisfied/max_satisfied.go
 */

// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func maxSatisfied(customers []int, grumpy []int, minutes int) int {
	max := func(a, b int) int {
		if a > b {
			return a
		}
		return b
	}
	total := 0
	n := len(customers)
	for i := 0; i < n; i++ {
		if grumpy[i] == 0 {
			total += customers[i]
		}
	}
	increase := 0
	for i := 0; i < minutes; i++ {
		increase += customers[i] * grumpy[i]
	}
	maxIncrease := increase
	for i := minutes; i < n; i++ {
		increase = increase - customers[i-minutes]*grumpy[i-minutes] + customers[i]*grumpy[i]
		maxIncrease = max(maxIncrease, increase)
	}
	return total + maxIncrease
}

func main() {
	tests := []struct {
		customers []int
		grumpy    []int
		minutes   int
		ans       int
	}{
		{[]int{1, 0, 1, 2, 1, 1, 7, 5}, []int{0, 1, 0, 1, 0, 1, 0, 1}, 3, 16},
		{[]int{1}, []int{0}, 1, 1},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, maxSatisfied(test.customers, test.grumpy, test.minutes), index)
	}
}
