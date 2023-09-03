/*
 * @Date: 2023-09-03
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-09-03
 * @FilePath: /algorithm/golang/1921_eliminate_maximum/eliminate_maximum.go
 */

// Package main ...
package main

import (
	"sort"
	"testing"

	"github.com/stretchr/testify/assert"
)

func eliminateMaximum(dist []int, speed []int) int {
	n := len(dist)
	arrivalTimes := make([]int, n)
	for i := 0; i < n; i++ {
		arrivalTimes[i] = (dist[i]-1)/speed[i] + 1
	}
	sort.Ints(arrivalTimes)
	for i := 0; i < n; i++ {
		if arrivalTimes[i] <= i {
			return i
		}
	}
	return n
}

func main() {
	tests := []struct {
		dist  []int
		speed []int
		ans   int
	}{
		{[]int{1, 3, 4}, []int{1, 1, 1}, 3},
		{[]int{1, 1, 2, 3}, []int{1, 1, 1, 1}, 1},
		{[]int{3, 2, 4}, []int{5, 3, 2}, 1},
	}

	for _, item := range tests {
		assert.Equal(&testing.T{}, item.ans, eliminateMaximum(item.dist, item.speed))
	}
}
