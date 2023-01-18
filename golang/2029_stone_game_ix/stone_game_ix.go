/*
 * @Date: 2022-01-19 16:15:27
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-01-19 16:19:52
 */

package main

func stoneGameIX(stones []int) bool {
	cnt0, cnt1, cnt2 := 0, 0, 0
	for _, val := range stones {
		val %= 3
		if val == 0 {
			cnt0++
		} else if val == 1 {
			cnt1++
		} else {
			cnt2++
		}
	}
	if cnt0%2 == 0 {
		return cnt1 >= 1 && cnt2 >= 1
	}
	return cnt1-cnt2 > 2 || cnt2-cnt1 > 2
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}
	assert(stoneGameIX([]int{2, 1}) == true)
	assert(stoneGameIX([]int{2}) == false)
	assert(stoneGameIX([]int{5, 1, 2, 4, 3}) == false)
}
