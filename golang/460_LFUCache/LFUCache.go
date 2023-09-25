/*
 * @Date: 2023-09-25
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-09-25
 * @FilePath: /algorithm/golang/460_LFUCache/LFUCache.go
 */

// Package main ...
package main

import (
	"container/list"
	"testing"

	"github.com/stretchr/testify/assert"
)

type entry struct {
	key, value, freq int // freq 表示这本书被看的次数
}

type LFUCache struct {
	capacity   int
	minFreq    int
	keyToNode  map[int]*list.Element
	freqToList map[int]*list.List
}

func Constructor(capacity int) LFUCache {
	return LFUCache{
		capacity:   capacity,
		keyToNode:  map[int]*list.Element{},
		freqToList: map[int]*list.List{},
	}
}

func (c *LFUCache) pushFront(e *entry) {
	if _, ok := c.freqToList[e.freq]; !ok {
		c.freqToList[e.freq] = list.New() // 双向链表
	}
	c.keyToNode[e.key] = c.freqToList[e.freq].PushFront(e)
}

func (c *LFUCache) getEntry(key int) *entry {
	node := c.keyToNode[key]
	if node == nil { // 没有这本书
		return nil
	}
	e := node.Value.(*entry)
	lst := c.freqToList[e.freq]
	lst.Remove(node)    // 把这本书抽出来
	if lst.Len() == 0 { // 抽出来后，这摞书是空的
		delete(c.freqToList, e.freq) // 移除空链表
		if c.minFreq == e.freq {
			c.minFreq++
		}
	}
	e.freq++
	c.pushFront(e) // 放在右边这摞书的最上面
	return e
}

func (c *LFUCache) Get(key int) int {
	if e := c.getEntry(key); e != nil { // 有这本书
		return e.value
	}
	return -1 // 没有这本书
}

func (c *LFUCache) Put(key, value int) {
	if e := c.getEntry(key); e != nil { // 有这本书
		e.value = value // 更新 value
		return
	}
	if len(c.keyToNode) == c.capacity { // 书太多了
		lst := c.freqToList[c.minFreq]                           // 最左边那摞书
		delete(c.keyToNode, lst.Remove(lst.Back()).(*entry).key) // 移除这摞书的最下面的书
		if lst.Len() == 0 {                                      // 这摞书是空的
			delete(c.freqToList, c.minFreq) // 移除空链表
		}
	}
	c.pushFront(&entry{key, value, 1}) // 新书放在「看过 1 次」的最上面
	c.minFreq = 1
}

func main() {
	t := &testing.T{}
	lfu := Constructor(2)
	lfu.Put(1, 1)                  // cache=[1,_], cnt(1)=1
	lfu.Put(2, 2)                  // cache=[2,1], cnt(2)=1, cnt(1)=1
	assert.Equal(t, lfu.Get(1), 1) // 返回 1
	// cache=[1,2], cnt(2)=1, cnt(1)=2
	lfu.Put(3, 3) // 去除键 2 ，因为 cnt(2)=1 ，使用计数最小
	// cache=[3,1], cnt(3)=1, cnt(1)=2
	assert.Equal(t, lfu.Get(2), -1) // 返回 -1（未找到）
	assert.Equal(t, lfu.Get(3), 3)  // 返回 3
	// cache=[3,1], cnt(3)=2, cnt(1)=2
	lfu.Put(4, 4) // 去除键 1 ，1 和 3 的 cnt 相同，但 1 最久未使用
	// cache=[4,3], cnt(4)=1, cnt(3)=2
	assert.Equal(t, lfu.Get(1), -1) // 返回 -1（未找到）
	assert.Equal(t, lfu.Get(3), 3)  // 返回 3
	// cache=[3,4], cnt(4)=1, cnt(3)=3
	assert.Equal(t, lfu.Get(4), 4) // 返回 4
	// cache=[3,4], cnt(4)=2, cnt(3)=3
}
