/*
 * @Date: 2022-04-09 07:35:19
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-04-09 07:37:54
 * @FilePath: /algorithm/780_reaching_points/reaching_points.go
 */

package main

func reachingPoints(sx, sy, tx, ty int) bool {
	for tx > sx && ty > sy && tx != ty {
		if tx > ty {
			tx %= ty
		} else {
			ty %= tx
		}
	}
	switch {
	case tx == sx && ty == sy:
		return true
	case tx == sx:
		return ty > sy && (ty-sy)%tx == 0
	case ty == sy:
		return tx > sx && (tx-sx)%ty == 0
	default:
		return false
	}
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}

	assert(reachingPoints(1, 1, 3, 5) == true)
	assert(reachingPoints(1, 1, 2, 2) == false)
	assert(reachingPoints(1, 1, 1, 1) == true)
}
