/*
 * @Date: 2024-05-08
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-05-08
 * @FilePath: /algorithm/golang/2097_watering_plants/watering_plants.go
 */

// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func wateringPlants(plants []int, capacity int) int {
	ans := 0
	rest := capacity
	for i, plant := range plants {
		if rest >= plant {
			ans++
			rest -= plant
		} else {
			ans += i*2 + 1
			rest = capacity - plant
		}
	}
	return ans
}

func main() {
	tests := []struct {
		plants   []int
		capacity int
		ans      int
	}{
		{[]int{2, 2, 3, 3}, 5, 14},
		{[]int{1, 1, 1, 4, 2, 3}, 4, 30},
		{[]int{7, 7, 7, 7, 7, 7, 7}, 8, 49},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, wateringPlants(test.plants, test.capacity), index)
	}
}
