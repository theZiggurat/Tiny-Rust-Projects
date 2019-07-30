csv_sorter crate by Max Davatelis

Synopsis:

	cargo run --release -- [-f FILE_NAME] [-c COL_NAME] [-d]
	
Args:

	[-f FILE_NAME]: name of csv file to be sorted
	[-c COL_NAME]: column name to be sorted on
    [-d]: flag that enables descending order

This is a rust rewrite of an old university project written in C [(linked here)](https://github.com/theZiggurat/Tiny-C-Projects/tree/master/csv_sorter). It is a csv sorter that sorts
in place on the input file. Unlike the C version, it was trivial to implement 
multithreading for the sorting. All it took was importing the *rayon* crate and changing **rows.sort_by()** 
to **rows.par_sort_by()** and that was it! The code is also vastly shorter and less complicated, 
thanks to all the functional features of Rust. This is a trend I expect to continue as I
port the bank server/client and digital logic calculator.