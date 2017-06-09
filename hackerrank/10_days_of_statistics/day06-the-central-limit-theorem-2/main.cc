#include <iostream>
#include <cmath>
#include <iomanip>

double NormalCdf(double x, double mu, double sigma) {
    return 0.5 * (1. + std::erf((x - mu) / sigma * M_SQRT1_2));
}

int main() {
    std::cout << std::fixed << std::setprecision(4);
    std::cout << NormalCdf(250, 2.4 * 100, sqrt(100) * 2) << std::endl;
    return 0;
}
