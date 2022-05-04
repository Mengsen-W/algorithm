/*
 * @Date: 2022-05-04 08:11:52
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-05-04 08:11:52
 * @FilePath: /algorithm/1823_find_the_winner/find_the_winner.go
 */

package main

func findTheWinner(n, k int) int {
	winner := 1
	for i := 2; i <= n; i++ {
		winner = (k+winner-1)%i + 1
	}
	return winner
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}

	assert(findTheWinner(5, 2) == 3)
	assert(findTheWinner(6, 5) == 1)
}
