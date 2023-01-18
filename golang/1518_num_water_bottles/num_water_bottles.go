/*
 * @Date: 2021-12-17 08:27:17
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-12-17 08:53:52
 */

package main

func numWaterBottles(numBottles int, numExchange int) int {
	if numBottles < numExchange {
		return numBottles
	}
	return (numBottles-numExchange)/(numExchange-1) + 1 + numBottles
}

func main() {
	assert := func(a, b int) {
		if a != b {
			panic("Not Passed")
		}
	}

	assert(numWaterBottles(9, 3), 13)
	assert(numWaterBottles(15, 4), 19)
	assert(numWaterBottles(5, 5), 6)
	assert(numWaterBottles(2, 3), 2)
}
