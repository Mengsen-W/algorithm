/*
 * @Date: 2022-11-02
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-11-02
 * @FilePath: /algorithm/1620_best_coordinate/best_coordinate.go
 */

package main

import (
	"math"
	"reflect"
)

func bestCoordinate(towers [][]int, radius int) []int {
	var xMax, yMax, cx, cy, maxQuality int
	max := func(a, b int) int {
		if b > a {
			return b
		}
		return a
	}
	for _, t := range towers {
		xMax = max(xMax, t[0])
		yMax = max(yMax, t[1])
	}
	for x := 0; x <= xMax; x++ {
		for y := 0; y <= yMax; y++ {
			quality := 0
			for _, t := range towers {
				d := (x-t[0])*(x-t[0]) + (y-t[1])*(y-t[1])
				if d <= radius*radius {
					quality += int(float64(t[2]) / (1 + math.Sqrt(float64(d))))
				}
			}
			if quality > maxQuality {
				cx, cy, maxQuality = x, y, quality
			}
		}
	}
	return []int{cx, cy}
}

func main() {
	assert := func(a, b []int) {
		if !reflect.DeepEqual(a, b) {
			panic("Not Passed")
		}
	}
	{
		towers := [][]int{{1, 2, 5}, {2, 1, 7}, {3, 1, 9}}
		radius := 2
		ans := []int{2, 1}
		assert(bestCoordinate(towers, radius), ans)
	}

	{
		towers := [][]int{{23, 11, 21}}
		radius := 9
		ans := []int{23, 11}
		assert(bestCoordinate(towers, radius), ans)
	}

	{
		towers := [][]int{{1, 2, 13}, {2, 1, 7}, {0, 1, 9}}
		radius := 2
		ans := []int{1, 2}
		assert(bestCoordinate(towers, radius), ans)
	}
}
