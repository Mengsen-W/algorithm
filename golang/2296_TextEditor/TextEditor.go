// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

type TextEditor struct {
	left  []byte
	right []byte
}

func Constructor() TextEditor {
	return TextEditor{}
}

func (t *TextEditor) AddText(text string) {
	t.left = append(t.left, text...)
}

func (t *TextEditor) DeleteText(k int) int {
	count := min(k, len(t.left))
	t.left = t.left[:len(t.left)-count]
	return count
}

func (t *TextEditor) CursorLeft(k int) string {
	move := min(k, len(t.left))
	for i := 0; i < move; i++ {
		t.right = append(t.right, t.left[len(t.left)-1])
		t.left = t.left[:len(t.left)-1]
	}
	return t.getLeftText()
}

func (t *TextEditor) CursorRight(k int) string {
	move := min(k, len(t.right))
	for i := 0; i < move; i++ {
		t.left = append(t.left, t.right[len(t.right)-1])
		t.right = t.right[:len(t.right)-1]
	}
	return t.getLeftText()
}

func (t *TextEditor) getLeftText() string {
	start := max(0, len(t.left)-10)
	return string(t.left[start:])
}

func main() {
	textEditor := Constructor()
	textEditor.AddText("leetcode")
	assert.Equal(&testing.T{}, textEditor.DeleteText(4), 4)
	textEditor.AddText("practice")
	assert.Equal(&testing.T{}, textEditor.CursorRight(3), "etpractice")
	assert.Equal(&testing.T{}, textEditor.CursorLeft(8), "leet")
	assert.Equal(&testing.T{}, textEditor.DeleteText(10), 4)
	assert.Equal(&testing.T{}, textEditor.CursorLeft(2), "")
	assert.Equal(&testing.T{}, textEditor.CursorRight(6), "practi")
}
