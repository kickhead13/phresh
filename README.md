## phresh
### Description

'phresh' is an interporetor for the photo editing programming language by the same name.

### Installation and Usage

Clone this repository:
```
$ git clone 'https://github.com/kickhead13/phresh.git'
```

Compile the interpretor and run example to see what happens :)
```
$ cd phresh 
$ cargo build --release
$ ./target/release/phresh ./examples/open_and_save_image.phresh
```

Have fun editing :P 

### Some fun examples 

<p align="center">
    <img src="/assets/pfp.jpg">
</p>

#### Fuzzy 

```
img image1 'assets/pfp.jpg'
img image2 'assets/pfp.jpg'
layer image1 image2 0 -10 33
layer image1 image2 0 10 33
save image1 'assets/fuzz.jpg' jpg
```

<p align="center">
    <img src="/assets/fuzz.jpg">
</p>


#### Inception

```
img image1 'assets/pfp.jpg'
downscale image1 image1 0 0 100 2
downscale image1 image1 0 0 100 4
downscale image1 image1 0 0 100 8
save image1 'assets/inception.jpg' jpg
```

<p align="center">
    <img src="/assets/inception.jpg">
</p>

#### Download from the web

```
img download 'https://onlinedi.vision/cdn/test/stest.jpeg'
save download 'assets/download.jpg' jpg
```

<p align="center">
    <img src="/assets/download.jpg">
</p>


