/*
 * @Date: 2024-04-18
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-04-18
 * @FilePath: /algorithm/golang/2007_find_original_array/find_original_array.go
 */

// Package main ...
package main

import (
	"sort"
	"testing"

	"github.com/stretchr/testify/assert"
)

func findOriginalArray(changed []int) []int {
	sort.Ints(changed)
	count := make(map[int]int)
	for _, num := range changed {
		count[num]++
	}
	res := []int{}
	for _, a := range changed {
		if count[a] == 0 {
			continue
		}
		count[a]--

		if count[a*2] == 0 {
			return []int{}
		}
		count[a*2]--

		res = append(res, a)
	}
	return res
}

func main() {
	tests := []struct {
		changed []int
		ans     []int
	}{
		{[]int{1, 3, 4, 2, 6, 8}, []int{1, 3, 4}},
		{[]int{6, 3, 0, 1}, []int{}},
		{[]int{1}, []int{}},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, findOriginalArray(test.changed), index)
	}
}
