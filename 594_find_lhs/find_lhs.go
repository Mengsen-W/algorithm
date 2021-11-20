/*
 * @Date: 2021-11-20 00:40:35
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-11-20 00:48:43
 */

package main

func findLHS(nums []int) (ans int) {
	cnt := map[int]int{}
	for _, num := range nums {
		cnt[num]++
	}
	for num, c := range cnt {
		if c1 := cnt[num+1]; c1 > 0 && c+c1 > ans {
			ans = c + c1
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

	assert(findLHS([]int{1, 3, 2, 2, 5, 2, 3, 7}) == 5)
	assert(findLHS([]int{1, 2, 3, 4}) == 2)
	assert(findLHS([]int{1, 1, 1, 1}) == 0)
}
