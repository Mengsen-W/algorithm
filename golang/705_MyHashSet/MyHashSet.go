/*
 * @Date: 2024-04-14
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-04-14
 * @FilePath: /algorithm/golang/705_MyHashSet/MyHashSet.go
 */

// Package main ...
package main

import (
	"container/list"
	"testing"

	"github.com/stretchr/testify/assert"
)

const base = 769

type MyHashSet struct {
	data []list.List
}

func Constructor() MyHashSet {
	return MyHashSet{make([]list.List, base)}
}

func (s *MyHashSet) hash(key int) int {
	return key % base
}

func (s *MyHashSet) Add(key int) {
	if !s.Contains(key) {
		h := s.hash(key)
		s.data[h].PushBack(key)
	}
}

func (s *MyHashSet) Remove(key int) {
	h := s.hash(key)
	for e := s.data[h].Front(); e != nil; e = e.Next() {
		if e.Value.(int) == key {
			s.data[h].Remove(e)
		}
	}
}

func (s *MyHashSet) Contains(key int) bool {
	h := s.hash(key)
	for e := s.data[h].Front(); e != nil; e = e.Next() {
		if e.Value.(int) == key {
			return true
		}
	}
	return false
}

func main() {
	myHashSet := Constructor()
	myHashSet.Add(1)                                         // set = [1]
	myHashSet.Add(2)                                         // set = [1, 2]
	assert.Equal(&testing.T{}, myHashSet.Contains(1), true)  // 返回 True
	assert.Equal(&testing.T{}, myHashSet.Contains(3), false) // 返回 False ，（未找到）
	myHashSet.Add(2)                                         // set = [1, 2]
	assert.Equal(&testing.T{}, myHashSet.Contains(2), true)  // 返回 True
	myHashSet.Remove(2)                                      // set = [1]
	assert.Equal(&testing.T{}, myHashSet.Contains(2), false) // 返回 False ，（已移除）
}
