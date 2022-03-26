/*
 * @Date: 2022-03-25 23:07:25
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-03-25 23:30:53
 * @FilePath: /algorithm/682_cal_points/cal_points.go
 */

package main

import "strconv"

func calPoints(ops []string) (ans int) {
	points := []int{}
	for _, op := range ops {
		n := len(points)
		switch op[0] {
		case '+':
			ans += points[n-1] + points[n-2]
			points = append(points, points[n-1]+points[n-2])
		case 'D':
			ans += points[n-1] * 2
			points = append(points, 2*points[n-1])
		case 'C':
			ans -= points[n-1]
			points = points[:len(points)-1]
		default:
			pt, _ := strconv.Atoi(op)
			ans += pt
			points = append(points, pt)
		}
	}
	return
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}

	{
		ops := []string{"5", "2", "C", "D", "+"}
		assert(calPoints(ops) == 30)
	}

	{
		ops := []string{"5", "-2", "4", "C", "D", "9", "+", "+"}
		assert(calPoints(ops) == 27)
	}

	{
		ops := []string{"1"}
		assert(calPoints(ops) == 1)
	}
}
