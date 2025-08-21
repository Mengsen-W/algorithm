// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func numSubmat(mat [][]int) int {
	n := len(mat[0])
	heights := make([]int, n)
	res := 0
	for _, row := range mat {
		for i := 0; i < n; i++ {
			if row[i] == 0 {
				heights[i] = 0
			} else {
				heights[i]++
			}
		}
		stack := [][3]int{{-1, 0, -1}}
		for i, h := range heights {
			for len(stack) > 1 && stack[len(stack)-1][2] >= h {
				stack = stack[:len(stack)-1]
			}
			top := stack[len(stack)-1]
			j, prev := top[0], top[1]
			cur := prev + (i-j)*h
			stack = append(stack, [3]int{i, cur, h})
			res += cur
		}
	}
	return res
}

func main() {
	tests := []struct {
		mat [][]int
		res int
	}{
		{[][]int{{1, 0, 1}, {1, 1, 0}, {1, 1, 0}}, 13},
		{[][]int{{0, 1, 1, 0}, {0, 1, 1, 1}, {1, 1, 1, 0}}, 24},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.res, numSubmat(test.mat), "index: %d", index)
	}
}
