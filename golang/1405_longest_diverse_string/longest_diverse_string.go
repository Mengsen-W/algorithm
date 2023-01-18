/*
 * @Date: 2022-02-07 05:30:12
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-02-07 05:39:06
 */

package main

import (
	"sort"
)

func longestDiverseString(a, b, c int) string {
	ans := []byte{}
	cnt := []struct {
		c  int
		ch byte
	}{{a, 'a'}, {b, 'b'}, {c, 'c'}}
	for {
		sort.Slice(cnt, func(i, j int) bool { return cnt[i].c > cnt[j].c })
		hasNext := false
		for i, p := range cnt {
			if p.c <= 0 {
				break
			}
			m := len(ans)
			if m >= 2 && ans[m-2] == p.ch && ans[m-1] == p.ch {
				continue
			}
			hasNext = true
			ans = append(ans, p.ch)
			cnt[i].c--
			break
		}
		if !hasNext {
			return string(ans)
		}
	}
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}
	assert(longestDiverseString(1, 1, 7) == "ccaccbcc")
	assert(longestDiverseString(2, 2, 1) == "abbac")
	assert(longestDiverseString(7, 1, 0) == "aabaa")
}
