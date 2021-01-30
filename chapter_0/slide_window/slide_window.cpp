/*
 * @Author: Mengsen.Wang
 * @Date: 2021-01-28 20:29:16
 * @Last Modified by: Mengsen.Wang
 * @Last Modified time: 2021-01-30 22:19:28
 */

#include <climits>
#include <iostream>
#include <string>
#include <unordered_map>
#include <vector>

using namespace std;

string min_window(string s, string t) {
  unordered_map<char, int> need = {}, window = {};
  for (char c : t) ++need[c];
  int left = 0, right = 0, valid = 0;
  int start = 0, len = INT_MAX;

  while (right < s.size()) {
    char c = s[right];
    ++right;
    if (need.count(c)) {
      ++window[c];
      if (window[c] == need[c]) ++valid;
    }

    while (valid == need.size()) {
      if (right - left < len) {
        start = left;
        len = right - left;
      }
      char d = s[left];
      ++left;

      if (need.count(d)) {
        if (window[d] == need[d]) --valid;
        --window[d];
      }
    }
  }
  return len == INT_MAX ? "" : s.substr(start, len);
}

bool check_inclusion(string t, string s) {
  unordered_map<char, int> need = {}, window = {};
  for (char c : t) ++need[c];

  int left = 0, right = 0, valid = 0;
  while (right < s.size()) {
    char c = s[right];
    ++right;
    if (need.count(c)) {
      ++window[c];
      if (window[c] == need[c]) ++valid;
    }

    while (right - left >= t.size()) {
      if (valid == need.size()) return true;
      char d = s[left];
      ++left;
      if (need.count(d)) {
        if (window[d] == need[d]) --valid;
        --window[d];
      }
    }
  }
  return false;
}

vector<int> find_anagrams(string s, string t) {
  unordered_map<char, int> need, window;
  for (char c : t) ++need[c];

  int left = 0, right = 0, valid = 0;
  vector<int> res = {};

  while (right < s.size()) {
    char c = s[right];
    ++right;
    if (need.count(c)) {
      ++window[c];
      if (window[c] == need[c]) ++valid;
    }

    while (right - left >= t.size()) {
      if (valid == need.size()) res.push_back(left);
      char d = s[left];
      ++left;
      if (need.count(d)) {
        if (window[d] == need[d]) --valid;
        --window[d];
      }
    }
  }
  return res;
}

int length_of_longest_substring(string s) {
  unordered_map<char, int> window;
  int left = 0, right = 0, valid = 0, res = 0;

  while (right < s.size()) {
    char c = s[right];
    ++right;
    ++window[c];
    while (window[c] > 1) {
      char d = s[left];
      ++left;
      --window[d];
    }
    res = max(right - left, res);
  }

  return res;
}

int main() {
  cout << min_window("ADOBECODEBANC", "ABC") << endl;
  cout << min_window("EBBANCF", "ABC") << endl;

  cout << check_inclusion("ab", "eidbaooo") << endl;
  cout << check_inclusion("ab", "eidboaoo") << endl;

  for (int i : find_anagrams("cbaebabacd", "abc")) {
    cout << i << ", ";
  }
  cout << endl;

  cout << length_of_longest_substring("abcabcbb") << endl;
  cout << length_of_longest_substring("bbbbb") << endl;
  cout << length_of_longest_substring("pwwkew") << endl;

  return 0;
}