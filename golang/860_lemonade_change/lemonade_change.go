/*
 * @Date: 2023-07-22
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-07-22
 * @FilePath: /algorithm/golang/860_lemonade_change/lemonade_change.go
 */

// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func lemonadeChange(bills []int) bool {
	five, ten := 0, 0
	for _, bill := range bills {
		if bill == 5 {
			five++
		} else if bill == 10 {
			if five == 0 {
				return false
			}
			five--
			ten++
		} else {
			if five > 0 && ten > 0 {
				five--
				ten--
			} else if five >= 3 {
				five -= 3
			} else {
				return false
			}
		}
	}
	return true
}

func main() {
	tests := []struct {
		bills []int
		ans   bool
	}{
		{[]int{5, 5, 5, 10, 20}, true},
		{[]int{5, 5, 10, 10, 20}, false},
	}

	for _, test := range tests {
		assert.Equal(&testing.T{}, test.ans, lemonadeChange(test.bills))
	}
}
