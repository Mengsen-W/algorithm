/*
 * @Date: 2022-03-07 00:02:55
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-03-07 00:09:48
 * @FilePath: /algorithm/504_convert_to_base7/convert_to_base7.go
 */

package main

import "reflect"

func convertToBase7(num int) string {
	if num == 0 {
		return "0"
	}
	negative := num < 0
	if negative {
		num = -num
	}
	s := []byte{}
	for num > 0 {
		s = append(s, '0'+byte(num%7))
		num /= 7
	}
	if negative {
		s = append(s, '-')
	}
	for i, n := 0, len(s); i < n/2; i++ {
		s[i], s[n-1-i] = s[n-1-i], s[i]
	}
	return string(s)
}

func main() {
	assert := func(a, b string) {
		if !reflect.DeepEqual(a, b) {
			panic("Not Passed")
		}
	}
	assert(convertToBase7(100), "202")
	assert(convertToBase7(-7), "-10")
}
