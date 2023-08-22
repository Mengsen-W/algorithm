/*
 * @Date: 2023-08-22
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-08-22
 * @FilePath: /algorithm/golang/849_max_dist_to_closest/max_dist_to_closest.go
 */

// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func maxDistToClosest(seats []int) int {
	max := func(a, b int) int {
		if a > b {
			return a
		}
		return b
	}
	res := 0
	l := 0
	for l < len(seats) && seats[l] == 0 {
		l++
	}

	res = max(res, l)

	for l < len(seats) {
		r := l + 1
		for r < len(seats) && seats[r] == 0 {
			r++
		}

		if r == len(seats) {
			res = max(res, r-l-1)
		} else {
			res = max(res, (r-l)/2)
		}
		l = r
	}
	return res
}

func main() {
	tests := []struct {
		seats []int
		ans   int
	}{
		{[]int{1, 0, 0, 0, 1, 0, 1}, 2},
		{[]int{1, 0, 0, 0}, 3},
		{[]int{0, 1}, 1},
	}

	for _, item := range tests {
		assert.Equal(&testing.T{}, item.ans, maxDistToClosest(item.seats))
	}
}
