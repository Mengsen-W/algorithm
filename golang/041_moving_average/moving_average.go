/*
 * @Date: 2022-07-16
 * @LastEditors: mengsenwang mengsen_wang@163.com
 * @LastEditTime: 2022-07-16
 * @FilePath: /algorithm/041_moving_average/moving_average.go
 */

package main

import "reflect"

type MovingAverage struct {
	size, sum int
	q         []int
}

func Constructor(size int) MovingAverage {
	return MovingAverage{size: size}
}

func (m *MovingAverage) Next(val int) float64 {
	if len(m.q) == m.size {
		m.sum -= m.q[0]
		m.q = m.q[1:]
	}
	m.sum += val
	m.q = append(m.q, val)
	return float64(m.sum) / float64(len(m.q))
}

func main() {
	assert := func(a, b float64) {
		if !reflect.DeepEqual(a, b) {
			panic("Not Passed")
		}
	}

	m := Constructor(3)
	assert(m.Next(1), 1.0)
	assert(m.Next(10), 5.5)
	assert(m.Next(3), 4.666666666666667)
	assert(m.Next(5), 6)
}
