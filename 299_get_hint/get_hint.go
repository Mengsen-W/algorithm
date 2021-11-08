/*
 * @Date: 2021-11-08 00:08:31
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-11-08 00:17:54
 */

package main

import (
	"fmt"
	"reflect"
)

func getHint(secret, guess string) string {
	bulls := 0
	var cntS, cntG [10]int
	for i := range secret {
		if secret[i] == guess[i] {
			bulls++
		} else {
			cntS[secret[i]-'0']++
			cntG[guess[i]-'0']++
		}
	}
	cows := 0
	for i := 0; i < 10; i++ {
		cows += min(cntS[i], cntG[i])
	}
	return fmt.Sprintf("%dA%dB", bulls, cows)
}

func min(a, b int) int {
	if a > b {
		return b
	}
	return a
}

func main() {
	assert := func(a, b string) {
		if reflect.DeepEqual(a, b) {
			panic("Not Passed")
		}
	}

	assert(getHint("1807", "7810"), "3B1A")
	assert(getHint("1123", "0111"), "1B1A")
	assert(getHint("1", "0"), "0B0A")
	assert(getHint("1", "1"), "0B1A")
}
