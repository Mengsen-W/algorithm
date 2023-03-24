/*
 * @Date: 2023-03-24
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-03-24
 * @FilePath: /algorithm/golang/1032_StreamChecker/StreamChecker.go
 */

// Package main ...
package main

type Trie struct {
	children [26]*Trie
	isEnd    bool
}

func newTrie() Trie {
	return Trie{}
}

func (this *Trie) Insert(word string) {
	node := this
	for i := len(word) - 1; i >= 0; i-- {
		idx := word[i] - 'a'
		if node.children[idx] == nil {
			node.children[idx] = &Trie{}
		}
		node = node.children[idx]
	}
	node.isEnd = true
}

func (this *Trie) Search(word []byte) bool {
	node := this
	for i, j := len(word)-1, 0; i >= 0 && j < 201; i, j = i-1, j+1 {
		idx := word[i] - 'a'
		if node.children[idx] == nil {
			return false
		}
		node = node.children[idx]
		if node.isEnd {
			return true
		}
	}
	return false
}

type StreamChecker struct {
	trie Trie
	s    []byte
}

func Constructor(words []string) StreamChecker {
	trie := newTrie()
	for _, w := range words {
		trie.Insert(w)
	}
	return StreamChecker{trie, []byte{}}
}

func (this *StreamChecker) Query(letter byte) bool {
	this.s = append(this.s, letter)
	return this.trie.Search(this.s)
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}

	words := []string{"cd", "f", "kl"}
	s := Constructor(words)
	assert(s.Query('a') == false)
	assert(s.Query('b') == false)
	assert(s.Query('c') == false)
	assert(s.Query('d') == true)
	assert(s.Query('e') == false)
	assert(s.Query('f') == true)
	assert(s.Query('h') == false)
	assert(s.Query('i') == false)
	assert(s.Query('j') == false)
	assert(s.Query('k') == false)
	assert(s.Query('l') == true)
}
