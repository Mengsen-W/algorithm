/*
 * @Date: 2023-06-29
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-06-29
 * @FilePath: /algorithm/golang/1253_reconstruct_matrix/reconstruct_matrix.go
 */

// Package main ...
package main

import (
	"math"
	"testing"

	"github.com/stretchr/testify/assert"
)

func reconstructMatrix(upper int, lower int, colsum []int) [][]int {
	n := len(colsum)
	sumVal := 0
	twoNum := 0
	for i := 0; i < n; i++ {
		if colsum[i] == 2 {
			twoNum++
		}
		sumVal += colsum[i]
	}
	if sumVal != upper+lower || math.Min(float64(upper), float64(lower)) < float64(twoNum) {
		return [][]int{}
	}
	upper -= twoNum
	lower -= twoNum
	res := make([][]int, 2)
	for i := 0; i < 2; i++ {
		res[i] = make([]int, n)
	}
	for i := 0; i < n; i++ {
		if colsum[i] == 2 {
			res[0][i], res[1][i] = 1, 1
		} else if colsum[i] == 1 {
			if upper > 0 {
				res[0][i] = 1
				upper--
			} else {
				res[1][i] = 1
			}
		}
	}
	return res
}

func main() {
	testMap := []struct {
		upper  int
		lower  int
		colsum []int
		ans    [][]int
	}{
		{2, 1, []int{1, 1, 1}, [][]int{{1, 0, 1}, {0, 1, 0}}},
		{2, 3, []int{2, 2, 1, 1}, [][]int{}},
		{5, 5, []int{2, 1, 2, 0, 1, 0, 1, 2, 0, 1}, [][]int{{1, 1, 1, 0, 1, 0, 0, 1, 0, 0}, {1, 0, 1, 0, 0, 0, 1, 1, 0, 1}}},
	}

	for _, item := range testMap {
		assert.Equal(&testing.T{}, reconstructMatrix(item.upper, item.lower, item.colsum), item.ans)
	}
}
