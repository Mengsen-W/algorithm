/*
 * @Date: 2021-11-09 01:24:22
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-11-09 02:08:14
 */

package main

var m map[string]int

const MAX_STEP int = 6

func findMinStep(board string, hand string) int {
	m = make(map[string]int)
	ans := dfs(board, hand)
	if ans > len(hand) {
		return -1
	}
	return ans
}

func dfs(s string, h string) int {
	key := s + "+" + h
	if _, exist := m[key]; exist {
		return m[key]
	}
	if len(s) == 0 {
		return 0
	}
	if len(h) == 0 {
		m[key] = MAX_STEP
		return MAX_STEP
	}
	ans := MAX_STEP
	for i := 0; i <= len(s); i++ {
		for j := 0; j < len(h); j++ {
			tmp := dfs(remove(s, i, h[j]), h[:j]+h[j+1:]) + 1
			if tmp < ans {
				ans = tmp
			}
		}
	}
	m[key] = ans
	return ans
}

func remove(s string, p int, h byte) string {
	if p == 0 {
		if len(s) >= 2 && h == s[0] && s[0] == s[1] {
			return s[2:]
		}
		return string(h) + s
	} else if p == len(s) {
		if len(s) >= 2 && h == s[len(s)-1] && s[len(s)-1] == s[len(s)-2] {
			return s[:len(s)-2]
		}
		return s[:1] + string(h) + s[1:]
	} else {
		s = s[:p] + string(h) + s[p:]
		for {
			do := false
			cnt := 0
			for i := 0; i < len(s)-1; i++ {
				if s[i] == s[i+1] {
					cnt++
				} else {
					if cnt >= 2 {
						j := i - cnt
						s = s[:j] + s[i+1:]
						do = true
						cnt = 0
						break
					}
					cnt = 0
				}
			}
			if cnt >= 2 {
				j := len(s) - 1 - cnt
				s = s[:j]
				cnt = 0
			}
			if !do {
				break
			}
		}
		return s
	}
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}
	assert(findMinStep("WRRBBW", "RB") == -1)
	assert(findMinStep("WWRRBBWW", "WRBRW") == 2)
	assert(findMinStep("G", "GGGGG") == 2)
	assert(findMinStep("RBYYBBRRB", "YRBGB") == 3)
}
