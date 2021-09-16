/*
 * @Date: 2021-09-16 08:44:23
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-09-16 09:04:07
 * @FilePath: /algorithm/212_find_words/find_words.go
 * @Description: file content
 */

package main

import (
	"reflect"
)

type Trie struct {
	children map[byte]*Trie
	word     string
}

func (t *Trie) Insert(word string) {
	node := t
	for i := range word {
		ch := word[i]
		if node.children[ch] == nil {
			node.children[ch] = &Trie{children: map[byte]*Trie{}}
		}
		node = node.children[ch]
	}
	node.word = word
}

var dirs = []struct{ x, y int }{{-1, 0}, {1, 0}, {0, -1}, {0, 1}}

func findWords(board [][]byte, words []string) (ans []string) {
	t := &Trie{children: map[byte]*Trie{}}
	for _, word := range words {
		t.Insert(word)
	}

	m, n := len(board), len(board[0])

	var dfs func(node *Trie, x, y int)
	dfs = func(node *Trie, x, y int) {
		ch := board[x][y]
		nxt := node.children[ch]
		if nxt == nil {
			return
		}

		if nxt.word != "" {
			ans = append(ans, nxt.word)
			nxt.word = ""
		}

		if len(nxt.children) > 0 {
			board[x][y] = '#'
			for _, d := range dirs {
				nx, ny := x+d.x, y+d.y
				if 0 <= nx && nx < m && 0 <= ny && ny < n && board[nx][ny] != '#' {
					dfs(nxt, nx, ny)
				}
			}
			board[x][y] = ch
		}

		if len(nxt.children) == 0 {
			delete(node.children, ch)
		}
	}
	for i, row := range board {
		for j := range row {
			dfs(t, i, j)
		}
	}

	return
}

func main() {
	assert := func(a, b []string) {
		if !reflect.DeepEqual(a, b) {
			panic("Not Passed")
		}
	}
	{
		board := [][]byte{{'o', 'a', 'a', 'n'},
			{'e', 't', 'a', 'e'},
			{'i', 'h', 'k', 'r'},
			{'i', 'f', 'l', 'v'}}
		words := []string{"oath", "pea", "eat", "rain"}
		ans := []string{"oath", "eat"}
		assert(findWords(board, words), ans)
	}
	{
		board := [][]byte{{'a', 'b'},
			{'c', 'd'}}

		words := []string{"abcd"}
		var ans []string
		assert(findWords(board, words), ans)
	}
}
