/*
 * @Date: 2021-12-12 05:21:56
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-12-12 05:35:16
 */

package main

import (
	"reflect"
	"strings"
)

func toLowerCase(s string) string {
	lower := &strings.Builder{}
	lower.Grow(len(s))
	for _, ch := range s {
		if 65 <= ch && ch <= 90 {
			ch |= 32
		}
		lower.WriteRune(ch)
	}
	return lower.String()
}

func main() {
	assert := func(a, b string) {
		if !reflect.DeepEqual(a, b) {
			panic("Not Passed")
		}
	}

	assert(toLowerCase("Hello"), "hello")
	assert(toLowerCase("here"), "here")
	assert(toLowerCase("LOVELY"), "lovely")
}
