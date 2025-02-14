// Package main ...
package main

import (
	"sort"
	"testing"

	"github.com/stretchr/testify/assert"
)

func maxDistance(position []int, m int) int {
	sort.Ints(position)
	left, right := 1, position[len(position)-1]-position[0]
	ans := -1
	for left <= right {
		mid := (left + right) / 2
		if check(mid, position, m) {
			ans = mid
			left = mid + 1
		} else {
			right = mid - 1
		}
	}
	return ans
}

func check(x int, position []int, m int) bool {
	pre, cnt := position[0], 1
	for i := 1; i < len(position); i++ {
		if position[i]-pre >= x {
			pre = position[i]
			cnt++
		}
	}
	return cnt >= m
}

func main() {
	tests := []struct {
		position []int
		m        int
		ans      int
	}{
		{[]int{1, 2, 3, 4, 7}, 3, 3},
		{[]int{5, 4, 3, 2, 1, 1000000000}, 2, 999999999},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, maxDistance(test.position, test.m), index)
	}
}
