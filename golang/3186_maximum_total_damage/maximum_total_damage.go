// Package main ...
package main

import (
	"sort"
	"testing"

	"github.com/stretchr/testify/assert"
)

func maximumTotalDamage(power []int) int64 {
	count := map[int]int{}
	for _, p := range power {
		count[p]++
	}
	keys := make([]int, 0, len(count))
	for k := range count {
		keys = append(keys, k)
	}
	sort.Ints(keys)
	vec := [][2]int{{-1000000000, 0}}
	for _, k := range keys {
		vec = append(vec, [2]int{k, count[k]})
	}
	n := len(vec)
	f := make([]int64, n)
	var mx int64
	var ans int64
	j := 1
	for i := 1; i < n; i++ {
		for j < i && vec[j][0] < vec[i][0]-2 {
			if f[j] > mx {
				mx = f[j]
			}
			j++
		}
		f[i] = mx + int64(vec[i][0])*int64(vec[i][1])
		if f[i] > ans {
			ans = f[i]
		}
	}
	return ans
}

func main() {
	tests := []struct {
		power []int
		ans   int64
	}{
		{[]int{1, 1, 3, 4}, 6},
		{[]int{7, 1, 6, 6}, 13},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, maximumTotalDamage(test.power), index)
	}
}
