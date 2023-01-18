/*
 * @Date: 2022-01-05 01:15:30
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-01-05 01:26:46
 */

package main

import (
	"reflect"
)

func modifyString(s string) string {
	res := []byte(s)
	n := len(res)
	for i, ch := range res {
		if ch == '?' {
			for b := byte('a'); b <= 'c'; b++ {
				if !(i > 0 && res[i-1] == b || i < n-1 && res[i+1] == b) {
					res[i] = b
					break
				}
			}
		}
	}
	return string(res)
}

func main() {
	assert := func(a, b string) {
		if !reflect.DeepEqual(a, b) {
			panic("Not Passed")
		}
	}
	assert(modifyString("?zs"), "azs")
	assert(modifyString("ubv?w"), "ubvaw")
	assert(modifyString("j?qg??b"), "jaqgacb")
	assert(modifyString("??yw?ipkj?"), "abywaipkja")
}
