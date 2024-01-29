## DESCRIPTION

An orderly trail of ants is marching across the park picnic area.

It looks something like this:

```
..ant..ant.ant...ant.ant..ant.ant....ant..ant.ant.ant...ant..
```
But suddenly there is a rumour that a dropped chicken sandwich has been spotted on the ground ahead. The ants surge forward! Oh No, it's an ant stampede!!

Some of the slower ants are trampled, and their poor little ant bodies are broken up into scattered bits.

The resulting carnage looks like this:

```
...ant...ant..nat.ant.t..ant...ant..ant..ant.anant..t
```

Can you find how many ants have died?

## Notes

When in doubt, assume that the scattered bits are from the same ant. e.g. 2 heads and 1 body = 2 dead ants, not 3

## SOLUTION

1. Count the total of 'a', 'n' and 't' characters in the string. Do this with the 'matches' and 'count' methods.
2. Count the total of live ants by counting the total of 'ant' substrings in the string. 
3. Subtract the total of live ants from the total of 'a', 'n' and 't' characters. 'saturating_sub' is used to avoid negative numbers.
4. Create an array of the counts, iterate over it and return the maximum value (the maximum number of dead ants).

## REFACTOR

1. Use an iterator to count the total of 'a', 'n' and 't' characters in the string. Collect the results in a vector 'counts'.
2. The calculation of dead ants is simplified by using the 'map' method on the 'counts' vector to subtract the number of live ants from each count. Again with 'saturating_sub' to avoid negative numbers. The 'max' method is used to return the maximum number of dead ants. 

