// Package main ...
package main

import (
	"sort"
	"testing"

	"github.com/stretchr/testify/assert"
)

func maxConsecutive(bottom int, top int, special []int) int {
	special = append(special, bottom-1)
	special = append(special, top+1)
	sort.Ints(special)
	ans := 0
	for i := 0; i < len(special)-1; i++ {
		ans = max(ans, special[i+1]-special[i]-1)
	}
	return ans
}

func main() {
	tests := []struct {
		bottom  int
		top     int
		special []int
		ans     int
	}{
		{2, 9, []int{4, 6}, 3},
		{6, 8, []int{7, 6, 8}, 0},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, maxConsecutive(test.bottom, test.top, test.special), index)
	}
}
