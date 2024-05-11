/*
 * @Date: 2024-05-11
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-05-11
 * @FilePath: /algorithm/golang/2391_garbage_collection/garbage_collection.go
 */

// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func garbageCollection(garbage []string, travel []int) int {
	distance := make(map[rune]int)
	res := 0
	curDis := 0
	for i, item := range garbage {
		res += len(item)
		if i > 0 {
			curDis += travel[i-1]
		}
		for _, c := range item {
			distance[c] = curDis
		}
	}
	for _, v := range distance {
		res += v
	}
	return res
}

func main() {
	tests := []struct {
		garbage []string
		travel  []int
		ans     int
	}{
		{[]string{"G", "P", "GP", "GG"}, []int{2, 4, 3}, 21},
		{[]string{"MMM", "PGM", "GP"}, []int{3, 10}, 37},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, garbageCollection(test.garbage, test.travel), index)
	}
}
