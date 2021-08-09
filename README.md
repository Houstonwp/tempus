# Tempus: Dates and Date Arithmetic for Rust

Tempus aims to provide an ergonomic API for dates and date artihmetic. It does not aim to implement time or time zones, but these could be extended in additional libraries.

Tempus's default implementation uses the proleptic Gregorian calendar with an epoch of 0000-03-01. It is assumed that *only* unsigned years are used. Further features to support negative years and arbitrary epochs will be added in the future.

Thanks, appreciation, and inspiration go to many people. Giants have come before me, and it would be impossible for me to do this without their willingness to put out information into the open source community:

*  [Chrono](https://github.com/chronotope/chrono) - the defacto standard Date library in Rust.

*  [Cassio Neri's Gregorian Calendar Algorithms](https://github.com/cassioneri/calendar) and [associated paper](https://arxiv.org/pdf/2102.06959.pdf) of which I aimed to implement his alogrithms in Rust.

*  [Howard Hinnant's Website](http://howardhinnant.github.io/date_algorithms.html#Yes,%20but%20how%20do%20you%20know%20this%20all%20really%20works?) which provided context on the underlying Date algorithms found in the C++20 standard library (which was admittedly modified from his original implementation).

* [Pacifico, Meridith, and Lakos's N3344](http://www.open-std.org/jtc1/sc22/wg21/docs/papers/2012/n3344.pdf) along with Bloomberg's [bdlt](https://bloomberg.github.io/bde-resources/doxygen/bde_api_prod/group__bdlt.html)

The Rust code in this library is pre-alpha. Given this is my first project and undertaking in Rust, it is likely inefficient (for now), incomplete (for now), and messy. Learning along the way and including documentation, error handling, and features as I'm able.