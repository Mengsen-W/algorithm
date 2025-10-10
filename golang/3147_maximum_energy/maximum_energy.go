// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func maximumEnergy(energy []int, k int) int {
	n := len(energy)
	ans := -1 << 31

	for i := n - k; i < n; i++ {
		sum := 0
		for j := i; j >= 0; j -= k {
			sum += energy[j]
			ans = max(ans, sum)
		}
	}
	return ans
}

func main() {
	tests := []struct {
		energy []int
		k      int
		ans    int
	}{
		{[]int{5, 2, -10, -5, 1}, 3, 3},
		{[]int{-2, -3, -1}, 2, -1},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, maximumEnergy(test.energy, test.k), index)
	}
}
