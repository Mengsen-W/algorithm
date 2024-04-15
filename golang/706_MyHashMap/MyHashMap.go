/*
 * @Date: 2024-04-15
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-04-15
 * @FilePath: /algorithm/golang/706_MyHashMap/MyHashMap.go
 */

// Package main ...
package main

import (
	"container/list"
	"testing"

	"github.com/stretchr/testify/assert"
)

const base = 769

type entry struct {
	key, value int
}

type MyHashMap struct {
	data []list.List
}

func Constructor() MyHashMap {
	return MyHashMap{make([]list.List, base)}
}

func (m *MyHashMap) hash(key int) int { return key % base }

func (m *MyHashMap) Put(key, value int) {
	h := m.hash(key)
	for e := m.data[h].Front(); e != nil; e = e.Next() {
		if et := e.Value.(entry); et.key == key {
			e.Value = entry{key, value}
			return
		}
	}
	m.data[h].PushBack(entry{key, value})
}

func (m *MyHashMap) Get(key int) int {
	h := m.hash(key)
	for e := m.data[h].Front(); e != nil; e = e.Next() {
		if et := e.Value.(entry); et.key == key {
			return et.value
		}
	}
	return -1
}

func (m *MyHashMap) Remove(key int) {
	h := m.hash(key)
	for e := m.data[h].Front(); e != nil; e = e.Next() {
		if e.Value.(entry).key == key {
			m.data[h].Remove(e)
		}
	}
}

func main() {
	myHashMap := Constructor()
	myHashMap.Put(1, 1)                              // myHashMap 现在为 [[1,1]]
	myHashMap.Put(2, 2)                              // myHashMap 现在为 [[1,1], [2,2]]
	assert.Equal(&testing.T{}, myHashMap.Get(1), 1)  // 返回 1 ，myHashMap 现在为 [[1,1], [2,2]]
	assert.Equal(&testing.T{}, myHashMap.Get(3), -1) // 返回 -1（未找到），myHashMap 现在为 [[1,1], [2,2]]
	myHashMap.Put(2, 1)                              // myHashMap 现在为 [[1,1], [2,1]]（更新已有的值）
	assert.Equal(&testing.T{}, myHashMap.Get(2), 1)  // 返回 1 ，myHashMap 现在为 [[1,1], [2,1]]
	myHashMap.Remove(2)                              // 删除键为 2 的数据，myHashMap 现在为 [[1,1]]
	assert.Equal(&testing.T{}, myHashMap.Get(2), -1) // 返回 -1（未找到），myHashMap 现在为 [[1,1]]
}
