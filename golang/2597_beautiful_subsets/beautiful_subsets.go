// Package main ...
package main

import (
	"sort"
	"testing"

	"github.com/stretchr/testify/assert"
)

func beautifulSubsets(nums []int, k int) int {
	groups := make(map[int]map[int]int)
	for _, a := range nums {
		mod := a % k
		if _, ok := groups[mod]; !ok {
			groups[mod] = make(map[int]int)
		}
		groups[mod][a]++
	}
	ans := 1
	for _, g := range groups {
		keys := make([]int, 0, len(g))
		for key := range g {
			keys = append(keys, key)
		}
		sort.Ints(keys)
		m := len(keys)
		f := make([][2]int, m)
		f[0][0] = 1
		f[0][1] = (1 << g[keys[0]]) - 1
		for i := 1; i < m; i++ {
			f[i][0] = f[i-1][0] + f[i-1][1]
			if keys[i]-keys[i-1] == k {
				f[i][1] = f[i-1][0] * ((1 << g[keys[i]]) - 1)
			} else {
				f[i][1] = (f[i-1][0] + f[i-1][1]) * ((1 << g[keys[i]]) - 1)
			}
		}
		ans *= f[m-1][0] + f[m-1][1]
	}
	return ans - 1
}

func main() {
	tests := []struct {
		nums []int
		k    int
		ans  int
	}{
		{[]int{2, 4, 6}, 2, 4},
		{[]int{1}, 1, 1},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, beautifulSubsets(test.nums, test.k), index)
	}
}
