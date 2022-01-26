/*
 * @Date: 2022-01-26 01:44:28
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-01-26 02:23:55
 */

package main

type DetectSquares map[int]map[int]int

func Constructor() DetectSquares {
	return DetectSquares{}
}

func (s DetectSquares) Add(point []int) {
	x, y := point[0], point[1]
	if s[y] == nil {
		s[y] = map[int]int{}
	}
	s[y][x]++
}

func (s DetectSquares) Count(point []int) (ans int) {
	x, y := point[0], point[1]
	if s[y] == nil {
		return
	}
	yCnt := s[y]
	for col, colCnt := range s {
		if col != y {
			// 根据对称性，这里可以不用取绝对值
			d := col - y
			ans += colCnt[x] * yCnt[x+d] * colCnt[x+d]
			ans += colCnt[x] * yCnt[x-d] * colCnt[x-d]
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
	ds := Constructor()
	ds.Add([]int{3, 10})
	ds.Add([]int{11, 2})
	ds.Add([]int{3, 2})
	assert(ds.Count([]int{11, 10}) == 1)
	assert(ds.Count([]int{14, 8}) == 0)
	ds.Add([]int{11, 2})
	assert(ds.Count([]int{11, 10}) == 2)

}
