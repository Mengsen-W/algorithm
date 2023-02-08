/*
 * @Date: 2023-02-08
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-02-08
 * @FilePath: /algorithm/golang/1223_remove_subfolders/remove_subfolders.go
 */

package main

import (
	"reflect"
	"strings"
)

type Trie struct {
	children map[string]*Trie
	state    bool
}

func newTrie() *Trie {
	return &Trie{
		children: map[string]*Trie{},
		state:    false,
	}
}

func (node *Trie) insert(s string) {
	path := strings.Split(s, "/")[1:]
	for _, c := range path {
		if _, ok := node.children[c]; !ok {
			node.children[c] = newTrie()
		}
		node = node.children[c]
		if node.state {
			return
		}
	}
	node.state = true
}

func removeSubfolders(folder []string) []string {
	trie := newTrie()
	for _, s := range folder {
		trie.insert(s)
	}
	res := []string{}

	var dfs func(*Trie, string)
	dfs = func(node *Trie, path string) {
		for k, v := range node.children {
			if v.state {
				res = append(res, path+"/"+k)
			} else {
				dfs(v, path+"/"+k)
			}
		}
	}

	dfs(trie, "")
	return res

}

func main() {
	assert := func(a, b []string) {
		if !reflect.DeepEqual(a, b) {
			panic("Not Passed")
		}
	}

	{
		folder := []string{"/a", "/a/b", "/c/d", "/c/d/e", "/c/f"}
		ans := []string{"/a", "/c/d", "/c/f"}
		assert(removeSubfolders(folder), ans)
	}

	{
		folder := []string{"/a", "/a/b/c", "/a/b/d"}
		ans := []string{"/a"}
		assert(removeSubfolders(folder), ans)
	}

	{
		folder := []string{"/a/b/c", "/a/b/ca", "/a/b/d"}
		ans := []string{"/a/b/c", "/a/b/ca", "/a/b/d"}
		assert(removeSubfolders(folder), ans)
	}
}
