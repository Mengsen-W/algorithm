/*
 * @Date: 2021-11-14 02:00:44
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-11-14 02:10:28
 */

package main

import "strings"

type MapSum map[string]int

func Constructor() MapSum {
	return MapSum{}
}

func (m MapSum) Insert(key string, val int) {
	m[key] = val
}

func (m MapSum) Sum(prefix string) (sum int) {
	for s, v := range m {
		if strings.HasPrefix(s, prefix) {
			sum += v
		}
	}
	return
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("assertion failed")
		}
	}
	ms := Constructor()
	ms.Insert("apple", 3)
	assert(ms.Sum("ap") == 3)
	ms.Insert("app", 2)
	assert(ms.Sum("ap") == 5)
}
