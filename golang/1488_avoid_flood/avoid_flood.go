// Package main ...
package main

import (
	"sort"
	"testing"

	"github.com/stretchr/testify/assert"
)

func avoidFlood(rains []int) []int {
	n := len(rains)
	ans := make([]int, n)
	st := []int{}
	mp := make(map[int]int)
	for i := 0; i < n; i++ {
		ans[i] = 1
	}
	for i, rain := range rains {
		if rain == 0 {
			st = append(st, i)
		} else {
			ans[i] = -1
			if day, ok := mp[rain]; ok {
				it := sort.SearchInts(st, day)
				if it == len(st) {
					return []int{}
				}
				ans[st[it]] = rain
				copy(st[it:len(st)-1], st[it+1:])
				st = st[:len(st)-1]
			}
			mp[rain] = i
		}
	}
	return ans
}

func main() {
	tests := []struct {
		rains []int
		ans   []int
	}{
		{[]int{1, 2, 3, 4}, []int{-1, -1, -1, -1}},
		{[]int{1, 2, 0, 0, 2, 1}, []int{-1, -1, 2, 1, -1, -1}},
		{[]int{1, 2, 0, 1, 2}, []int{}},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, avoidFlood(test.rains), "case:%d", index)
	}
}
