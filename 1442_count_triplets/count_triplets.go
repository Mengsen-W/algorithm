/*
 * @Date: 2021-05-18 08:23:22
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-05-18 08:29:19
 */

package main

func countTriplets(arr []int) (ans int) {
	cnt := map[int]int{}
	total := map[int]int{}
	s := 0
	for k, val := range arr {
		if m, has := cnt[s^val]; has {
			ans += m*k - total[s^val]
		}
		cnt[s]++
		total[s] += k
		s ^= val
	}
	return
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed!")
		}
	}
	assert(countTriplets([]int{2, 3, 1, 6, 7}) == 4)
	assert(countTriplets([]int{1, 1, 1, 1, 1}) == 10)
	assert(countTriplets([]int{2, 3}) == 0)
	assert(countTriplets([]int{1, 3, 5, 7, 9}) == 3)
	assert(countTriplets([]int{7, 11, 12, 9, 5, 2, 7, 17, 22}) == 8)
}
