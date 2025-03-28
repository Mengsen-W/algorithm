// Package main ...
package main

import (
	"math"
	"testing"

	"github.com/stretchr/testify/assert"
)

func nonSpecialCount(l int, r int) int {
	n := int(math.Sqrt(float64(r)))
	v := make([]int, n+1)
	res := r - l + 1
	for i := 2; i <= n; i++ {
		if v[i] == 0 {
			if i*i >= l && i*i <= r {
				res--
			}
			for j := i * 2; j <= n; j += i {
				v[j] = 1
			}
		}
	}
	return res
}

func main() {
	tests := []struct {
		l   int
		r   int
		ans int
	}{
		{5, 7, 3},
		{4, 16, 11},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, nonSpecialCount(test.l, test.r), index)
	}
}
