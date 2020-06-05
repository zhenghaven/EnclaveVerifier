#!/bin/bash

tests='is_prime scope_test test_bexps test_fcall_1 test_fcall_2 test_glvar_and_returnv test_ifel test_overloading'

for curr_test in $tests
do
    cargo run --bin=type_checker ${curr_test}
    if [ $? -ne 0 ]; then
      echo "Test ${curr_test} failed."
      exit 1
    fi
done

echo "All tests passed!"
