# Calculation of Clebsch-Gordan Coefficient Library

## Introduction
This library aims to provide efficient calculations of Clebsch-Gordan coefficients, 3-j symbols, and 6-j symbols. Currently, a naive implementation of the Clebsch-Gordan coefficient calculation is available, which you can find [here](./naive_cg_coefficient).

## To-Do List

1. Modularize Clebsch-Gordan calculation: The first task is to divide the Clebsch-Gordan calculation into smaller functions, allowing for better organization and reusability of code. By breaking down the process into smaller, self-contained functions, the overall code structure will become more maintainable and easier to understand.

2. Implement arbitrary precision for factorial calculations: Currently, the factorial calculation (n!) in the Clebsch-Gordan coefficient formula might suffer from precision limitations, particularly for large values of n. To address this, an implementation that supports arbitrary precision arithmetic should be incorporated into the library. This will ensure accurate calculations even for high values of n.
