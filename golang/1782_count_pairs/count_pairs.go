/*
 * @Date: 2023-08-23
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-08-23
 * @FilePath: /algorithm/golang/1782_count_pairs/count_pairs.go
 */

// Package main ...
package main

import (
	"sort"
	"testing"

	"github.com/stretchr/testify/assert"
)

func countPairs(n int, edges [][]int, queries []int) []int {
	max := func(a int, b int) int {
		if a > b {
			return a
		}
		return b
	}
	degree := make([]int, n)
	cnt := map[int]int{}
	for _, edge := range edges {
		x, y := edge[0]-1, edge[1]-1
		if x > y {
			x, y = y, x
		}
		degree[x]++
		degree[y]++
		cnt[x*n+y]++
	}

	arr := make([]int, n)
	copy(arr, degree)
	sort.Ints(arr)
	ans := []int{}
	for _, bound := range queries {
		total := 0
		for i, j := 0, n-1; i < n; i++ {
			for j > i && arr[i]+arr[j] > bound {
				j--
			}
			total += n - 1 - max(i, j)
		}
		for val, freq := range cnt {
			x, y := val/n, val%n
			if degree[x]+degree[y] > bound && degree[x]+degree[y]-freq <= bound {
				total--
			}
		}
		ans = append(ans, total)
	}
	return ans
}

func main() {
	tests := []struct {
		n       int
		edges   [][]int
		queries []int
		ans     []int
	}{
		{4, [][]int{{1, 2}, {2, 4}, {1, 3}, {2, 3}, {2, 1}}, []int{2, 3}, []int{6, 5}},
		{5, [][]int{{1, 5}, {1, 5}, {3, 4}, {2, 5}, {1, 3}, {5, 1}, {2, 3}, {2, 5}}, []int{1, 2, 3, 4, 5}, []int{10, 10, 9, 8, 6}},
	}

	for _, item := range tests {
		assert.Equal(&testing.T{}, item.ans, countPairs(item.n, item.edges, item.queries))
	}
}
