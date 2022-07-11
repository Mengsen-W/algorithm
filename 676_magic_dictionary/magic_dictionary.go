/*
 * @Date: 2022-07-11
 * @LastEditors: mengsenwang mengsen_wang@163.com
 * @LastEditTime: 2022-07-11
 * @FilePath: /algorithm/676_magic_dictionary/magic_dictionary.go
 */

package main

type trie struct {
	children   [26]*trie
	isFinished bool
}

type MagicDictionary struct {
	*trie
}

func Constructor() MagicDictionary {
	return MagicDictionary{&trie{}}
}

func (d *MagicDictionary) BuildDict(dictionary []string) {
	for _, word := range dictionary {
		cur := d.trie
		for _, c := range word {
			c -= 'a'
			if cur.children[c] == nil {
				cur.children[c] = &trie{}
			}
			cur = cur.children[c]
		}
		cur.isFinished = true
	}
}

func dfs(node *trie, searchWord string, modified bool) bool {
	if searchWord == "" {
		return modified && node.isFinished
	}
	c := searchWord[0] - 'a'
	if node.children[c] != nil && dfs(node.children[c], searchWord[1:], modified) {
		return true
	}
	if !modified {
		for i, child := range node.children {
			if i != int(c) && child != nil && dfs(child, searchWord[1:], true) {
				return true
			}
		}
	}
	return false
}

func (d *MagicDictionary) Search(searchWord string) bool {
	return dfs(d.trie, searchWord, false)
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}
	m := Constructor()
	m.BuildDict([]string{"hello", "leetcode"})
	assert(m.Search("hello") == false)
	assert(m.Search("hhllo") == true)
	assert(m.Search("hell") == false)
	assert(m.Search("leetcoded") == false)
}
