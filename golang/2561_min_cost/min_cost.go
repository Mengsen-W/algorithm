// Package main ...
package main

import (
	"math"
	"sort"
	"testing"

	"github.com/stretchr/testify/assert"
)

func minCost(basket1 []int, basket2 []int) int64 {
	freq := map[int]int{}
	m := math.MaxInt
	for _, b := range basket1 {
		freq[b]++
		if b < m {
			m = b
		}
	}
	for _, b := range basket2 {
		freq[b]--
		if b < m {
			m = b
		}
	}

	var merge []int
	for k, c := range freq {
		if c%2 != 0 {
			return -1
		}
		for i := 0; i < abs(c)/2; i++ {
			merge = append(merge, k)
		}
	}

	sort.Ints(merge)
	var res int64
	for i := 0; i < len(merge)/2; i++ {
		if 2*m < merge[i] {
			res += int64(2 * m)
		} else {
			res += int64(merge[i])
		}
	}
	return res
}

func abs(a int) int {
	if a < 0 {
		return -a
	}
	return a
}

func main() {
	tests := []struct {
		basket1 []int
		basket2 []int
		res     int64
	}{
		{[]int{4, 2, 2, 2}, []int{1, 4, 1, 2}, 1},
		{[]int{2, 3, 4, 1}, []int{3, 2, 5, 1}, -1},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.res, minCost(test.basket1, test.basket2), index)
	}
}
