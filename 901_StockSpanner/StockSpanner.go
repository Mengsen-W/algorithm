/*
 * @Date: 2022-10-21
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-10-21
 * @FilePath: /algorithm/901_StockSpanner/StockSpanner.go
 */

package main

import "math"

type StockSpanner struct {
	stack [][2]int
	idx   int
}

func Constructor() StockSpanner {
	return StockSpanner{[][2]int{{-1, math.MaxInt32}}, -1}
}

func (s *StockSpanner) Next(price int) int {
	s.idx++
	for price >= s.stack[len(s.stack)-1][1] {
		s.stack = s.stack[:len(s.stack)-1]
	}
	s.stack = append(s.stack, [2]int{s.idx, price})
	return s.idx - s.stack[len(s.stack)-2][0]
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}
	s := Constructor()
	assert(s.Next(100) == 1)
	assert(s.Next(80) == 1)
	assert(s.Next(60) == 1)
	assert(s.Next(70) == 2)
	assert(s.Next(60) == 1)
	assert(s.Next(75) == 4)
	assert(s.Next(85) == 6)
}
