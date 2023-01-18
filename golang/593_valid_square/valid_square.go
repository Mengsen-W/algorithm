/*
 * @Date: 2022-07-29
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-07-29
 * @FilePath: /algorithm/593_valid_square/valid_square.go
 */

package main

func checkLength(v1, v2 []int) bool {
	return v1[0]*v1[0]+v1[1]*v1[1] == v2[0]*v2[0]+v2[1]*v2[1]
}

func checkMidPoint(p1, p2, p3, p4 []int) bool {
	return p1[0]+p2[0] == p3[0]+p4[0] && p1[1]+p2[1] == p3[1]+p4[1]
}

func calCos(v1, v2 []int) int {
	return v1[0]*v2[0] + v1[1]*v2[1]
}

func help(p1, p2, p3, p4 []int) bool {
	v1 := []int{p1[0] - p2[0], p1[1] - p2[1]}
	v2 := []int{p3[0] - p4[0], p3[1] - p4[1]}
	return checkMidPoint(p1, p2, p3, p4) && checkLength(v1, v2) && calCos(v1, v2) == 0
}

func validSquare(p1, p2, p3, p4 []int) bool {
	if p1[0] == p2[0] && p1[1] == p2[1] {
		return false
	}
	if help(p1, p2, p3, p4) {
		return true
	}
	if p1[0] == p3[0] && p1[1] == p3[1] {
		return false
	}
	if help(p1, p3, p2, p4) {
		return true
	}
	if p1[0] == p4[0] && p1[1] == p4[1] {
		return false
	}
	if help(p1, p4, p2, p3) {
		return true
	}
	return false
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}
	{
		p1 := []int{0, 0}
		p2 := []int{1, 1}
		p3 := []int{1, 0}
		p4 := []int{0, 1}
		assert(validSquare(p1, p2, p3, p4) == true)
	}
	{
		p1 := []int{0, 0}
		p2 := []int{1, 1}
		p3 := []int{1, 0}
		p4 := []int{0, 12}
		assert(validSquare(p1, p2, p3, p4) == false)
	}
	{
		p1 := []int{1, 0}
		p2 := []int{-1, 0}
		p3 := []int{0, 1}
		p4 := []int{0, -1}
		assert(validSquare(p1, p2, p3, p4) == true)
	}
}
