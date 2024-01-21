# DESCRIPTION:

In a factory a printer prints labels for boxes. For one kind of boxes the printer has to use colors which, for the sake of simplicity, are named with letters from a to m.

The colors used by the printer are recorded in a control string. For example a "good" control string would be aaabbbbhaijjjm meaning that the printer used three times color a, four times color b, one time color h then one time color a...

Sometimes there are problems: lack of colors, technical malfunction and a "bad" control string is produced e.g. aaaxbbbbyyhwawiwjjjwwm with letters not from a to m.

You have to write a function printer_error which given a string will return the error rate of the printer as a string representing a rational whose numerator is the number of errors and the denominator the length of the control string. Don't reduce this fraction to a simpler expression.

The string has a length greater or equal to one and contains only letters from ato z.

# Examples:

```
s="aaabbbbhaijjjm"
printer_error(s) => "0/14"

s="aaaxbbbbyyhwawiwjjjwwm"
printer_error(s) => "8/22"
```

# Solution

1. put all legal characters in a vector : legal_chars
    - exlusive range 0..10
    - inclusive range 0..=10
    - collect() to create a collection of the range
2. count the errors in the given string : error_count
    - chars() converts the string s into an iterator over it's characters, so that subsequent operations can be performed on each character individually.
    - filter(|x|) & !legal_chars.contains(x) : filters out all the characters in the string that are in legal_chars, leaving only the "error" characters
    - count() the remaining error characters, to_string() because a string is expected in the result
3. count the characters of the string with len() : total_count
4. build up the result error_count/total_count using format! macro
