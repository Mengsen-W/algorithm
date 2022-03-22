/*
 * @Date: 2022-03-22 00:44:55
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-03-22 00:55:18
 * @FilePath: /algorithm/2038_winner_of_game/winner_of_game.go
 */

package main

func winnerOfGame(colors string) bool {
	freq := [2]int{}
	cur, cnt := 'C', 0
	for _, c := range colors {
		if c != cur {
			cur, cnt = c, 1
		} else {
			cnt++
			if cnt >= 3 {
				freq[cur-'A']++
			}
		}
	}
	return freq[0] > freq[1]
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}

	assert(winnerOfGame("AAABABB") == true)
	assert(winnerOfGame("AA") == false)
	assert(winnerOfGame("ABBBBBBBAAA") == false)
}
