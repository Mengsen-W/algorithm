package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func minimumLevels(possible []int) int {
	n := len(possible)
	tot := 0
	for _, v := range possible {
		tot += v
	}
	tot = tot*2 - n
	pre := 0
	for i := 0; i < n-1; i++ {
		if possible[i] == 1 {
			pre++
		} else {
			pre--
		}
		if 2*pre > tot {
			return i + 1
		}
	}
	return -1
}

func main() {
	tests := []struct {
		possible []int
		ans      int
	}{
		{[]int{1, 0, 1, 0}, 1},
		{[]int{1, 1, 1, 1, 1}, 3},
		{[]int{0, 0}, -1},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, minimumLevels(test.possible), index)
	}
}
