#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

// 题意确定为0是最后一个字符，即从倒数第二个字符开始算起
// 找到倒数第二个0的位置，和倒数第一个0之间全为1
// 如果1的个数为偶数则最后一个0为第一种字符
// 如果1的个数为奇数则最后一个0为第二种字符

// 设倒数第二个为i = n - 2
// 则 (n - 2) - i 为 1 的个数
// (n - 2) - i 的奇偶性与 n - i 一致

class Solution {
 public:
  bool isOneBitCharacter(vector<int> bits) {
    int n = bits.size(), i = n - 2;
    while (i >= 0 and bits[i]) --i;
    return (n - i) % 2 == 0;
  }
};

int main() {
  vector<tuple<vector<int>, bool>> tests{{{1, 0, 0}, true}, {{1, 1, 1, 0}, false}};

  for (auto& [bits, expect] : tests) {
    assert(Solution().isOneBitCharacter(bits) == expect);
  }
}
