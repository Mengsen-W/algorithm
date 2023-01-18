/*
 * @Date: 2021-06-19 09:44:02
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-06-19 10:05:42
 */

package main

import "math/bits"

func maxLength(arr []string) (ans int) {
	max := func(a, b int) int {
		if a > b {
			return a
		}
		return b
	}
	masks := []int{0} // 0 对应空串
outer:
	for _, s := range arr {
		mask := 0
		for _, ch := range s {
			ch -= 'a'
			if mask>>ch&1 > 0 { // 若 mask 已有 ch，则说明 s 含有重复字母，无法构成可行解
				continue outer
			}
			mask |= 1 << ch // 将 ch 加入 mask 中
		}
		for _, m := range masks {
			if m&mask == 0 { // m 和 mask 无公共元素
				masks = append(masks, m|mask)
				ans = max(ans, bits.OnesCount(uint(m|mask)))
			}
		}
	}
	return
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed!")
		}
	}
	{
		arr := []string{"un", "iq", "ue"}
		assert(maxLength(arr) == 4)
	}
	{
		arr := []string{"cha", "r", "act", "ers"}
		assert(maxLength(arr) == 6)
	}
	{
		arr := []string{"abcdefghijklmnopqrstuvwxyz"}
		assert(maxLength(arr) == 26)
	}
}
