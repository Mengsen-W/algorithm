#include <cassert>
#include <string>
#include <tuple>
#include <unordered_set>
#include <vector>

using namespace std;

class Solution {
 public:
  vector<vector<int>> T;
  unordered_set<long> seen;
  vector<vector<int>> A;

  bool pyramidTransition(string bottom, vector<string>& allowed) {
    // 构建转换表，T[i][j] 表示底部为i和j时，顶部可能的字符位掩码
    T = vector<vector<int>>(7, vector<int>(7, 0));
    for (const string& a : allowed) {
      int left = a[0] - 'A';
      int right = a[1] - 'A';
      int top = a[2] - 'A';
      T[left][right] |= 1 << top;
    }

    seen.clear();
    int N = bottom.size();
    // 金字塔状态数组
    A = vector<vector<int>>(N, vector<int>(N, 0));
    // 初始化底部行
    for (int i = 0; i < N; i++) {
      A[N - 1][i] = bottom[i] - 'A';
    }
    return solve(0, N - 1, 0);
  }

  /**
   * 递归解决金字塔构建问题
   * @param R 当前行的状态编码（用于记忆化）
   * @param N 当前处理的行号
   * @param i 当前行中的位置索引
   * @return 是否可以成功构建金字塔
   */
  bool solve(long R, int N, int i) {
    // 基本情况：成功构建到金字塔顶部
    if (N == 1 && i == 1) {
      return true;
    } else if (i == N) {  // 当前行处理完成，准备处理下一行
      // 记忆化检查：如果已经处理过相同的行状态，直接返回失败
      if (seen.find(R) != seen.end()) {
        return false;
      }
      // 记录当前行状态
      seen.insert(R);
      // 递归处理下一行
      return solve(0, N - 1, 0);
    } else {  // 处理当前行的当前位置
      // 获取当前两个底部块对应的可能顶部块位掩码
      int w = T[A[N][i]][A[N][i + 1]];
      // 遍历所有可能的顶部块
      for (int b = 0; b < 7; ++b) {
        if ((w >> b) & 1) {
          // 设置顶部块
          A[N - 1][i] = b;
          // 递归处理下一个位置，更新状态编码
          // 使用base-8编码来记录当前行的状态
          if (solve(R * 8 + (b + 1), N, i + 1)) {
            return true;
          }
        }
      }
      return false;
    }
  }
};

int main() {
  vector<tuple<string, vector<string>, bool>> tests{
      {"BCD", {"BCC", "CDE", "CEA", "FFF"}, true},
      {"AAAA", {"AAB", "AAC", "BCD", "BBE", "DEF"}, false},
  };

  for (auto& [bottom, allowed, expect] : tests) {
    assert(Solution().pyramidTransition(bottom, allowed) == expect);
  }
}