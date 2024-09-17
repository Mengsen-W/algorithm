/*
 * @Date: 2021-06-28 08:36:16
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-06-28 08:56:15
 */

// Package main ...
package main

import (
	"math"
	"testing"

	"github.com/stretchr/testify/assert"
)

func numBusesToDestination(routes [][]int, source, target int) int {
	if source == target {
		return 0
	}

	n := len(routes)
	edge := make([][]bool, n)
	for i := range edge {
		edge[i] = make([]bool, n)
	}
	rec := map[int][]int{}
	for i, route := range routes {
		for _, site := range route {
			for _, j := range rec[site] {
				edge[i][j] = true
				edge[j][i] = true
			}
			rec[site] = append(rec[site], i)
		}
	}

	dis := make([]int, n)
	for i := range dis {
		dis[i] = -1
	}
	q := []int{}
	for _, site := range rec[source] {
		dis[site] = 1
		q = append(q, site)
	}
	for len(q) > 0 {
		x := q[0]
		q = q[1:]
		for y, b := range edge[x] {
			if b && dis[y] == -1 {
				dis[y] = dis[x] + 1
				q = append(q, y)
			}
		}
	}

	ans := math.MaxInt32
	for _, site := range rec[target] {
		if dis[site] != -1 && dis[site] < ans {
			ans = dis[site]
		}
	}
	if ans < math.MaxInt32 {
		return ans
	}
	return -1
}

func main() {
	tests := []struct {
		routes [][]int
		source int
		target int
		ans    int
	}{
		{[][]int{{1, 2, 7}, {3, 6, 7}}, 1, 6, 2},
		{[][]int{{7, 12}, {4, 5, 15}, {6}, {15, 19}, {9, 12, 13}}, 15, 12, -1},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, numBusesToDestination(test.routes, test.source, test.target), index)
	}
}
