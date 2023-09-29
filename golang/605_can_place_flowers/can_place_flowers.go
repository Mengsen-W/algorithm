/*
 * @Date: 2023-09-29
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-09-29
 * @FilePath: /algorithm/golang/605_can_place_flowers/can_place_flowers.go
 */

// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func canPlaceFlowers(flowerbed []int, n int) bool {
	count := 0
	m := len(flowerbed)
	prev := -1
	for i := 0; i < m; i++ {
		if flowerbed[i] == 1 {
			if prev < 0 {
				count += i / 2
			} else {
				count += (i - prev - 2) / 2
			}
			if count >= n {
				return true
			}
			prev = i
		}
	}
	if prev < 0 {
		count += (m + 1) / 2
	} else {
		count += (m - prev - 1) / 2
	}
	return count >= n
}

func main() {
	tests := []struct {
		flowerbed []int
		n         int
		ans       bool
	}{
		{[]int{1, 0, 0, 0, 1}, 1, true},
		{[]int{1, 0, 0, 0, 1}, 2, false},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, canPlaceFlowers(test.flowerbed, test.n), index)
	}
}
