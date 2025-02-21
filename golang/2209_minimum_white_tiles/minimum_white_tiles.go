// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

const INF = 0x3f3f3f3f

func minimumWhiteTiles(floor string, numCarpets int, carpetLen int) int {
	n := len(floor)
	d := make([]int, n+1)
	f := make([]int, n+1)
	for i := range d {
		d[i] = INF
		f[i] = INF
	}
	d[0] = 0
	for i := 1; i <= n; i++ {
		d[i] = d[i-1]
		if floor[i-1] == '1' {
			d[i]++
		}
	}
	for j := 1; j <= numCarpets; j++ {
		f[0] = 0
		for i := 1; i <= n; i++ {
			f[i] = f[i-1]
			if floor[i-1] == '1' {
				f[i]++
			}
			f[i] = min(f[i], d[max(0, i-carpetLen)])
		}
		d, f = f, d
	}
	return d[n]
}

func main() {
	tests := []struct {
		floor      string
		numCarpets int
		carpetLen  int
		ans        int
	}{
		{"10110101", 2, 2, 2},
		{"11111", 2, 3, 0},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, minimumWhiteTiles(test.floor, test.numCarpets, test.carpetLen), index)
	}
}
