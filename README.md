# geohash

A geohash library to get some usefull data

#### Usage

use library to get:

    1. Distance between two geo-located points in Km, Miles and Nautic miles:
        - pass _lat, lon, lat, lon_ and _unit_ parameters to function.
        
    2. Minimum geohash precision for a given geoBound array and the number of squares you want to divide it.
        - pass an array like _[ top_left => [ lat => x, lon => y ], bottom_right => [ lat => x, lon => y ]_.
        - pass the number of squares, it has to be multiple of two.
        - Function will return the minimal geohash precision (from 1 to 12) who can contain every square.
        