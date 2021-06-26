/*
 * @Date: 2021-06-26 09:57:44
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-06-26 15:26:10
 */

#include <array>
#include <cassert>
#include <queue>
#include <string>
#include <unordered_set>
#include <vector>
using namespace std;

struct AStar {
  // 曼哈顿距离
  static constexpr array<array<int, 6>, 6> dist = {{{0, 1, 2, 1, 2, 3},
                                                    {1, 0, 1, 2, 1, 2},
                                                    {2, 1, 0, 3, 2, 1},
                                                    {1, 2, 3, 0, 1, 2},
                                                    {2, 1, 2, 1, 0, 1},
                                                    {3, 2, 1, 2, 1, 0}}};

  // 计算启发函数
  static int getH(const string& status) {
    int ret = 0;
    for (int i = 0; i < 6; ++i) {
      if (status[i] != '0') {
        ret += dist[i][status[i] - '1'];
      }
    }
    return ret;
  };

  AStar(const string& status, int g)
      : status_{status}, g_{g}, h_{getH(status)} {
    f_ = g_ + h_;
  }

  bool operator<(const AStar& that) const { return f_ > that.f_; }

  string status_;
  int f_, g_, h_;
};

class Solution {
 private:
  vector<vector<int>> neighbors = {{1, 3}, {0, 2, 4}, {1, 5},
                                   {0, 4}, {1, 3, 5}, {2, 4}};

 public:
  int slidingPuzzle(vector<vector<int>>& board) {
    // 枚举 status 通过一次交换操作得到的状态
    auto get = [&](string& status) -> vector<string> {
      vector<string> ret;
      int x = status.find('0');
      for (int y : neighbors[x]) {
        swap(status[x], status[y]);
        ret.push_back(status);
        swap(status[x], status[y]);
      }
      return ret;
    };

    string initial;
    for (int i = 0; i < 2; ++i) {
      for (int j = 0; j < 3; ++j) {
        initial += char(board[i][j] + '0');
      }
    }
    if (initial == "123450") {
      return 0;
    }

    priority_queue<AStar> q;
    q.emplace(initial, 0);
    unordered_set<string> seen = {initial};

    while (!q.empty()) {
      AStar node = q.top();
      q.pop();
      for (auto&& next_status : get(node.status_)) {
        if (!seen.count(next_status)) {
          if (next_status == "123450") {
            return node.g_ + 1;
          }
          q.emplace(next_status, node.g_ + 1);
          seen.insert(move(next_status));
        }
      }
    }

    return -1;
  }
};

int main() {
  {
    vector<vector<int>> board{{1, 2, 3}, {4, 0, 5}};
    assert(Solution().slidingPuzzle(board) == 1);
  }
  {
    vector<vector<int>> board{{1, 2, 3}, {5, 4, 0}};
    assert(Solution().slidingPuzzle(board) == -1);
  }
  {
    vector<vector<int>> board{{4, 1, 2}, {5, 0, 3}};
    assert(Solution().slidingPuzzle(board) == 5);
  }
  {
    vector<vector<int>> board{{3, 2, 4}, {1, 5, 0}};
    assert(Solution().slidingPuzzle(board) == 14);
  }
}
