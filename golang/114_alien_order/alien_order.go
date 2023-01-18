/*
 * @Date: 2022-05-31 10:03:37
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-05-31 10:18:39
 * @FilePath: /algorithm/114_alien_order/alien_order.go
 */

package main

func alienOrder(words []string) string {
	g := map[byte][]byte{}
	inDeg := map[byte]int{}
	for _, c := range words[0] {
		inDeg[byte(c)] = 0
	}
next:
	for i := 1; i < len(words); i++ {
		s, t := words[i-1], words[i]
		for _, c := range t {
			inDeg[byte(c)] += 0
		}
		for j := 0; j < len(s) && j < len(t); j++ {
			if s[j] != t[j] {
				g[s[j]] = append(g[s[j]], t[j])
				inDeg[t[j]]++
				continue next
			}
		}
		if len(s) > len(t) {
			return ""
		}
	}

	order := make([]byte, len(inDeg))
	q := order[:0]
	for u, d := range inDeg {
		if d == 0 {
			q = append(q, u)
		}
	}
	for len(q) > 0 {
		u := q[0]
		q = q[1:]
		for _, v := range g[u] {
			if inDeg[v]--; inDeg[v] == 0 {
				q = append(q, v)
			}
		}
	}
	if cap(q) == 0 {
		return string(order)
	}
	return ""
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}

	assert(alienOrder([]string{"wrt", "wrf", "er", "ett", "rftt"}) == "wertf")
	assert(alienOrder([]string{"z", "x"}) == "zx")
	assert(alienOrder([]string{"z", "x", "z"}) == "")
}
