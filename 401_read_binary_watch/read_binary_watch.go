/*
 * @Date: 2021-06-21 08:30:39
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-06-21 08:39:58
 */

package main

import (
	"fmt"
	"math/bits"
)

func readBinaryWatch_enum_HourMinutes(turnedOn int) (ans []string) {
	for h := uint8(0); h < 12; h++ {
		for m := uint8(0); m < 60; m++ {
			if bits.OnesCount8(h)+bits.OnesCount8(m) == turnedOn {
				ans = append(ans, fmt.Sprintf("%d:%02d", h, m))
			}
		}
	}
	return
}

func readBinaryWatch_enum_Binary(turnedOn int) (ans []string) {
	for i := 0; i < 1024; i++ {
		h, m := i>>6, i&63 // 用位运算取出高 4 位和低 6 位
		if h < 12 && m < 60 && bits.OnesCount(uint(i)) == turnedOn {
			ans = append(ans, fmt.Sprintf("%d:%02d", h, m))
		}
	}
	return
}

func main() {
	assert := func(a, b []string) {
		len := len(a)
		for i := 0; i < len; i++ {
			if a[i] != b[i] {
				panic("Not Passed!")
			}
		}
	}
	assert(readBinaryWatch_enum_HourMinutes(1), readBinaryWatch_enum_Binary(1))
	assert(readBinaryWatch_enum_HourMinutes(1), readBinaryWatch_enum_Binary(1))
}
