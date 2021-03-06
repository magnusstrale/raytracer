# The Ray Tracer in Rust

This repo contains the code that I've written by following 
[The Ray Tracer Challenge book](https://www.amazon.com/Ray-Tracer-Challenge-Test-Driven-Renderer/dp/1680502719). 
It is intended for my personal development, but if you find it useful - please let me know.

You can also read the annonucement blog post here https://dev.to/magnusstrale/the-ray-tracer-challenge-in-rust-58ej

Running the code will produce a number of .png-files from the various stages of the book. Some
files will take time to produce since they are rendered in higher resolution. I e don't
assume that the program hangs when you start it and nothing happens. Hang on for a minute
and everything will be fine.

## How to run the code

- Download the code
- run 'cargo test raytracer' to run the tests
- run 'cargo run raytracer' to get some sample files

## What is the three_spheres_acne.png file?

This is a sample file that was produced when some of the finer points in ray tracing showed up - 
the concept of 'self shadowing'. I decided to keep it around and since the code no longer can 
produce a file like that it ended up in the repo.

