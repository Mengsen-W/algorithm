/*
 * @Date: 2022-06-21 09:53:27
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-06-21 09:56:50
 * @FilePath: /algorithm/1108_defang_i_paddr/defang_i_paddr.go
 */

package main

import (
	"reflect"
	"strings"
)

func defangIPaddr(address string) string {
	return strings.ReplaceAll(address, ".", "[.]")
}

func main() {
	assert := func(a, b string) {
		if !reflect.DeepEqual(a, b) {
			panic("Not Passed")
		}
	}
	assert(defangIPaddr("1.1.1.1"), "1[.]1[.]1[.]1")
	assert(defangIPaddr("255.100.50.0"), "255[.]100[.]50[.]0")
}
