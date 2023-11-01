/*
 * @Date: 2023-11-01
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-11-01
 * @FilePath: /algorithm/golang/2127_maximum_invitations/maximum_invitations.go
 */

// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func maximumInvitations(favorite []int) int {
	max := func(x, y int) int {
		if x < y {
			return y
		}
		return x
	}

	n := len(favorite)
	indeg := make([]int, n)
	for _, x := range favorite {
		indeg[x]++
	}

	used := make([]bool, n)
	f := make([]int, n)
	for i := 0; i < n; i++ {
		f[i] = 1
	}

	q := []int{}
	for i := 0; i < n; i++ {
		if indeg[i] == 0 {
			q = append(q, i)
		}
	}
	for len(q) > 0 {
		u := q[0]
		used[u] = true
		q = q[1:]
		v := favorite[u]
		// 状态转移
		f[v] = max(f[v], f[u]+1)
		indeg[v]--
		if indeg[v] == 0 {
			q = append(q, v)
		}
	}
	ring, total := 0, 0
	for i := 0; i < n; i++ {
		if !used[i] {
			j := favorite[i]
			if favorite[j] == i {
				total += f[i] + f[j]
				used[i], used[j] = true, true
			} else {
				u, cnt := i, 0
				for {
					cnt++
					u = favorite[u]
					used[u] = true
					if u == i {
						break
					}
				}
				ring = max(ring, cnt)
			}
		}
	}
	return max(ring, total)
}

func main() {
	tests := []struct {
		favorite []int
		ans      int
	}{
		{[]int{2, 2, 1, 2}, 3},
		{[]int{1, 2, 0}, 3},
		{[]int{3, 0, 1, 4, 1}, 4},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, maximumInvitations(test.favorite), index)
	}
}
