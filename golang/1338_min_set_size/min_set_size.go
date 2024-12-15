// Package main ...
package main

import (
	"sort"
	"testing"

	"github.com/stretchr/testify/assert"
)

func minSetSize(arr []int) int {
	freq := make(map[int]int)
	for _, num := range arr {
		freq[num]++
	}

	occ := []int{}
	for _, v := range freq {
		occ = append(occ, v)
	}
	sort.Sort(sort.Reverse(sort.IntSlice(occ)))
	cnt, ans := 0, 0
	for _, c := range occ {
		cnt += c
		ans++
		if cnt*2 >= len(arr) {
			break
		}
	}
	return ans
}

func main() {
	tests := []struct {
		arr []int
		ans int
	}{
		{[]int{3, 3, 3, 3, 5, 5, 5, 2, 2, 7}, 2},
		{[]int{7, 7, 7, 7, 7, 7}, 1},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, minSetSize(test.arr), index)
	}
}
