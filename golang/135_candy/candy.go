// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func candy(ratings []int) int {
	n := len(ratings)
	ans, inc, dec, pre := 1, 1, 0, 1
	for i := 1; i < n; i++ {
		if ratings[i] >= ratings[i-1] {
			dec = 0
			if ratings[i] == ratings[i-1] {
				pre = 1
			} else {
				pre++
			}
			ans += pre
			inc = pre
		} else {
			dec++
			if dec == inc {
				dec++
			}
			ans += dec
			pre = 1
		}
	}
	return ans
}

func main() {
	tests := []struct {
		ratings []int
		ans     int
	}{
		{[]int{1, 0, 2}, 5},
		{[]int{1, 2, 2}, 4},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, candy(test.ratings), index)
	}
}
