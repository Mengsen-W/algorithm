/*
 * @Author: Mengsen.Wang
 * @Date: 2021-01-29 18:50:01
 * @Last Modified by: Mengsen.Wang
 * @Last Modified time: 2021-01-29 18:50:21
 */

/*
dp[i][k][j] 表示利润
i 表示天数
k 表示允许操作的最大次数
j 表示持有状态 1 表示持有 0表示未持有

dp[i][k][0] = max(dp[i-1][k][0], dp[i-1][k][1] + prices[i])
dp[i][k][1] = max(dp[i-1][k][1], dp[i-1][k-1][0] - prices[i])

dp[0][k][0] = 0
第0天还没有开始 这时利润为0
dp[0][k][1] = -INT_MAX
第0天不可能持有，表示不可能状态
dp[i][0][0] = 0
不允许交易，利润是0
dp[i][0][1] = -INT_MAX
不允许交易，还持有股票，表示不可能状态
*/

#include <vector>

using namespace std;

int max_profit_k_1(vector<int> prices) {}

int main() {}
