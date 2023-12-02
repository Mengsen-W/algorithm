/*
 * @Date: 2023-12-02
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-12-02
 * @FilePath: /algorithm/golang/1094_car_pooling/car_pooling.go
 */

// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func carPooling(trips [][]int, capacity int) bool {
	toMax := 0
	for _, trip := range trips {
		toMax = max(toMax, trip[2])
	}

	diff := make([]int, toMax+1)
	for _, trip := range trips {
		diff[trip[1]] += trip[0]
		diff[trip[2]] -= trip[0]
	}

	count := 0
	for i := 0; i < toMax; i++ {
		count += diff[i]
		if count > capacity {
			return false
		}
	}
	return true
}

func main() {
	tests := []struct {
		trips    [][]int
		capacity int
		ans      bool
	}{
		{[][]int{{2, 1, 5}, {3, 3, 7}}, 4, false},
		{[][]int{{2, 1, 5}, {3, 3, 7}}, 5, true},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, carPooling(test.trips, test.capacity), index)
	}
}
