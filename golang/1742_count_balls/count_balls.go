/*
 * @Date: 2022-11-23
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-11-23
 * @FilePath: /algorithm/1742_count_balls/count_balls.go
 */

package main

func countBalls(lowLimit, highLimit int) (ans int) {
	max := func(a, b int) int {
		if a > b {
			return a
		}
		return b
	}
	count := map[int]int{}
	for i := lowLimit; i <= highLimit; i++ {
		sum := 0
		for x := i; x > 0; x /= 10 {
			sum += x % 10
		}
		count[sum]++
		ans = max(ans, count[sum])
	}
	return
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}
	assert(countBalls(1, 10) == 2)
	assert(countBalls(5, 15) == 2)
	assert(countBalls(19, 28) == 2)
}
