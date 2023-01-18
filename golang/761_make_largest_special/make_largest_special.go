/*
 * @Date: 2022-08-08
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-08-08
 * @FilePath: /algorithm/761_make_largest_special/make_largest_special.go
 */

package main

import (
	"reflect"
	"sort"
	"strings"
)

func makeLargestSpecial(s string) string {
	if len(s) <= 2 {
		return s
	}
	subs := sort.StringSlice{}
	cnt, left := 0, 0
	for i, ch := range s {
		if ch == '1' {
			cnt++
		} else if cnt--; cnt == 0 {
			subs = append(subs, "1"+makeLargestSpecial(s[left+1:i])+"0")
			left = i + 1
		}
	}
	sort.Sort(sort.Reverse(subs))
	return strings.Join(subs, "")
}

func main() {
	func(a, b string) {
		if !reflect.DeepEqual(a, b) {
			panic("Not Passed")
		}
	}(makeLargestSpecial("11011000"), "11100100")
}
