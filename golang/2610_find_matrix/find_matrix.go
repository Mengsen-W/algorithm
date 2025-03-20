// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func findMatrix(nums []int) [][]int {
	cnt := make(map[int]int)
	for _, x := range nums {
		cnt[x]++
	}

	var ans [][]int
	for len(cnt) > 0 {
		arr := []int{}
		for k, v := range cnt {
			cnt[k] = v - 1
			arr = append(arr, k)
			if cnt[k] == 0 {
				delete(cnt, k)
			}
		}
		ans = append(ans, arr)
	}

	return ans
}

func main() {
	tests := []struct {
		nums []int
		ans  [][]int
	}{
		{[]int{1, 3, 4, 1, 2, 3, 1}, [][]int{{1, 3, 4, 2}, {1, 3}, {1}}},
		{[]int{1, 2, 3, 4}, [][]int{{4, 3, 2, 1}}},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, findMatrix(test.nums), index)
	}
}
