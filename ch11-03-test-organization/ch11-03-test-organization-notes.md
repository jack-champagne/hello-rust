# Unit and Integration Test

There are two different types of testing in general. There are Unit tests and Integration tests:

* Unit tests - Test small pieces of code isolated from other modules
* Integration tests - Test how a library user would interact with the library, multiple modules and the sort.

This terminology is not universal for all languages and communities.

## Unit tests

Unit tests are written inside the file with the code they are testing, right in the src dir.
They are annotated with \#\[cfg(test)\] as they should not be compiled with the release or normal run.

When we created a new library earlier with ```cargo new --lib \[libname\]```, the pre-generated code at the top
of the file was a unit test (annotation and all).

We *can* test private functions in unit tests as we scope them in with ```rust use super::*;```, and tests is
just another module and the same file.

## Integration tests

For integration tests, it is entirely external. Because of that, they also do not need the annotation at the top
of their module declaration. Making a new integration test consists of a few steps:

1. Create tests dir next to src dir (if not there already)
2. Inside test file, scope in the module(s) to test and define tests after test annotation.
3. Note that each integration test file will be treated as its own module

When running ```cargo test``` now we find there will be another section for the integration tests. Each integration test will display in its own section in the test output, we can run them individually by doing:

```
cargo test --test integration-test1
```

If we have a setup that was shared between different integration tests, we might create tests/common.rs. This
however, when we run cargo test, will be treated as a test (even if there are no tests inside) on the output.
Instead create tests/common/mod.rs for rust to understand this is not a test but shared code between tests. The
scoping it still exactly the same.

Integration tests for binary crates (ones without a lib.rs file) do not exist, they do not make sense! Since
it is a binary crate, code inside will not be able to be scoped by external code, hence no integration testing,
just unit testing.

