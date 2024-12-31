// Package main ...
package main

import (
	"sort"
	"testing"

	"github.com/stretchr/testify/assert"
)

func minimumCost(m int, n int, horizontalCut []int, verticalCut []int) int64 {
	sort.Ints(horizontalCut)
	sort.Ints(verticalCut)
	h, v := 1, 1
	var res int64
	for len(horizontalCut) > 0 || len(verticalCut) > 0 {
		if len(verticalCut) == 0 || len(horizontalCut) > 0 && horizontalCut[len(horizontalCut)-1] > verticalCut[len(verticalCut)-1] {
			res += int64(horizontalCut[len(horizontalCut)-1] * h)
			horizontalCut = horizontalCut[:len(horizontalCut)-1]
			v++
		} else {
			res += int64(verticalCut[len(verticalCut)-1] * v)
			verticalCut = verticalCut[:len(verticalCut)-1]
			h++
		}
	}
	return res
}

func main() {
	tests := []struct {
		m             int
		n             int
		horizontalCut []int
		verticalCut   []int
		ans           int64
	}{
		{3, 2, []int{1, 3}, []int{5}, 13},
		{2, 2, []int{7}, []int{4}, 15},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, minimumCost(test.m, test.n, test.horizontalCut, test.verticalCut), index)
	}
}
