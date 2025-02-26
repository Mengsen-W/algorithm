package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

type BrowserHistory struct {
	urls      []string
	currIndex int
}

func Constructor(homepage string) BrowserHistory {
	return BrowserHistory{
		urls:      []string{homepage},
		currIndex: 0,
	}
}

func (b *BrowserHistory) Visit(url string) {
	for len(b.urls) > b.currIndex+1 {
		b.urls = b.urls[:b.currIndex+1]
	}
	b.urls = append(b.urls, url)
	b.currIndex++
}

func (b *BrowserHistory) Back(steps int) string {
	b.currIndex = max(b.currIndex-steps, 0)
	return b.urls[b.currIndex]
}

func (b *BrowserHistory) Forward(steps int) string {
	b.currIndex = min(b.currIndex+steps, len(b.urls)-1)
	return b.urls[b.currIndex]
}

func main() {
	browserHistory := Constructor("leetcode.com")
	browserHistory.Visit("google.com")
	browserHistory.Visit("facebook.com")
	browserHistory.Visit("youtube.com")
	assert.Equal(&testing.T{}, browserHistory.Back(1), "facebook.com")
	assert.Equal(&testing.T{}, browserHistory.Back(1), "google.com")
	assert.Equal(&testing.T{}, browserHistory.Forward(1), "facebook.com")
	browserHistory.Visit("linkedin.com")
	assert.Equal(&testing.T{}, browserHistory.Forward(2), "linkedin.com")
	assert.Equal(&testing.T{}, browserHistory.Back(2), "google.com")
	assert.Equal(&testing.T{}, browserHistory.Back(7), "leetcode.com")
}
