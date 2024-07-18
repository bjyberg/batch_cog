A simple little function to write cloud-optimized geotiffs (COGs) from other raster types (geotiff, asc, etc.) with rust.
It takes a folder path as an input, runs in parallel,and seems to be quicker than a similar R function (as expected).
This was done partly to optimize some workflows but also as a way for me to learn some basic rust programming.
This is my first attempt at rust so if anyone comes across this and has more of an idea of what
they are doing, I'm super open to feedback.