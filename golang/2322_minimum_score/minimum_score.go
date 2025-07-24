// Package main ...
package main

import (
	"math"
	"testing"

	"github.com/stretchr/testify/assert"
)

func minimumScore(nums []int, edges [][]int) int {
	n := len(nums)
	e := make([][]int, n)
	for _, v := range edges {
		e[v[0]] = append(e[v[0]], v[1])
		e[v[1]] = append(e[v[1]], v[0])
	}

	sum := 0
	for _, x := range nums {
		sum ^= x
	}

	res := math.MaxInt32
	var dfs2 func(int, int, int, int) int
	dfs2 = func(x, f, oth, anc int) int {
		son := nums[x]
		for _, y := range e[x] {
			if y == f {
				continue
			}
			son ^= dfs2(y, x, oth, anc)
		}
		if f == anc {
			return son
		}

		res = min(res, calc(oth, son, sum^oth^son))
		return son
	}

	var dfs func(int, int) int
	dfs = func(x, f int) int {
		son := nums[x]
		for _, y := range e[x] {
			if y == f {
				continue
			}
			son ^= dfs(y, x)
		}

		for _, y := range e[x] {
			if y == f {
				dfs2(y, x, son, x)
			}
		}
		return son
	}

	dfs(0, -1)
	return res
}

func calc(part1, part2, part3 int) int {
	return max(part1, max(part2, part3)) - min(part1, min(part2, part3))
}

func main() {
	tests := []struct {
		nums  []int
		edges [][]int
		ans   int
	}{
		{[]int{1, 5, 5, 4, 11}, [][]int{{0, 1}, {1, 2}, {1, 3}, {3, 4}}, 9},
		{[]int{5, 5, 2, 4, 4, 2}, [][]int{{0, 1}, {1, 2}, {5, 2}, {4, 3}, {1, 3}}, 0},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, minimumScore(test.nums, test.edges), "case %d", index)
	}
}
