/*
 * @Date: 2022-09-07
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-09-07
 * @FilePath: /algorithm/1592_reorder_spaces/reorder_spaces.go
 */

package main

import (
	"reflect"
	"strings"
)

func reorderSpaces(s string) (ans string) {
	words := strings.Fields(s)
	space := strings.Count(s, " ")
	lw := len(words) - 1
	if lw == 0 {
		return words[0] + strings.Repeat(" ", space)
	}
	return strings.Join(words, strings.Repeat(" ", space/lw)) + strings.Repeat(" ", space%lw)
}

func main() {
	assert := func(a, b string) {
		if !reflect.DeepEqual(a, b) {
			panic("Not Passed")
		}
	}
	{
		text := "  this   is  a sentence "
		ans := "this   is   a   sentence"
		assert(reorderSpaces(text), ans)
	}

	{
		text := " practice   makes   perfect"
		ans := "practice   makes   perfect "
		assert(reorderSpaces(text), ans)
	}

	{
		text := "hello   world"
		ans := "hello   world"
		assert(reorderSpaces(text), ans)
	}

	{
		text := "  walks  udp package   into  bar a"
		ans := "walks  udp  package  into  bar  a "
		assert(reorderSpaces(text), ans)
	}

	{
		text := "a"
		ans := "a"
		assert(reorderSpaces(text), ans)
	}
}
