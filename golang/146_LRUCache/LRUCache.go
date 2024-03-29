/*
 * @Date: 2023-09-24
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-09-24
 * @FilePath: /algorithm/golang/146_LRUCache/LRUCache.go
 */

// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

type LRUCache struct {
	size       int
	capacity   int
	cache      map[int]*DLinkedNode
	head, tail *DLinkedNode
}

type DLinkedNode struct {
	key, value int
	prev, next *DLinkedNode
}

func initDLinkedNode(key, value int) *DLinkedNode {
	return &DLinkedNode{
		key:   key,
		value: value,
	}
}

func Constructor(capacity int) LRUCache {
	l := LRUCache{
		cache:    map[int]*DLinkedNode{},
		head:     initDLinkedNode(0, 0),
		tail:     initDLinkedNode(0, 0),
		capacity: capacity,
	}
	l.head.next = l.tail
	l.tail.prev = l.head
	return l
}

func (this *LRUCache) Get(key int) int {
	if _, ok := this.cache[key]; !ok {
		return -1
	}
	node := this.cache[key]
	this.moveToHead(node)
	return node.value
}

func (this *LRUCache) Put(key int, value int) {
	if _, ok := this.cache[key]; !ok {
		node := initDLinkedNode(key, value)
		this.cache[key] = node
		this.addToHead(node)
		this.size++
		if this.size > this.capacity {
			removed := this.removeTail()
			delete(this.cache, removed.key)
			this.size--
		}
	} else {
		node := this.cache[key]
		node.value = value
		this.moveToHead(node)
	}
}

func (this *LRUCache) addToHead(node *DLinkedNode) {
	node.prev = this.head
	node.next = this.head.next
	this.head.next.prev = node
	this.head.next = node
}

func (this *LRUCache) removeNode(node *DLinkedNode) {
	node.prev.next = node.next
	node.next.prev = node.prev
}

func (this *LRUCache) moveToHead(node *DLinkedNode) {
	this.removeNode(node)
	this.addToHead(node)
}

func (this *LRUCache) removeTail() *DLinkedNode {
	node := this.tail.prev
	this.removeNode(node)
	return node
}

func main() {
	t := &testing.T{}
	lRUCache := Constructor(2)
	lRUCache.Put(1, 1)                   // 缓存是 {1=1}
	lRUCache.Put(2, 2)                   // 缓存是 {1=1, 2=2}
	assert.Equal(t, lRUCache.Get(1), 1)  // 返回 1
	lRUCache.Put(3, 3)                   // 该操作会使得关键字 2 作废，缓存是 {1=1, 3=3}
	assert.Equal(t, lRUCache.Get(2), -1) // 返回 -1 (未找到)
	lRUCache.Put(4, 4)                   // 该操作会使得关键字 1 作废，缓存是 {4=4, 3=3}
	assert.Equal(t, lRUCache.Get(1), -1) // 返回 -1 (未找到)
	assert.Equal(t, lRUCache.Get(3), 3)  // 返回 3
	assert.Equal(t, lRUCache.Get(4), 4)  // 返回 4
}
