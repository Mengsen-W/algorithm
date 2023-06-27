/*
 * @Date: 2023-06-26
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-06-27
 * @FilePath: /algorithm/golang/2485_pivot_integer/pivot_integer.go
 */

// Package main ...
package main

import (
	"math"
	"testing"

	"github.com/stretchr/testify/assert"
)

func pivotInteger(n int) int {
	t := (n*n + n) / 2
	x := int(math.Sqrt(float64(t)))
	if x*x == t {
		return x
	}
	return -1
}

func main() {
	testMap := map[int]int{
		8: 6,
		1: 1,
		4: -1,
	}
	for key, value := range testMap {
		assert.Equal(&testing.B{}, pivotInteger(key), value)
	}
}
