// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func numberOfSets(n int, maxDistance int, roads [][]int) int {
	res := 0
	opened := make([]int, n)
	d := make([][]int, n)

	for mask := 0; mask < (1 << n); mask++ {
		for i := 0; i < n; i++ {
			opened[i] = mask & (1 << i)
		}
		for i := range d {
			d[i] = make([]int, n)
			for j := range d[i] {
				d[i][j] = 1000000
			}
		}
		for _, road := range roads {
			i, j, r := road[0], road[1], road[2]
			if opened[i] > 0 && opened[j] > 0 {
				if r < d[i][j] {
					d[i][j] = r
					d[j][i] = r
				}
			}
		}

		// Floyd-Warshall algorithm
		for k := 0; k < n; k++ {
			if opened[k] > 0 {
				for i := 0; i < n; i++ {
					if opened[i] > 0 {
						for j := i + 1; j < n; j++ {
							if opened[j] > 0 {
								if d[i][k]+d[k][j] < d[i][j] {
									d[i][j] = d[i][k] + d[k][j]
									d[j][i] = d[i][j] // Ensure symmetry
								}
							}
						}
					}
				}
			}
		}

		// Validate
		good := 1
		for i := 0; i < n; i++ {
			if opened[i] > 0 {
				for j := i + 1; j < n; j++ {
					if opened[j] > 0 && d[i][j] > maxDistance {
						good = 0
						break
					}
				}
				if good == 0 {
					break
				}
			}
		}
		res += good
	}
	return res
}

func main() {
	tests := []struct {
		n           int
		maxDistance int
		roads       [][]int
		ans         int
	}{
		{3, 5, [][]int{{0, 1, 2}, {1, 2, 10}, {0, 2, 10}}, 5},
		{3, 5, [][]int{{0, 1, 20}, {0, 1, 10}, {1, 2, 2}, {0, 2, 2}}, 7},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, numberOfSets(test.n, test.maxDistance, test.roads), index)
	}
}
