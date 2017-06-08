#include <iostream>
#include <cmath>
#include <iomanip>

double NormalCdf(double x, double mu, double sigma) {
    return 0.5 * (1. + std::erf((x - mu) / sigma * M_SQRT1_2));
}

int main() {
    std::cout << std::fixed << std::setprecision(2);
    std::cout << (1. - NormalCdf(80, 70, 10)) * 100 << std::endl;
    std::cout << (1. - NormalCdf(60, 70, 10)) * 100 << std::endl;
    std::cout << NormalCdf(60, 70, 10) * 100 << std::endl;
    return 0;
}
