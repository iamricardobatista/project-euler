Feature: Largest prime factor

    The prime factors of 13195 are 5, 7,13 and 29.
    What is the largest prime factor of the number 6008514751432 ?

    Scenario: What is the largest prime factor of a given natural number
        Given the prime factors for number 13195
        Then its prime factors are
            | factor |
            | 5      |
            | 7      |
            | 13     |
            | 29     |
