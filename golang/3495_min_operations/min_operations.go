// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func get(num int) int64 {
	var cnt int64
	i := 1
	base := 1

	for base <= num {
		end := base*2 - 1
		if end > num {
			end = num
		}
		cnt += int64((i+1)/2) * int64(end-base+1)
		i++
		base *= 2
	}
	return cnt
}

func minOperations(queries [][]int) int64 {
	var res int64
	for _, q := range queries {
		count1 := get(q[1])
		count2 := get(q[0] - 1)
		res += (count1 - count2 + 1) / 2
	}
	return res
}

func main() {
	tests := []struct {
		queries [][]int
		ans     int64
	}{
		{[][]int{{1, 2}, {2, 4}}, 3},
		{[][]int{{2, 6}}, 4},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, minOperations(test.queries), index)
	}
}
