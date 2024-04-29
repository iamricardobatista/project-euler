Feature: Sum squares - Squares of the Sum

    The sum of the squares of the first ten natural numbers is,
    1^2 + 2^2 + ... + 10^2 = 385

    The square of the sum of the first ten natural numbers is,
    (1 + 2 + ... + 10)^2 = 55^2 = 3025

    Hence the difference between the sum of the squares of the first ten natural
    numbers and the square of the sum is 3025 - 385 = 2640.

    Find the difference between the sum of the squares of the first one hundred
    natural numbers and the square of the sum .

    Scenario: Sum of squares
        Given a natural number 10
        Then the sum of squares is 385
        And the squares of the sum is 3025
        And the difference between the sum of squares and squares of the sum is 2640

