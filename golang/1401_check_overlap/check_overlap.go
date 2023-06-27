/*
 * @Date: 2023-06-25
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-06-25
 * @FilePath: /algorithm/golang/1401_check_overlap/check_overlap.go
 */

// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func checkOverlap(radius int, xCenter int, yCenter int, x1 int, y1 int, x2 int, y2 int) bool {
	min := func(a int, b int) int {
		if a < b {
			return a
		}
		return b
	}
	dist := 0
	if xCenter < x1 || xCenter > x2 {
		dist += min((x1-xCenter)*(x1-xCenter), (x2-xCenter)*(x2-xCenter))
	}
	if yCenter < y1 || yCenter > y2 {
		dist += min((y1-yCenter)*(y1-yCenter), (y2-yCenter)*(y2-yCenter))
	}
	return dist <= radius*radius
}

func main() {
	{
		radius := 1
		xCenter := 0
		yCenter := 0
		x1 := 1
		y1 := -1
		x2 := 3
		y2 := 1
		assert.Equal(&testing.B{}, checkOverlap(radius, xCenter, yCenter, x1, y1, x2, y2), true)
	}

	{
		radius := 1
		xCenter := 1
		yCenter := 1
		x1 := 1
		y1 := -3
		x2 := 2
		y2 := -1
		assert.Equal(&testing.B{}, checkOverlap(radius, xCenter, yCenter, x1, y1, x2, y2), false)
	}

	{
		radius := 1
		xCenter := 0
		yCenter := 0
		x1 := -1
		y1 := 0
		x2 := 0
		y2 := 1
		assert.Equal(&testing.B{}, checkOverlap(radius, xCenter, yCenter, x1, y1, x2, y2), true)
	}
}
