/*
 * @Date: 2022-02-06 02:18:20
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-02-06 02:28:08
 */

package main

func sumOfUnique(nums []int) (ans int) {
	state := map[int]int{}
	for _, num := range nums {
		if state[num] == 0 {
			ans += num
			state[num] = 1
		} else if state[num] == 1 {
			ans -= num
			state[num] = 2
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

	assert(sumOfUnique([]int{1, 2, 3, 2}) == 4)
	assert(sumOfUnique([]int{1, 1, 1, 1, 1}) == 0)
	assert(sumOfUnique([]int{1, 2, 3, 4, 5}) == 15)

}
