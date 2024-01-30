# Rot13

## DESCRIPTION
ROT13 is a simple letter substitution cipher that replaces a letter with the letter 13 letters after it in the alphabet. ROT13 is an example of the Caesar cipher.

Create a function that takes a string and returns the string ciphered with Rot13. If there are numbers or special characters included in the string, they should be returned as they are. Only letters from the latin/english alphabet should be shifted, like in the original Rot13 "implementation".

## SOLUTION:

1. Loop over characters of the given message and check if the character is a letter from the latin/english alphabet. 
2. If it is, follow these steps to build the new character: 
   1. set the base to the byte literal of **a** (97) if the character is lowercase, or to the byte literal of **A** (65) if the character is uppercase.
   2. the character is converted to a number (u8)
   3. 13 is added to the number
   4. the base is subtracted from the previous steps result
   5. then modulo 26 is applied to the result to ensure that the result is in the range of the alphabetic characters.
   6. the base is added to the result to get the new character
3. If the character is not a letter from the latin/english alphabet, it is returned as is.