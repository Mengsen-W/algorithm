// Package main ...
package main

import (
	"sort"
	"testing"

	"github.com/stretchr/testify/assert"
)

var f = make(map[int]int)

func getF(x int) int {
	if val, exists := f[x]; exists {
		return val
	}
	if x == 1 {
		f[x] = 0
		return 0
	}
	if x&1 == 1 {
		f[x] = getF(x*3+1) + 1
	} else {
		f[x] = getF(x/2) + 1
	}
	return f[x]
}

func getKth(lo int, hi int, k int) int {
	v := make([]int, 0)
	for i := lo; i <= hi; i++ {
		v = append(v, i)
	}
	sort.Slice(v, func(i, j int) bool {
		if getF(v[i]) != getF(v[j]) {
			return getF(v[i]) < getF(v[j])
		}
		return v[i] < v[j]
	})
	return v[k-1]
}

func main() {
	tests := []struct {
		lo  int
		hi  int
		k   int
		ans int
	}{
		{12, 15, 2, 13},
		{7, 11, 4, 7},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, getKth(test.lo, test.hi, test.k), index)
	}
}
