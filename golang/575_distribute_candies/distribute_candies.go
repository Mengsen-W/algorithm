/*
 * @Date: 2021-11-01 01:10:58
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-11-01 01:17:30
 */

package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func distributeCandies(candyType []int) int {
	set := map[int]struct{}{}
	for _, t := range candyType {
		set[t] = struct{}{}
	}
	ans := len(candyType) / 2
	if len(set) < ans {
		ans = len(set)
	}
	return ans
}

func main() {
	tests := []struct {
		candyType []int
		ans       int
	}{
		{[]int{1, 1, 2, 2, 3, 3}, 3},
		{[]int{1, 1, 2, 3}, 2},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, distributeCandies(test.candyType), index)
	}
}
