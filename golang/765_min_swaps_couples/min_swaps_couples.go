/*
 * @Date: 2023-11-11
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-11-11
 * @FilePath: /algorithm/golang/765_min_swaps_couples/min_swaps_couples.go
 */

// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func minSwapsCouples(row []int) (ans int) {
	n := len(row)
	graph := make([][]int, n/2)
	for i := 0; i < n; i += 2 {
		l, r := row[i]/2, row[i+1]/2
		if l != r {
			graph[l] = append(graph[l], r)
			graph[r] = append(graph[r], l)
		}
	}
	vis := make([]bool, n/2)
	for i, vs := range vis {
		if !vs {
			vis[i] = true
			cnt := 0
			q := []int{i}
			for len(q) > 0 {
				cnt++
				v := q[0]
				q = q[1:]
				for _, w := range graph[v] {
					if !vis[w] {
						vis[w] = true
						q = append(q, w)
					}
				}
			}
			ans += cnt - 1
		}
	}
	return
}

func main() {
	tests := []struct {
		row []int
		ans int
	}{
		{[]int{0, 2, 1, 3}, 1},
		{[]int{3, 2, 0, 1}, 0},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, minSwapsCouples(test.row), index)
	}
}
