// Package main ...
package main

import (
	"sort"
	"strings"
	"testing"

	"github.com/stretchr/testify/assert"
)

type Trie struct {
	serial   string           // 当前节点结构的序列化表示
	children map[string]*Trie // 当前节点的子节点
}

func deleteDuplicateFolder(paths [][]string) [][]string {
	root := &Trie{children: make(map[string]*Trie)} // 根节点
	// 构建字典树
	for _, path := range paths {
		cur := root
		for _, node := range path {
			if _, ok := cur.children[node]; !ok {
				cur.children[node] = &Trie{children: make(map[string]*Trie)}
			}
			cur = cur.children[node]
		}
	}

	freq := make(map[string]int) // 哈希表记录每一种序列化表示的出现次数
	// 基于深度优先搜索的后序遍历，计算每一个节点结构的序列化表示
	var construct func(*Trie)
	construct = func(node *Trie) {
		if len(node.children) == 0 {
			return // 如果是叶节点，无需操作
		}
		v := make([]string, 0, len(node.children))
		for folder, child := range node.children {
			construct(child)
			v = append(v, folder+"("+child.serial+")")
		}
		sort.Strings(v)
		node.serial = strings.Join(v, "")
		freq[node.serial]++
	}
	construct(root)
	ans := make([][]string, 0)
	path := make([]string, 0)
	// 操作字典树，删除重复文件夹
	var operate func(*Trie)
	operate = func(node *Trie) {
		if freq[node.serial] > 1 {
			return // 如果序列化表示出现超过1次，需要删除
		}

		if len(path) > 0 {
			tmp := make([]string, len(path))
			copy(tmp, path)
			ans = append(ans, tmp)
		}

		for folder, child := range node.children {
			path = append(path, folder)
			operate(child)
			path = path[:len(path)-1]
		}
	}
	operate(root)

	return ans
}

func main() {
	tests := []struct {
		paths [][]string
		ans   [][]string
	}{
		{[][]string{{"a"}, {"c"}, {"d"}, {"a", "b"}, {"c", "b"}, {"d", "a"}}, [][]string{{"d"}, {"d", "a"}}},
		{
			[][]string{{"a"}, {"c"}, {"a", "b"}, {"c", "b"}, {"a", "b", "x"}, {"a", "b", "x", "y"}, {"w"}, {"w", "y"}},
			[][]string{{"c"}, {"c", "b"}, {"a"}, {"a", "b"}},
		},
		{[][]string{{"a", "b"}, {"c", "d"}, {"c"}, {"a"}}, [][]string{{"c"}, {"c", "d"}, {"a"}, {"a", "b"}}},
		{[][]string{{"a"}, {"a", "x"}, {"a", "x", "y"}, {"a", "z"}, {"b"}, {"b", "x"}, {"b", "x", "y"}, {"b", "z"}}, [][]string{}},
		{
			[][]string{{"a"}, {"a", "x"}, {"a", "x", "y"}, {"a", "z"}, {"b"}, {"b", "x"}, {"b", "x", "y"}, {"b", "z"}, {"b", "w"}},
			[][]string{{"b"}, {"b", "w"}, {"b", "z"}, {"a"}, {"a", "z"}},
		},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, deleteDuplicateFolder(test.paths), index)
	}
}
