// Package main ...
package main

import (
	"math"
	"testing"

	"github.com/stretchr/testify/assert"
)

func distributeCandies(candies int, numPeople int) []int {
	n := numPeople
	// how many people received complete gifts
	p := int(math.Sqrt(float64(2*candies)+0.25) - 0.5)
	remaining := candies - int(float64((p+1)*p)*0.5)
	rows, cols := p/n, p%n

	d := make([]int, n)
	for i := 0; i < n; i++ {
		// complete rows
		d[i] = (i+1)*rows + int(float64(rows*(rows-1)*n)*0.5)
		// cols in the last row
		if i < cols {
			d[i] += i + 1 + rows*n
		}
	}
	// remaining candies
	d[cols] += remaining
	return d
}

func main() {
	tests := []struct {
		candies   int
		numPeople int
		ans       []int
	}{
		{7, 4, []int{1, 2, 3, 1}},
		{10, 3, []int{5, 2, 3}},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, distributeCandies(test.candies, test.numPeople), index)
	}
}
