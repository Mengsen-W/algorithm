/*
 * @Date: 2021-12-28 00:49:27
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-12-28 01:32:04
 */

package main

import (
	"reflect"
	"sort"
)

type trie struct {
	children [26]*trie
	isEnd    bool
}

func (root *trie) insert(word string) {
	node := root
	for _, ch := range word {
		ch -= 'a'
		if node.children[ch] == nil {
			node.children[ch] = &trie{}
		}
		node = node.children[ch]
	}
	node.isEnd = true
}

func (root *trie) dfs(vis []bool, word string) bool {
	if word == "" {
		return true
	}
	if vis[len(word)-1] {
		return false
	}
	vis[len(word)-1] = true
	node := root
	for i, ch := range word {
		node = node.children[ch-'a']
		if node == nil {
			return false
		}
		if node.isEnd && root.dfs(vis, word[i+1:]) {
			return true
		}
	}
	return false
}

func findAllConcatenatedWordsInADict(words []string) (ans []string) {
	sort.Slice(words, func(i, j int) bool { return len(words[i]) < len(words[j]) })

	root := &trie{}
	for _, word := range words {
		if word == "" {
			continue
		}
		vis := make([]bool, len(word))
		if root.dfs(vis, word) {
			ans = append(ans, word)
		} else {
			root.insert(word)
		}
	}
	return
}

func main() {
	assert := func(a, b []string) {
		sort.Strings(a)
		sort.Strings(b)
		if !reflect.DeepEqual(a, b) {
			panic("Not Passed")
		}
	}

	assert(findAllConcatenatedWordsInADict([]string{"cat", "cats", "catsdogcats", "dog", "dogcatsdog", "hippopotamuses", "rat", "ratcatdogcat"}), []string{"catsdogcats", "dogcatsdog", "ratcatdogcat"})
	assert(findAllConcatenatedWordsInADict([]string{"cat", "dog", "catdog"}), []string{"catdog"})
}
