/*
 * @Date: 2022-10-01
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-10-01
 * @FilePath: /algorithm/1694_reformat_number/reformat_number.go
 */

package main

import (
	"reflect"
	"strings"
)

func reformatNumber(number string) string {
	s := strings.ReplaceAll(number, " ", "")
	s = strings.ReplaceAll(s, "-", "")
	ans := []string{}
	i := 0
	for ; i+4 < len(s); i += 3 {
		ans = append(ans, s[i:i+3])
	}
	s = s[i:]
	if len(s) < 4 {
		ans = append(ans, s)
	} else {
		ans = append(ans, s[:2], s[2:])
	}
	return strings.Join(ans, "-")
}

func main() {
	assert := func(a, b string) {
		if !reflect.DeepEqual(a, b) {
			panic("Not Passed")
		}
	}
	assert(reformatNumber("1-23-45 6"), "123-456")
	assert(reformatNumber("123 4-567"), "123-45-67")
	assert(reformatNumber("123 4-5678"), "123-456-78")
	assert(reformatNumber("12"), "12")
	assert(reformatNumber("--17-5 229 35-39475 "), "175-229-353-94-75")
}
