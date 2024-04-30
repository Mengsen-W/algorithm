/*
 * @Date: 2024-04-30
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-04-30
 * @FilePath: /algorithm/golang/2798_number_of_employees_who_met_target/number_of_employees_who_met_target.go
 */

// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func numberOfEmployeesWhoMetTarget(hours []int, target int) int {
	ans := 0
	for i := 0; i < len(hours); i++ {
		if hours[i] >= target {
			ans++
		}
	}
	return ans
}

func main() {
	tests := []struct {
		hours  []int
		target int
		ans    int
	}{
		{[]int{0, 1, 2, 3, 4}, 2, 3},
		{[]int{5, 1, 4, 2, 2}, 6, 0},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, numberOfEmployeesWhoMetTarget(test.hours, test.target), index)
	}
}
