/*
 * @Date: 2021-06-25 09:00:43
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-06-25 09:15:28
 */

package main

import "container/heap"

type astar struct {
	g, h   int
	status string
}
type hp []astar

func (h hp) Len() int            { return len(h) }
func (h hp) Less(i, j int) bool  { return h[i].g+h[i].h < h[j].g+h[j].h }
func (h hp) Swap(i, j int)       { h[i], h[j] = h[j], h[i] }
func (h *hp) Push(v interface{}) { *h = append(*h, v.(astar)) }
func (h *hp) Pop() interface{}   { a := *h; v := a[len(a)-1]; *h = a[:len(a)-1]; return v }

// 计算启发函数
func getH(status, target string) (ret int) {
	for i := 0; i < 4; i++ {
		dist := abs(int(status[i]) - int(target[i]))
		ret += min(dist, 10-dist)
	}
	return
}

func openLock(deadends []string, target string) int {
	const start = "0000"
	if target == start {
		return 0
	}

	dead := map[string]bool{}
	for _, s := range deadends {
		dead[s] = true
	}
	if dead[start] {
		return -1
	}

	get := func(status string) (ret []string) {
		s := []byte(status)
		for i, b := range s {
			s[i] = b - 1
			if s[i] < '0' {
				s[i] = '9'
			}
			ret = append(ret, string(s))
			s[i] = b + 1
			if s[i] > '9' {
				s[i] = '0'
			}
			ret = append(ret, string(s))
			s[i] = b
		}
		return
	}

	type pair struct {
		status string
		step   int
	}
	h := hp{{0, getH(start, target), start}}
	seen := map[string]bool{start: true}
	for len(h) > 0 {
		node := heap.Pop(&h).(astar)
		for _, nxt := range get(node.status) {
			if !seen[nxt] && !dead[nxt] {
				if nxt == target {
					return node.g + 1
				}
				seen[nxt] = true
				heap.Push(&h, astar{node.g + 1, getH(nxt, target), nxt})
			}
		}
	}
	return -1
}

func abs(x int) int {
	if x < 0 {
		return -x
	}
	return x
}

func min(a, b int) int {
	if a < b {
		return a
	}
	return b
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed!")
		}
	}
	{
		deadends := []string{"0201", "0101", "0102", "1212", "2002"}
		target := "0202"
		assert(openLock(deadends, target) == 6)
	}
	{
		deadends := []string{"8888"}
		target := "0009"
		assert(openLock(deadends, target) == 1)
	}
	{
		deadends := []string{"8887", "8889", "8878", "8898", "8788", "8988", "7888", "9888"}
		target := "8888"
		assert(openLock(deadends, target) == -1)
	}
	{
		deadends := []string{"0000"}
		target := "8888"
		assert(openLock(deadends, target) == -1)
	}
}
