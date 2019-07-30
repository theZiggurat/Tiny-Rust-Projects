SimpleCSVSorter by Max Davatelis

Synopsis:
	cat [INPUT] | cargo run --release -- [-d] [-c [COL_NAME]] > OUTPUT.csv
	
	[INPUT]: name of file used as CSV input (MUST be in csv file format)
	[-c COL_NAME]: column name to be sorted on
    [-d]: flag that enables descending order
	[OUTPUT]: name out file to dump sorted CSV to
