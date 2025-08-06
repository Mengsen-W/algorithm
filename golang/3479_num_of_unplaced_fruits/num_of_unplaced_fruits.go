// Package main ...
package main

import (
	"math"
	"testing"

	"github.com/stretchr/testify/assert"
)

func numOfUnplacedFruits(fruits []int, baskets []int) int {
	n := len(baskets)
	m := int(math.Sqrt(float64(n)))
	section := (n + m - 1) / m
	count := 0
	maxV := make([]int, section)

	for i := 0; i < n; i++ {
		sec := i / m
		maxV[sec] = max(maxV[sec], baskets[i])
	}

	for _, fruit := range fruits {
		unset := 1
		for sec := 0; sec < section; sec++ {
			if maxV[sec] < fruit {
				continue
			}
			choose := 0
			maxV[sec] = 0
			for i := 0; i < m; i++ {
				pos := sec*m + i
				if pos < n && baskets[pos] >= fruit && choose == 0 {
					baskets[pos] = 0
					choose = 1
				}
				if pos < n {
					maxV[sec] = max(maxV[sec], baskets[pos])
				}
			}
			unset = 0
			break
		}
		count += unset
	}
	return count
}

func main() {
	tests := []struct {
		fruits  []int
		baskets []int
		expect  int
	}{
		{[]int{4, 2, 5}, []int{3, 5, 4}, 1},
		{[]int{3, 6, 1}, []int{6, 4, 7}, 0},
	}
	for index, test := range tests {
		result := numOfUnplacedFruits(test.fruits, test.baskets)
		println(result)
		assert.Equal(&testing.T{}, test.expect, numOfUnplacedFruits(test.fruits, test.baskets), index)
	}
}
