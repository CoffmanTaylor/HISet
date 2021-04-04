# HI Set
`HISet` is a Hashable and Indexable Set. The hash code and index order will be the same regardless of the order of operations applied to the set.

This library also provides a `HIBag` that is a Hashable and Indexable Bag, meaning there can be duplicates. The exact order of equal elements is arbitrary.

## Design
Under the hood, both are just defined as sorted vectors.