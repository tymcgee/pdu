# pdu, a disk-usage utility

This is `pdu`, a tool similar to `du` but with more precision and less features.

My first reason for making this was that I didn't like how difficult it was to get more than one decimal of precision in the human-readable output of `du`.

My second reason was boredom, and the desire to write something in rust.

# What does it do?
`pdu` iterates recursively through the directory you're in and reports back on how large the folders and files are in a human-readable format with three decimals of precision.
If it comes across a folder, it will only report summary of the size of everything contained within that folder.
The output is sorted by filesize, from lowest to highest.

# License
This project is licensed under MIT. See the `LICENSE` file for more details.
