/*
 * @Date: 2022-03-29 01:59:28
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-03-29 02:16:10
 */

package main

func maxConsecutiveAnswers(answerKey string, k int) int {
	max := func(a, b int) int {
		if b > a {
			return b
		}
		return a
	}
	maxConsecutiveChar := func(answerKey string, k int, ch byte) (ans int) {
		left, sum := 0, 0
		for right := range answerKey {
			if answerKey[right] != ch {
				sum++
			}
			for sum > k {
				if answerKey[left] != ch {
					sum--
				}
				left++
			}
			ans = max(ans, right-left+1)
		}
		return
	}
	return max(maxConsecutiveChar(answerKey, k, 'T'),
		maxConsecutiveChar(answerKey, k, 'F'))
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}
	assert(maxConsecutiveAnswers("TTFF", 2) == 4)
	assert(maxConsecutiveAnswers("TFFT", 1) == 3)
	assert(maxConsecutiveAnswers("TTFTTFTT", 1) == 5)
}
