Feature: The sum of all the multiples of 3 or 5 below a posive natural number.

    If we list all the natural numbers below 10 that are multiples
    of 3 or 5, we get 3, 5, 6 and 9. The sum of these multiples is 23.
    Find the sum of all the multiples of 3 or 5 below 1000.

    Scenario: Find the sum of all multiples of 3 or 5 below a positive natural number
        Given the positive natural number 10
        Then the sum of all multiples of natural positive numbers bellow that number is 23
            | natural number |
            | 3              |
            | 5              |

