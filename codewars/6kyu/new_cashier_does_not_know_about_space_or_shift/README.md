## DESCRIPTION

Some new cashiers started to work at your restaurant.

They are good at taking orders, but they don't know how to capitalize words, or use a space bar!

All the orders they create look something like this:

```
"milkshakepizzachickenfriescokeburgerpizzasandwichmilkshakepizza"
```

The kitchen staff are threatening to quit, because of how difficult it is to read the orders.

Their preference is to get the orders as a nice clean string with spaces and capitals like so:

````
"Burger Fries Chicken Pizza Pizza Pizza Sandwich Milkshake Milkshake Coke"
````

The kitchen staff expect the items to be in the same order as they appear in the menu.

The menu items are fairly simple, there is no overlap in the names of the items:

````
1. Burger
2. Fries
3. Chicken
4. Pizza
5. Sandwich
6. Onionrings
7. Milkshake
8. Coke
````

## SOLUTION

1. Define a vector of *MenuItem* structs, each with a name and a sequence number, which determines the order in which the items appear in the menu.
2. The *MenuItem* struct and associated *new* function are defined below the *get_order* function. The *new* function is a constructor that takes a name and sequence number and returns a new MenuItem struct. 
3. Iterate over the menu vector, and for each MenuItem, count the number of times it appears in the input string. This is done by the [matches](https://doc.rust-lang.org/std/primitive.str.html#method.matches) method, which returns an iterator over all non-overlapping matches for a pattern in a string. The [count](https://doc.rust-lang.org/core/iter/trait.Iterator.html#method.count) method returns the number of matches.
4. If the count is greater than zero, return a type [Some](https://doc.rust-lang.org/std/option/enum.Option.html#variant.Some) - an optional value of type [Option](https://doc.rust-lang.org/std/option/) - containing a new string that repeats the menuItem's name followed by a space the same number of times as the count. This is done by the [repeat](https://doc.rust-lang.org/std/primitive.str.html#method.repeat) method. If count is zero, return [None](https://doc.rust-lang.org/std/option/enum.Option.html#variant.None)
5. Collect these strings into a vector, join them into a single string, trim any trailling spaces, and return the result.