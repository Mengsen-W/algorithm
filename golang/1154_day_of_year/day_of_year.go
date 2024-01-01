/*
 * @Date: 2021-12-21 01:20:51
 * @Author: Mengsen Wang
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-12-31
 */

// Package main ...
package main

import (
	"strconv"
	"testing"

	"github.com/stretchr/testify/assert"
)

func dayOfYear(date string) int {
	year, _ := strconv.Atoi(date[:4])
	month, _ := strconv.Atoi(date[5:7])
	day, _ := strconv.Atoi(date[8:])

	days := []int{31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31}
	if year%400 == 0 || (year%4 == 0 && year%100 != 0) {
		days[1]++
	}

	ans := day
	for _, d := range days[:month-1] {
		ans += d
	}
	return ans
}

func main() {
	tests := []struct {
		date string
		ans  int
	}{
		{"2019-01-09", 9},
		{"2019-02-10", 41},
		{"2003-03-01", 60},
		{"2004-03-01", 61},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, dayOfYear(test.date), index)
	}
}
