#include <iostream>
#include <cmath>
#include <iomanip>

double NormalCdf(double x, double mu, double sigma) {
    return 0.5 * (1. + std::erf((x - mu) / sigma * M_SQRT1_2));
}

int main() {
    std::cout << std::fixed << std::setprecision(3);
    std::cout << NormalCdf(19.5, 20, 2) << std::endl;
    std::cout << NormalCdf(22, 20, 2) - NormalCdf(20, 20, 2) << std::endl;
    return 0;
}
