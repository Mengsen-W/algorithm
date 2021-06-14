/*
 * @Date: 2021-06-14 09:36:26
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-06-14 09:51:59
 */

package main

import "sort"

var PICK int = 0

func guess(n int) int {
	if n > PICK {
		return -1
	} else if n < PICK {
		return 1
	} else {
		return 0
	}
}

func guessNumber(n int) int {
	return sort.Search(n, func(x int) bool { return guess(x) <= 0 })
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed!")
		}
	}
	{
		PICK = 6
		n := 10
		assert(guessNumber(n) == PICK)
	}
	{
		PICK = 1
		n := 1
		assert(guessNumber(n) == PICK)
	}
	{
		PICK = 1
		n := 2
		assert(guessNumber(n) == PICK)
	}
	{
		PICK = 6
		n := 10
		assert(guessNumber(n) == PICK)
	}
	{
		PICK = 2
		n := 2
		assert(guessNumber(n) == PICK)
	}
}
