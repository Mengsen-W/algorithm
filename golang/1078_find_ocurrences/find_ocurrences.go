/*
 * @Date: 2021-12-26 01:07:00
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-12-26 01:47:43
 */

package main

import (
	"reflect"
	"strings"
)

func findOcurrences(text string, first string, second string) (ret []string) {
	splitArr := strings.Split(text, " ")
	for i := 0; i < len(splitArr)-1; i++ {
		if splitArr[i] == first && splitArr[i+1] == second && i+2 < len(splitArr) {
			ret = append(ret, splitArr[i+2])
		}
	}
	return
}

func main() {
	assert := func(a, b []string) {
		if !reflect.DeepEqual(a, b) {
			panic("Not Passed")
		}
	}

	{
		text := "alice is a good girl she is a good student"
		first := "a"
		second := "good"
		assert(findOcurrences(text, first, second), []string{"girl", "student"})
	}

	{
		text := "we will we will rock you"
		first := "we"
		second := "will"
		assert(findOcurrences(text, first, second), []string{"we", "rock"})
	}
}
