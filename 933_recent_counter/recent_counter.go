/*
 * @Date: 2022-05-06 07:13:37
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-05-06 07:18:37
 * @FilePath: /algorithm/933_recent_counter/recent_counter.go
 */

package main

type RecentCounter []int

func Constructor() (_ RecentCounter) { return }

func (q *RecentCounter) Ping(t int) int {
	*q = append(*q, t)
	for (*q)[0] < t-3000 {
		*q = (*q)[1:]
	}
	return len(*q)
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}

	rc := Constructor()
	assert(rc.Ping(1) == 1)
	assert(rc.Ping(100) == 2)
	assert(rc.Ping(3001) == 3)
	assert(rc.Ping(3002) == 3)
}
