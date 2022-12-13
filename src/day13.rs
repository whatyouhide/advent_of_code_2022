use std::cmp::Ordering;

#[derive(Debug, PartialEq, Eq)]
enum Value {
    Int(u16),
    List(Box<LinkedList>),
}

#[derive(Debug, PartialEq, Eq)]
enum LinkedList {
    Empty,
    Cons(Value, Box<LinkedList>),
}

impl LinkedList {
    pub fn from_string(string: &str) -> LinkedList {
        let mut chars = string.chars().skip(1);
        Self::from_chars(&mut chars)
    }

    fn from_chars(chars: &mut impl Iterator<Item = char>) -> LinkedList {
        let mut char_digits = String::new();

        loop {
            match chars.next().unwrap() {
                '[' => {
                    return LinkedList::Cons(
                        Value::List(Box::new(Self::from_chars(chars))),
                        Box::new(Self::from_chars(chars)),
                    );
                }
                ']' => {
                    if char_digits.is_empty() {
                        return LinkedList::Empty;
                    } else {
                        let int = char_digits.parse::<u16>().unwrap();
                        return LinkedList::Cons(Value::Int(int), Box::new(LinkedList::Empty));
                    }
                }
                ',' => {
                    if !char_digits.is_empty() {
                        let int = char_digits.parse::<u16>().unwrap();
                        return LinkedList::Cons(
                            Value::Int(int),
                            Box::new(Self::from_chars(chars)),
                        );
                    } else {
                        continue;
                    }
                }
                char => {
                    char_digits.push(char);
                }
            }
        }
    }
}

impl PartialOrd for LinkedList {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (LinkedList::Empty, LinkedList::Empty) => Some(Ordering::Equal),

            // If the left list runs out of items first, the inputs are in the right order.
            (LinkedList::Empty, _) => Some(Ordering::Less),

            // If the right list runs out of items first, the inputs are in the wrong order.
            (_, LinkedList::Empty) => Some(Ordering::Greater),

            (LinkedList::Cons(value, tail), LinkedList::Cons(other_value, other_tail)) => {
                // If both values are integers, the lower integer should come first.
                match (value, other_value) {
                    (Value::Int(int), Value::Int(other_int)) => match int.partial_cmp(other_int) {
                        Some(Ordering::Equal) => tail.partial_cmp(other_tail),
                        Some(ordering) => Some(ordering),
                        None => None,
                    },

                    // If exactly one value is an integer, convert the integer to a list which
                    // contains that integer as its only value, then retry the comparison.
                    (Value::Int(int), Value::List(list)) => {
                        let wrapped_int =
                            LinkedList::Cons(Value::Int(*int), Box::new(LinkedList::Empty));

                        match wrapped_int.partial_cmp(list) {
                            Some(Ordering::Equal) => tail.partial_cmp(other_tail),
                            Some(ordering) => Some(ordering),
                            None => None,
                        }
                    }

                    // If exactly one value is an integer, convert the integer to a list which
                    // contains that integer as its only value, then retry the comparison.
                    (Value::List(list), Value::Int(int)) => {
                        let wrapped_int =
                            LinkedList::Cons(Value::Int(*int), Box::new(LinkedList::Empty));

                        match list.partial_cmp(&Box::new(wrapped_int)) {
                            Some(Ordering::Equal) => tail.partial_cmp(other_tail),
                            Some(ordering) => Some(ordering),
                            None => None,
                        }
                    }

                    (Value::List(list), Value::List(other_list)) => {
                        match list.partial_cmp(other_list) {
                            Some(Ordering::Equal) => tail.partial_cmp(other_tail),
                            Some(ordering) => Some(ordering),
                            None => None,
                        }
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod linked_list_tests {
    use super::*;

    #[test]
    fn test_from_string_with_empty_list() {
        let list = LinkedList::from_string("[]");
        assert_eq!(list, LinkedList::Empty);
    }

    #[test]
    fn test_from_string_with_list_with_one_element() {
        assert_eq!(
            LinkedList::from_string("[7]"),
            LinkedList::Cons(Value::Int(7), Box::new(LinkedList::Empty))
        );
        assert_eq!(
            LinkedList::from_string("[23]"),
            LinkedList::Cons(Value::Int(23), Box::new(LinkedList::Empty))
        );
    }

    #[test]
    fn test_from_string_with_list_with_two() {
        let list = LinkedList::from_string("[7,3]");
        assert_eq!(
            list,
            LinkedList::Cons(
                Value::Int(7),
                Box::new(LinkedList::Cons(Value::Int(3), Box::new(LinkedList::Empty)))
            )
        );
    }

    #[test]
    fn test_from_string_with_nested_empty_lists() {
        let list = LinkedList::from_string("[[]]");

        assert_eq!(
            list,
            LinkedList::Cons(
                Value::List(Box::new(LinkedList::Empty)),
                Box::new(LinkedList::Empty)
            )
        );
    }

    #[test]
    fn test_from_string_with_complex_nested_list() {
        let list = LinkedList::from_string("[[],1,[2,3],4,[5,[6],7]]");

        assert_eq!(
            list,
            LinkedList::Cons(
                Value::List(Box::new(LinkedList::Empty)),
                Box::new(LinkedList::Cons(
                    Value::Int(1),
                    Box::new(LinkedList::Cons(
                        Value::List(Box::new(LinkedList::Cons(
                            Value::Int(2),
                            Box::new(LinkedList::Cons(Value::Int(3), Box::new(LinkedList::Empty)))
                        ))),
                        Box::new(LinkedList::Cons(
                            Value::Int(4),
                            Box::new(LinkedList::Cons(
                                Value::List(Box::new(LinkedList::Cons(
                                    Value::Int(5),
                                    Box::new(LinkedList::Cons(
                                        Value::List(Box::new(LinkedList::Cons(
                                            Value::Int(6),
                                            Box::new(LinkedList::Empty)
                                        ))),
                                        Box::new(LinkedList::Cons(
                                            Value::Int(7),
                                            Box::new(LinkedList::Empty)
                                        ))
                                    ))
                                ))),
                                Box::new(LinkedList::Empty)
                            ))
                        ))
                    ))
                ))
            )
        );
    }

    #[test]
    fn test_partial_ord() {
        let list1 = LinkedList::from_string("[]");
        let list2 = LinkedList::from_string("[]");
        assert!(list1 == list2);

        let list1 = LinkedList::from_string("[]");
        let list2 = LinkedList::from_string("[1]");
        assert!(list1 < list2);

        let list1 = LinkedList::from_string("[1]");
        let list2 = LinkedList::from_string("[1]");
        assert!(list1 == list2);

        let list1 = LinkedList::from_string("[1]");
        let list2 = LinkedList::from_string("[3]");
        assert!(list1 < list2);

        let list1 = LinkedList::from_string("[1]");
        let list2 = LinkedList::from_string("[[1]]");
        assert_eq!(list1.partial_cmp(&list2), Some(Ordering::Equal));

        let list1 = LinkedList::from_string("[[1],[2,3,4]]");
        let list2 = LinkedList::from_string("[[1],4]");
        assert!(list1 < list2);
    }

    #[test]
    fn test_partial_ord_with_tricky_case() {
        let list1 = LinkedList::from_string("[[10]]");
        let list2 = LinkedList::from_string("[5]");
        assert!(list1 > list2);
    }
}

pub fn run(input: &str) {
    let mut sum_of_ordered_indexes = 0;

    for (pair_index, pair) in input.split("\n\n").enumerate() {
        let pair_index = pair_index + 1;

        let (left, right) = pair.split_once("\n").unwrap();
        let left = LinkedList::from_string(left);
        let right = LinkedList::from_string(right);

        if left < right {
            sum_of_ordered_indexes += pair_index;
            println!("Pair {pair_index} is ordered");
        } else if left > right {
            println!("Pair {pair_index} is not ordered");
        } else {
            println!("Pair {pair_index} is equal");
        }
    }

    println!("Sum of ordered indexs is {sum_of_ordered_indexes}");
}
