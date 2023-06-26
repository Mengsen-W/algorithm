/*
 * @Date: 2023-06-23
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-06-23
 * @FilePath: /algorithm/golang/2496_maximum_value/maximum_value.go
 */

// Package main
package main

import (
	"testing"

	"github.com/stretchr/testify"
)

func maximumValue(strs []string) (ans int) {
	f := func(s string) (x int) {
		for _, c := range s {
			if c >= 'a' && c <= 'z' {
				return len(s)
			}
			x = x*10 + int(c-'0')
		}
		return
	}
	for _, s := range strs {
		if x := f(s); ans < x {
			ans = x
		}
	}
	return
}

func main() {
	{
		strs := []string{"alic3", "bob", "3", "4", "00000"}
		ans := 5
		assert.Equal(&testing.B{}, maximumValue(strs), ans)
	}

	{
		strs := []string{"1", "01", "001", "0001"}
		ans := 1
		assert.Equal(&testing.B{}, maximumValue(strs), ans)
	}
}
