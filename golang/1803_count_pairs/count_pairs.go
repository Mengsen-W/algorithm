/*
 * @Date: 2023-01-05
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-01-05
 * @FilePath: /algorithm/1803_count_pairs/count_pairs.go
 */

package main

type Trie struct {
	children [2]*Trie
	cnt      int
}

func newTrie() *Trie {
	return &Trie{}
}

func (this *Trie) insert(x int) {
	node := this
	for i := 15; i >= 0; i-- {
		v := (x >> i) & 1
		if node.children[v] == nil {
			node.children[v] = newTrie()
		}
		node = node.children[v]
		node.cnt++
	}
}

func (this *Trie) search(x, limit int) (ans int) {
	node := this
	for i := 15; i >= 0 && node != nil; i-- {
		v := (x >> i) & 1
		if (limit >> i & 1) == 1 {
			if node.children[v] != nil {
				ans += node.children[v].cnt
			}
			node = node.children[v^1]
		} else {
			node = node.children[v]
		}
	}
	return
}

func countPairs(nums []int, low int, high int) (ans int) {
	tree := newTrie()
	for _, x := range nums {
		ans += tree.search(x, high+1) - tree.search(x, low)
		tree.insert(x)
	}
	return
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}

	{
		nums := []int{1, 4, 2, 7}
		low := 2
		high := 6
		ans := 6
		assert(countPairs(nums, low, high) == ans)
	}

	{
		nums := []int{9, 8, 4, 2, 1}
		low := 5
		high := 14
		ans := 8
		assert(countPairs(nums, low, high) == ans)
	}
}
