/*
 * @Date: 2023-12-24
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-12-24
 * @FilePath: /algorithm/golang/1954_minimum_perimeter/minimum_perimeter.go
 */

// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func minimumPerimeter(neededApples int64) int64 {
	left, right := int64(1), int64(100000)
	ans := int64(0)
	for left <= right {
		mid := (left + right) / 2
		if 2*mid*(mid+1)*(mid*2+1) >= neededApples {
			ans = mid
			right = mid - 1
		} else {
			left = mid + 1
		}
	}
	return ans * 8
}

func main() {
	tests := []struct {
		neededApples int64
		ans          int64
	}{
		{1, 8},
		{13, 16},
		{1000000000, 5040},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, minimumPerimeter(test.neededApples), index)
	}
}
