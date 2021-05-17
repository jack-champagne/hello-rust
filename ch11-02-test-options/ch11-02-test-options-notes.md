# Testing options and Command line Options

Testing using cargo takes two sets of options specifiers. One for the binary and one for testing.
Delineated by --
```
cargo test --help
// gives options for testing in general 
cargo test -- --help
// Everything after -- gives options for binary testing executable compiled when we compile our library for testing
```

Passing tests will not display std output like failing test. To show
```
cargo test -- --show-output
```

If we have a test called my_test_fn inside the tests, we can run specifically that test by doing the following
```
cargo test my_test_fn
```

We can also specify substrings as well to filter out tests that don't have a matching substring in their name
```
cargo test add
// Will run any tests that have `add` in their name
```

Inside the testing file, we can ignore expensive or otherwise unimportant to constantly run tests by adding the attribute
\#\[ignore\]. A test with this attribute will not run unless the command line option below is specified
```
cargo test -- --ignored
```

That way when you have time to wait around for an hour for a test to complete, you can run it like so.