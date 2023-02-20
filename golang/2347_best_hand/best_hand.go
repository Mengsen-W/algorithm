/*
 * @Date: 2023-02-20
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-02-20
 * @FilePath: /algorithm/golang/2347_best_hand/best_hand.go
 */

package main

import "bytes"

func bestHand(ranks []int, suits []byte) string {
	if bytes.Count(suits, suits[:1]) == 5 {
		return "Flush"
	}
	cnt, pair := map[int]int{}, false
	for _, r := range ranks {
		cnt[r]++
		if cnt[r] == 3 {
			return "Three of a Kind"
		}
		if cnt[r] == 2 {
			pair = true
		}
	}
	if pair {
		return "Pair"
	}
	return "High Card"
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}
	{
		ranks := []int{13, 2, 3, 1, 9}
		suits := []byte{'a', 'a', 'a', 'a', 'a'}
		ans := "Flush"
		assert(bestHand(ranks, suits) == ans)
	}

	{
		ranks := []int{4, 4, 2, 4, 4}
		suits := []byte{'d', 'a', 'a', 'b', 'c'}
		ans := "Three of a Kind"
		assert(bestHand(ranks, suits) == ans)
	}

	{
		ranks := []int{10, 10, 2, 12, 9}
		suits := []byte{'a', 'b', 'c', 'a', 'd'}
		ans := "Pair"
		assert(bestHand(ranks, suits) == ans)
	}
}
