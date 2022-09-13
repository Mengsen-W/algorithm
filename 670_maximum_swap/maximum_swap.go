/*
 * @Date: 2022-09-13
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-09-13
 * @FilePath: /algorithm/670_maximum_swap/maximum_swap.go
 */

package main

import "strconv"

func maximumSwap(num int) int {
	s := []byte(strconv.Itoa(num))
	n := len(s)
	maxIdx, idx1, idx2 := n-1, -1, -1
	for i := n - 1; i >= 0; i-- {
		if s[i] > s[maxIdx] {
			maxIdx = i
		} else if s[i] < s[maxIdx] {
			idx1, idx2 = i, maxIdx
		}
	}
	if idx1 < 0 {
		return num
	}
	s[idx1], s[idx2] = s[idx2], s[idx1]
	v, _ := strconv.Atoi(string(s))
	return v
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}

	assert(maximumSwap(2736) == 7236)
	assert(maximumSwap(9973) == 9973)

}
