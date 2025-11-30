#include <cassert>
#include <vector>

using namespace std;

class Bank {
 private:
  vector<long long> balance;

 public:
  Bank(vector<long long>& balance) : balance(balance) {}

  bool transfer(int account1, int account2, long long money) {
    if (account1 > balance.size() || account2 > balance.size() || balance[account1 - 1] < money) {
      return false;
    }
    balance[account1 - 1] -= money;
    balance[account2 - 1] += money;
    return true;
  }

  bool deposit(int account, long long money) {
    if (account > balance.size()) {
      return false;
    }
    balance[account - 1] += money;
    return true;
  }

  bool withdraw(int account, long long money) {
    if (account > balance.size() || balance[account - 1] < money) {
      return false;
    }
    balance[account - 1] -= money;
    return true;
  }
};

int main() {
  vector<long long> blance{10, 100, 20, 50, 30};
  Bank bank = Bank(blance);
  assert(bank.withdraw(3, 10) == true);
  assert(bank.transfer(5, 1, 20) == true);
  assert(bank.deposit(5, 20) == true);
  assert(bank.transfer(3, 4, 15) == false);
  assert(bank.withdraw(10, 50) == false);
}
