/*
 * @Date: 2023-11-29
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-11-29
 * @FilePath: /algorithm/golang/2236_SmallestInfiniteSet/SmallestInfiniteSet.go
 */

package main

import (
	"testing"

	"github.com/emirpasic/gods/sets/treeset"
	"github.com/stretchr/testify/assert"
)

type SmallestInfiniteSet struct {
	thres int
	s     *treeset.Set
}

func Constructor() SmallestInfiniteSet {
	return SmallestInfiniteSet{
		thres: 1,
		s:     treeset.NewWithIntComparator(),
	}
}

func (this *SmallestInfiniteSet) PopSmallest() int {
	if this.s.Empty() {
		ans := this.thres
		this.thres++
		return ans
	}
	it := this.s.Iterator()
	it.Next()
	ans := it.Value().(int)
	this.s.Remove(ans)
	return ans
}

func (this *SmallestInfiniteSet) AddBack(num int) {
	if num < this.thres {
		this.s.Add(num)
	}
}

func main() {
	t := &testing.T{}
	smallestInfiniteSet := Constructor()
	smallestInfiniteSet.AddBack(2)                        // 2 已经在集合中，所以不做任何变更。
	assert.Equal(t, smallestInfiniteSet.PopSmallest(), 1) // 返回 1 ，因为 1 是最小的整数，并将其从集合中移除。
	assert.Equal(t, smallestInfiniteSet.PopSmallest(), 2) // 返回 2 ，并将其从集合中移除。
	assert.Equal(t, smallestInfiniteSet.PopSmallest(), 3) // 返回 3 ，并将其从集合中移除。
	smallestInfiniteSet.AddBack(1)                        // 将 1 添加到该集合中。
	assert.Equal(t, smallestInfiniteSet.PopSmallest(), 1) // 返回 1 ，因为 1 在上一步中被添加到集合中，
	// 且 1 是最小的整数，并将其从集合中移除。
	assert.Equal(t, smallestInfiniteSet.PopSmallest(), 4) // 返回 4 ，并将其从集合中移除。
	assert.Equal(t, smallestInfiniteSet.PopSmallest(), 5) // 返回 5 ，并将其从集合中移除。
}
