/*
 * @Date: 2021-07-02 14:57:28
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-07-02 15:49:18
 */

package main

func maxIceCream(costs []int, coins int) (ans int) {
	min := func(a, b int) int {
		if a < b {
			return a
		}
		return b
	}
	const mx int = 1e5
	freq := [mx + 1]int{}
	for _, c := range costs {
		freq[c]++
	}
	for i := 1; i <= mx && coins >= i; i++ {
		c := min(freq[i], coins/i)
		ans += c
		coins -= i * c
	}
	return
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed!")
		}
	}
	{
		costs := []int{1, 3, 2, 4, 1}
		coins := 7
		assert(maxIceCream(costs, coins) == 4)
	}
	{
		costs := []int{10, 6, 8, 7, 7, 8}
		coins := 5
		assert(maxIceCream(costs, coins) == 0)
	}
	{
		costs := []int{1, 6, 3, 1, 2, 5}
		coins := 20
		assert(maxIceCream(costs, coins) == 6)
	}
}
