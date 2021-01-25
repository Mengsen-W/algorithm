/*
 * @Author: Mengsen.Wang
 * @Date: 2021-01-24 22:49:30
 * @Last Modified by: Mengsen.Wang
 * @Last Modified time: 2021-01-24 22:52:01
 */

#include <iostream>
#include <queue>
#include <string>
#include <unordered_set>
#include <vector>

using namespace std;

string plus_up(string s, int& j) {
  if (s[j] == '9') {
    s[j] = '0';
  } else
    s[j] += 1;
  return s;
}

string minus_down(string s, int& j) {
  if (s[j] == '0') {
    s[j] = '9';
  } else
    s[j] -= 1;
  return s;
}

int openLock(vector<string>& deadends, string target) {
  unordered_set<string> deads = {};
  for (string& s : deadends) deads.insert(s);

  unordered_set<string> visited = {};
  queue<string> q = {};
  int step = 0;
  q.push("0000");
  visited.insert("0000");

  while (!q.empty()) {
    int size = q.size();
    for (int i = 0; i < size; ++i) {
      string cur = q.front();
      if (deads.count(cur)) continue;
      if (cur == target) return step;
      std::cout << cur << std::endl;
      q.pop();

      for (int j = 0; j < 4; ++j) {
        string up = plus_up(cur, j);
        if (!visited.count(up)) {
          q.push(up);
          visited.insert(up);
        }
        string down = minus_down(cur, j);
        if (!visited.count(down)) {
          q.push(down);
          visited.insert(down);
        }
      }
    }
    ++step;
  }
  return -1;
}

int openLock_binary(vector<string>& deadends, string target) {
  unordered_set<string> deads = {};
  for (string& s : deadends) deads.insert(s);
  unordered_set<string> q1 = {};
  unordered_set<string> q2 = {};
  unordered_set<string> visited = {};
  int step = 0;
  q1.insert("0000");
  q2.insert(target);
}

int main() {}