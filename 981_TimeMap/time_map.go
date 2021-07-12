/*
 * @Date: 2021-07-12 08:28:07
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-07-12 08:47:45
 */

package main

import "sort"

type pair struct {
	timestamp int
	value     string
}

type TimeMap struct {
	m map[string][]pair
}

func Constructor() *TimeMap {
	return &TimeMap{map[string][]pair{}}
}

func (m *TimeMap) Set(key string, value string, timestamp int) {
	m.m[key] = append(m.m[key], pair{timestamp, value})
}

func (m *TimeMap) Get(key string, timestamp int) string {
	pairs := m.m[key]
	i := sort.Search(len(pairs), func(i int) bool { return pairs[i].timestamp > timestamp })
	if i > 0 {
		return pairs[i-1].value
	}
	return ""
}

/**
 * Your TimeMap object will be instantiated and called as such:
 * obj := ;
 * obj.Set(key,value,timestamp);
 * param_2 := obj.Get(key,timestamp);
 */

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed!")
		}
	}
	{
		tv := Constructor()
		tv.Set("foo", "bar", 1)
		assert(tv.Get("foo", 1) == "bar")
		assert(tv.Get("foo", 3) == "bar")
		tv.Set("foo", "bar2", 4)
		assert(tv.Get("foo", 4) == "bar2")
		assert(tv.Get("foo", 5) == "bar2")
	}
	{
		tv := Constructor()
		tv.Set("love", "high", 10)
		tv.Set("love", "low", 20)
		assert(tv.Get("love", 5) == "")
		assert(tv.Get("love", 10) == "high")
		assert(tv.Get("love", 15) == "high")
		assert(tv.Get("love", 20) == "low")
	}
}
