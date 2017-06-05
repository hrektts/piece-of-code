#include <iostream>
#include <iomanip>
#include <limits>

using namespace std;

int main() {
    int i = 4;
    double d = 4.0;
    string s = "HackerRank ";

    int ii;
    double dd;
    string ss;

    cin >> ii >> dd;
    cin.ignore();
    getline(cin, ss);

    cout << i + ii
         << "\n"
         << fixed
         << setprecision(1)
         << d + dd
         << "\n"
         << s + ss
         << "\n";

    return 0;
}
