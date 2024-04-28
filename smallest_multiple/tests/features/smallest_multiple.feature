Feature: Smallest evenly divisible number

    2520 is the smallest number that can be divided by each of the numbers from 1 to 10
    without any remainder.

    What is the smallest positive number that is evenly divisible by all of the numbers
    from 1 to 20?

    Scenario: Smallest evenly divisible number by a set of numbers
        Given a set of numbers from 1 to 10
        Then the smallest number that can be divided by all numbers of the set without remainder is 2520
