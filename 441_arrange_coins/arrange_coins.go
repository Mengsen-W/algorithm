/*
 * @Date: 2021-10-10 09:26:28
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-10-10 09:30:41
 */

package main

import (
	"math"
)

// func arrangeCoins(n int) int {
// 	return sort.Search(n, func(k int) bool { k++; return k*(k+1) > 2*n })
// }

func arrangeCoins(n int) int {
	return int((math.Sqrt(float64(8*n+1)) - 1) / 2)
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}
	assert(arrangeCoins(5) == 2)
	assert(arrangeCoins(8) == 3)
}
