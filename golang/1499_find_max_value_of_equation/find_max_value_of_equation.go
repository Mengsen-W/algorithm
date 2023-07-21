/*
 * @Date: 2023-07-21
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-07-21
 * @FilePath: /algorithm/golang/1499_find_max_value_of_equation/find_max_value_of_equation.go
 */

// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func findMaxValueOfEquation(points [][]int, k int) int {
	max := func(a, b int) int {
		if a > b {
			return a
		}
		return b
	}
	res := -0x3f3f3f3f
	q := [][]int{}
	for _, point := range points {
		x, y := point[0], point[1]
		for len(q) > 0 && x-q[0][1] > k {
			q = q[1:]
		}
		if len(q) > 0 {
			res = max(res, x+y+q[0][0])
		}
		for len(q) > 0 && y-x >= q[len(q)-1][0] {
			q = q[:len(q)-1]
		}
		q = append(q, []int{y - x, x})
	}
	return res
}

func main() {
	tests := []struct {
		points [][]int
		k      int
		ans    int
	}{
		{[][]int{{1, 3}, {2, 0}, {5, 10}, {6, -10}}, 1, 4},
		{[][]int{{0, 0}, {3, 0}, {9, 2}}, 3, 3},
	}

	for _, i := range tests {
		assert.Equal(&testing.T{}, findMaxValueOfEquation(i.points, i.k), i.ans)
	}
}
