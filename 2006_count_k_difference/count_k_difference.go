/*
 * @Date: 2022-02-08 23:56:19
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-02-09 00:13:47
 */

package main

func countKDifference(nums []int, k int) (ans int) {
	cnt := map[int]int{}
	for _, num := range nums {
		ans += cnt[num-k] + cnt[num+k]
		cnt[num]++
	}
	return
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}

	assert(countKDifference([]int{1, 2, 2, 1}, 1) == 4)
	assert(countKDifference([]int{1, 3}, 3) == 0)
	assert(countKDifference([]int{3, 2, 1, 5, 4}, 2) == 3)

}
