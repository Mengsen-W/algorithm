// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func findLHS(nums []int) (ans int) {
	cnt := map[int]int{}
	for _, num := range nums {
		cnt[num]++
	}
	for num, c := range cnt {
		if c1 := cnt[num+1]; c1 > 0 && c+c1 > ans {
			ans = c + c1
		}
	}
	return
}

func main() {
	tests := []struct {
		nums []int
		ans  int
	}{
		{[]int{0, 3, 2, 2, 5, 2, 3, 7}, 5},
		{[]int{0, 2, 3, 4}, 2},
		{[]int{0, 1, 1, 1}, 0},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, findLHS(test.nums), index)
	}
}
