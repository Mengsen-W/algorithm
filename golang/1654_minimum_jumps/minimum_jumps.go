/*
 * @Date: 2023-08-30
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-08-30
 * @FilePath: /algorithm/golang/1654_minimum_jumps/minimum_jumps.go
 */

// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func minimumJumps(forbidden []int, a int, b int, x int) int {
	max := func(x int, y int) int {
		if x < y {
			return y
		}
		return x
	}
	lower := 0
	maxVal := 0
	for _, val := range forbidden {
		maxVal = max(maxVal, val)
	}
	upper := max(maxVal+a, x) + b
	q := [][3]int{{0, 1, 0}}
	visited := make(map[int]bool)
	forbiddenSet := make(map[int]bool)
	visited[0] = true

	for _, position := range forbidden {
		forbiddenSet[position] = true
	}
	for len(q) > 0 {
		position, direction, step := q[0][0], q[0][1], q[0][2]
		q = q[1:]
		if position == x {
			return step
		}
		nextPosition, nextDirection := position+a, 1
		_, ok1 := visited[nextPosition*nextDirection]
		_, ok2 := forbiddenSet[nextPosition]
		if lower <= nextPosition && nextPosition <= upper && !ok1 && !ok2 {
			visited[nextPosition*nextDirection] = true
			q = append(q, [3]int{nextPosition, nextDirection, step + 1})
		}
		if direction == 1 {
			nextPosition, nextDirection := position-b, -1
			_, ok1 := visited[nextPosition*nextDirection]
			_, ok2 := forbiddenSet[nextPosition]
			if lower <= nextPosition && nextPosition <= upper && !ok1 && !ok2 {
				visited[nextPosition*nextDirection] = true
				q = append(q, [3]int{nextPosition, nextDirection, step + 1})
			}
		}
	}
	return -1
}

func main() {
	tests := []struct {
		forbidden []int
		a         int
		b         int
		x         int
		ans       int
	}{
		{[]int{14, 4, 18, 1, 15}, 3, 15, 9, 3},
		{[]int{8, 3, 16, 6, 12, 20}, 15, 13, 11, -1},
		{[]int{1, 6, 2, 14, 5, 17, 4}, 16, 9, 7, 2},
	}

	for _, item := range tests {
		assert.Equal(&testing.T{}, item.ans, minimumJumps(item.forbidden, item.a, item.b, item.x))
	}
}
