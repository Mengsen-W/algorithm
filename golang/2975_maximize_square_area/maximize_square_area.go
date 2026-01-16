// Package main ...
package main

import (
	"sort"
	"testing"

	"github.com/stretchr/testify/assert"
)

func maximizeSquareArea(m int, n int, hFences []int, vFences []int) int {
	hEdges := getEdges(hFences, m)
	vEdges := getEdges(vFences, n)

	var res int64 = 0
	for e := range hEdges {
		if _, exists := vEdges[e]; exists {
			if int64(e) > res {
				res = int64(e)
			}
		}
	}

	if res == 0 {
		return -1
	}
	return int((res * res) % 1000000007)
}

func getEdges(fences []int, border int) map[int]bool {
	set := make(map[int]bool)
	list := make([]int, len(fences))

	copy(list, fences)
	list = append(list, 1, border)
	sort.Ints(list)

	for i := 0; i < len(list); i++ {
		for j := i + 1; j < len(list); j++ {
			set[list[j]-list[i]] = true
		}
	}

	return set
}

func main() {
	tests := []struct {
		m       int
		n       int
		hFences []int
		vFences []int
		expect  int
	}{
		{4, 3, []int{2, 3}, []int{2}, 4},
		{6, 7, []int{2}, []int{4}, -1},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.expect, maximizeSquareArea(test.m, test.n, test.hFences, test.vFences), index)
	}
}
