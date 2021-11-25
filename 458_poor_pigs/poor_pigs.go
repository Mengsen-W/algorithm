/*
 * @Date: 2021-11-25 02:40:34
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-11-25 02:45:58
 */

package main

import "math"

func poorPigs(buckets, minutesToDie, minutesToTest int) int {
	states := minutesToTest/minutesToDie + 1
	return int(math.Ceil(math.Log(float64(buckets)) / math.Log(float64(states))))
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}

	assert(poorPigs(1000, 15, 60) == 5)
	assert(poorPigs(4, 15, 15) == 2)
	assert(poorPigs(4, 15, 30) == 2)
}
