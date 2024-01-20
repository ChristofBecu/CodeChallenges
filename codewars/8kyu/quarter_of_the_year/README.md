## DESCRIPTION:

Given a month as an integer from 1 to 12, return to which quarter of the year it belongs as an integer number.

For example: month 2 (February), is part of the first quarter; month 6 (June), is part of the second quarter; and month 11 (November), is part of the fourth quarter.

# Constraint:

- 1 <= month <= 12

## SOLUTION

1. a quarter of one year has 3 months, this is divisor
2. to get which quarter belongs the given month, add 2 to the month, to get to the maximum months of the quarter
3. divide result with divisor