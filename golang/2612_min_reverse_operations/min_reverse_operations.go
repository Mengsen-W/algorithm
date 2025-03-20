// Package main ...
package main

import (
	"slices"
	"sort"
	"testing"

	"github.com/stretchr/testify/assert"
)

func minReverseOperations(n int, p int, banned []int, k int) []int {
	ban := make(map[int]bool)
	for _, b := range banned {
		ban[b] = true
	}
	sets := make([][]int, 2)
	for i := 0; i < n; i++ {
		if i != p && !ban[i] {
			sets[i%2] = append(sets[i%2], i)
		}
	}
	for i := range sets {
		sort.Ints(sets[i])
	}
	ans := make([]int, n)
	for i := range ans {
		ans[i] = -1
	}
	q := []int{p}
	ans[p] = 0
	for len(q) > 0 {
		i := q[0]
		q = q[1:]
		mn := max(i-k+1, k-i-1)
		mx := min(i+k-1, 2*n-k-i-1)
		targetSet := sets[mx%2]
		toRemove := []int{}
		left := sort.SearchInts(targetSet, mn)
		right := sort.SearchInts(targetSet, mx+1)
		for j := left; j < right; j++ {
			val := targetSet[j]
			ans[val] = ans[i] + 1
			q = append(q, val)
			toRemove = append(toRemove, val)
		}
		for _, val := range toRemove {
			idx := sort.SearchInts(targetSet, val)
			if idx < len(targetSet) && targetSet[idx] == val {
				targetSet = slices.Delete(targetSet, idx, idx+1)
			}
		}
		sets[mx%2] = targetSet
	}

	return ans
}

func main() {
	tests := []struct {
		n      int
		p      int
		banned []int
		k      int
		ans    []int
	}{
		{4, 0, []int{1, 2}, 4, []int{0, -1, -1, 1}},
		{5, 0, []int{2, 4}, 3, []int{0, -1, -1, -1, -1}},
		{4, 2, []int{0, 1, 3}, 1, []int{-1, -1, 0, -1}},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, minReverseOperations(test.n, test.p, test.banned, test.k), index)
	}
}
