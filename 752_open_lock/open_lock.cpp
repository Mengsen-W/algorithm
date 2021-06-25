/*
 * @Date: 2021-06-25 09:00:18
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-06-25 09:09:36
 */

#include <cassert>
#include <queue>
#include <string>
#include <unordered_set>
#include <vector>
using namespace std;

struct AStar {
  // 计算启发函数
  static int getH(const string& status, const string& target) {
    int ret = 0;
    for (int i = 0; i < 4; ++i) {
      int dist = abs(int(status[i]) - int(target[i]));
      ret += min(dist, 10 - dist);
    }
    return ret;
  };

  AStar(const string& status, const string& target, int g)
      : status_{status}, g_{g}, h_{getH(status, target)} {
    f_ = g_ + h_;
  }

  bool operator<(const AStar& that) const { return f_ > that.f_; }

  string status_;
  int f_, g_, h_;
};

int openLock(vector<string>& deadends, string target) {
  if (target == "0000") {
    return 0;
  }

  unordered_set<string> dead(deadends.begin(), deadends.end());
  if (dead.count("0000")) {
    return -1;
  }

  auto num_prev = [](char x) -> char { return (x == '0' ? '9' : x - 1); };
  auto num_succ = [](char x) -> char { return (x == '9' ? '0' : x + 1); };

  auto get = [&](string& status) -> vector<string> {
    vector<string> ret;
    for (int i = 0; i < 4; ++i) {
      char num = status[i];
      status[i] = num_prev(num);
      ret.push_back(status);
      status[i] = num_succ(num);
      ret.push_back(status);
      status[i] = num;
    }
    return ret;
  };

  priority_queue<AStar> q;
  q.emplace("0000", target, 0);
  unordered_set<string> seen = {"0000"};

  while (!q.empty()) {
    AStar node = q.top();
    q.pop();
    for (auto&& next_status : get(node.status_)) {
      if (!seen.count(next_status) && !dead.count(next_status)) {
        if (next_status == target) {
          return node.g_ + 1;
        }
        q.emplace(next_status, target, node.g_ + 1);
        seen.insert(move(next_status));
      }
    }
  }

  return -1;
}

int main() {
  {
    vector<string> deadends{"0201", "0101", "0102", "1212", "2002"};
    assert(openLock(deadends, "0202") == 6);
  }
  {
    vector<string> deadends{"8888"};
    assert(openLock(deadends, "0009") == 1);
  }
  {
    vector<string> deadends{"8887", "8889", "8878", "8898",
                            "8788", "8988", "7888", "9888"};
    assert(openLock(deadends, "8888") == -1);
  }
  {
    vector<string> deadends{"0000"};
    assert(openLock(deadends, "8888") == -1);
  }
  return 0;
}