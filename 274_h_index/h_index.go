/*
 * @Date: 2021-07-12 08:24:51
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-07-12 08:26:54
 */

package main

func hIndex(citations []int) (h int) {
	n := len(citations)
	counter := make([]int, n+1)
	for _, citation := range citations {
		if citation >= n {
			counter[n]++
		} else {
			counter[citation]++
		}
	}
	for i, tot := n, 0; i >= 0; i-- {
		tot += counter[i]
		if tot >= i {
			return i
		}
	}
	return 0
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed!")
		}
	}
	citations := []int{0, 1, 3, 5, 6}
	assert(hIndex(citations) == 3)
}
