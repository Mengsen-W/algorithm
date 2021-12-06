/*
 * @Date: 2021-12-06 02:36:16
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-12-06 02:40:04
 */

package main

func truncateSentence(s string, k int) string {
	n, end, count := len(s), 0, 0
	for i := 1; i <= n; i++ {
		if i == n || s[i] == ' ' {
			count++
			if count == k {
				end = i
				break
			}
		}
	}
	return s[:end]
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}
	assert(truncateSentence("Hello how are you Contestant", 4) ==
		"Hello how are you")
	assert(truncateSentence("What is the solution to this problem",
		4) == "What is the solution")
	assert(truncateSentence("chopper is not a tanuki", 5) ==
		"chopper is not a tanuki")
}
