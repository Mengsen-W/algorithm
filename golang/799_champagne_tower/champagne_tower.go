/*
 * @Date: 2022-11-20
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-11-20
 * @FilePath: /algorithm/799_champagne_tower/champagne_tower.go
 */

package main

import "math"

func champagneTower(poured, queryRow, queryGlass int) float64 {
	row := []float64{float64(poured)}
	for i := 1; i <= queryRow; i++ {
		nextRow := make([]float64, i+1)
		for j, volume := range row {
			if volume > 1 {
				nextRow[j] += (volume - 1) / 2
				nextRow[j+1] += (volume - 1) / 2
			}
		}
		row = nextRow
	}
	return math.Min(1, row[queryGlass])
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}
	assert(champagneTower(1, 1, 1) == 0.0)
	assert(champagneTower(2, 1, 1) == 0.5)
	assert(champagneTower(100000009, 33, 17) == 1.0)
}
