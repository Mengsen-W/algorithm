/*
 * @Date: 2021-11-07 02:02:14
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-11-07 02:09:02
 */

package main

func maxCount(m, n int, ops [][]int) int {
	mina, minb := m, n
	for _, op := range ops {
		mina = min(mina, op[0])
		minb = min(minb, op[1])
	}
	return mina * minb
}

func min(a, b int) int {
	if a > b {
		return b
	}
	return a
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}
	assert(maxCount(3, 3, [][]int{{2, 2}, {3, 3}}) == 4)
}
