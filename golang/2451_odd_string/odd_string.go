/*
 * @Date: 2023-05-25
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-05-25
 * @FilePath: /algorithm/golang/2451_odd_string/odd_string.go
 */

// Package main ...
package main

import "reflect"

func get(word string) []int {
	diff := make([]int, len(word)-1)
	for i := 0; i+1 < len(word); i++ {
		diff[i] = int(word[i+1]) - int(word[i])
	}
	return diff
}

func oddString(words []string) string {
	diff0 := get(words[0])
	diff1 := get(words[1])
	if reflect.DeepEqual(diff0, diff1) {
		for i := 2; i < len(words); i++ {
			if !reflect.DeepEqual(diff0, get(words[i])) {
				return words[i]
			}
		}
	}
	if reflect.DeepEqual(diff0, get(words[2])) {
		return words[1]
	}
	return words[0]
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}

	{
		words := []string{"adc", "wzy", "abc"}
		ans := "abc"
		assert(oddString(words) == ans)
	}

	{
		words := []string{"aaa", "bob", "ccc", "ddd"}
		ans := "bob"
		assert(oddString(words) == ans)
	}
}
