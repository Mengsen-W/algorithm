// Package main ...
package main

import (
	"sort"
	"testing"

	"github.com/stretchr/testify/assert"
)

func findAllPeople(n int, meetings [][]int, firstPerson int) []int {
	m := len(meetings)
	sort.Slice(meetings, func(i, j int) bool {
		return meetings[i][2] < meetings[j][2]
	})
	secret := make([]bool, n)
	secret[0] = true
	secret[firstPerson] = true

	for i := 0; i < m; {
		// meetings[i .. j] 为同一时间
		j := i
		for j+1 < m && meetings[j+1][2] == meetings[i][2] {
			j++
		}

		vertices := make(map[int]bool)
		edges := make(map[int][]int)
		for k := i; k <= j; k++ {
			x, y := meetings[k][0], meetings[k][1]
			vertices[x] = true
			vertices[y] = true
			edges[x] = append(edges[x], y)
			edges[y] = append(edges[y], x)
		}

		q := []int{}
		for u := range vertices {
			if secret[u] {
				q = append(q, u)
			}
		}
		for len(q) > 0 {
			u := q[0]
			q = q[1:]
			for _, v := range edges[u] {
				if !secret[v] {
					secret[v] = true
					q = append(q, v)
				}
			}
		}

		i = j + 1
	}

	ans := []int{}
	for i := 0; i < n; i++ {
		if secret[i] {
			ans = append(ans, i)
		}
	}
	return ans
}

func main() {
	tests := []struct {
		n           int
		meetings    [][]int
		firstPerson int
		expect      []int
	}{
		{6, [][]int{{1, 2, 5}, {2, 3, 8}, {1, 5, 10}}, 1, []int{0, 1, 2, 3, 5}},
		{4, [][]int{{3, 1, 3}, {1, 2, 2}, {0, 3, 3}}, 3, []int{0, 1, 3}},
		{5, [][]int{{3, 4, 2}, {1, 2, 1}, {2, 3, 1}}, 1, []int{0, 1, 2, 3, 4}},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.expect, findAllPeople(test.n, test.meetings, test.firstPerson), index)
	}
}
