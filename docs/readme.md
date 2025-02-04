# cmprs: Multithreaded Image Compression

`cmprs` ("compress") is a program to reduce the size of JPEG and PNG image files. Many phones, cameras, and image editing software save in an uncompressed format which can quickly consume space. Compressing these files makes it easier to store them locally and transfer them over the Internet.

`cmprs` uses multithreading to optimize files in parallel and a work-stealing queue to distribute uneven workloads (such as compressing large and small files together). This allows it to scale efficiently with the hardware of your computer.

Nonessential metadata is also removed to further reduce size:

- EXIF data in JPEG files is removed. `mozjpeg` does not preserve EXIF information during the compression process.
  - Because the `Orientation` value can affect display, input images are first transformed according to this value before compression.
- The `--png-strip` option offers different levels of removal for nonessential PNG data chunks. See the help output below for more.

## Compression Libraries

- **JPEG:** [mozjpeg](https://crates.io/crates/mozjpeg)
- **PNG:** [oxipng](https://github.com/shssoichiro/oxipng)

## Command-Line Options

```text
$ cmprs --help
Multithreaded image compressor

Usage: cmprs [OPTIONS] <PATHS>...

Arguments:
  <PATHS>...
          Set of input files to compress

Options:
  -j, --jobs <JOBS>
          Maximum number of threads to use

  -o, --output-format <OUTPUT_FORMAT>
          Format of output file names

          Format specifiers:
          - %e: File extension without leading dot
          - %s: File stem (file name before last dot)
          - %%: The '%' character

          [default: compressed_%s.%e]

      --overwrite
          Overwrite input files with compressed outputs (short for --output-format %s.%e)

      --jpg-quality <JPG_QUALITY>
          Quality of JPEG files (1-100; 60-80 recommended)

          [default: 75]

      --png-strip <PNG_STRIP>
          Strip nonessential PNG chunks

          [default: safe]

          Possible values:
          - none: Don't strip headers
          - safe: Strip headers that don't affect rendering
          - all:  Strip all non-critical headers

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```

## Usage

`cmprs` accepts either a single file (as in `20240422_193142.jpg`) or a glob pattern referring to a set of files (as in `camera/*.jpg`). If you want to compress all files inside a directory, you must pass a glob pattern to `cmprs` which points to the files inside the directory. For example, run `cmprs dir/*` rather than `cmprs dir/`.

By default, `cmprs` attempts to create the maximum number of threads your CPU supports (typically number of cores * 2). The `-j` option allows you to change the number of threads it runs.

> [!IMPORTANT]
> The actual number of threads that `cmprs` creates will never be larger than the number of input files it receives.

When `cmprs` operates on files, it does not overwrite them by default. Instead, compressed files have a `compressed_` prefix. To overwrite the original files, use the `--overwrite` switch. To set a custom output file name, use the `-o`/`--output-format` option. This option accepts format specifiers listed in the help text above.

> [!WARNING]
> Overwriting input files may leave you unable to access them. Back up important data.

The `--jpg-quality` option allows you to control the quality of compressed JPEG files. Lower quality corresponds to higher compression levels, which can save additional space but might use extra processing time. In addition, since JPEG is a lossy format, more prominent artifacting may be visible with greater compression. The default value is a good choice for most files.

The `--png-strip` option allows you to control the removal of nonessential metadata contained within PNG files, such as dates and text blobs. The `safe` option removes all chunks that are not involved in rendering, while the `all` option is more extensive, preserving only the image data itself. Usage of `all` may cause the removal of certain chunks that affect how the image is processed, such as `gAMA` (gamma correction) and `pHYs` (aspect ratio), depending on if they were present in the input file.

## Example Command and Output

```text
$ cmprs photos/* --overwrite
Compression with up to 16 threads.
photos/20240421_141115.jpg: saved 1.56 MB (82.34%)
photos/20240414_235123.jpg: saved 2.38 MB (80.32%)
photos/20240420_170206.jpg: saved 2.13 MB (74.84%)
photos/20240422_192926.jpg: saved 2.86 MB (86.13%)
photos/20240414_235127.jpg: saved 2.66 MB (80.18%)
photos/20240418_215533.jpg: saved 2.98 MB (80.50%)
photos/20240416_211258.jpg: saved 2.95 MB (79.65%)
photos/20240418_215007.jpg: saved 3.17 MB (79.07%)
photos/20240422_192922.jpg: saved 3.55 MB (79.71%)
photos/20240418_215458.jpg: saved 3.28 MB (77.86%)
photos/20240416_210401.jpg: saved 3.20 MB (75.77%)
photos/20240416_210439.jpg: saved 2.70 MB (71.24%)
photos/20240416_210420.jpg: saved 2.64 MB (73.16%)
photos/20240413_192412.jpg: saved 3.94 MB (76.18%)
photos/20240420_170221.jpg: saved 3.08 MB (63.55%)
photos/20240421_141049.jpg: saved 2.66 MB (85.08%)
photos/20240421_141129.jpg: saved 2.09 MB (74.53%)
photos/20240421_141136.jpg: saved 3.53 MB (75.92%)
photos/20240421_141034.jpg: saved 2.78 MB (64.06%)
photos/20240421_141039.jpg: saved 3.08 MB (63.79%)
photos/20240416_210635.jpg: saved 8.45 MB (76.00%)
Total time: 5.367695s, saved 65.65 MB (75.54%)
```

## Implementation Details

The compressor code is designed to be as generic as possible, and file-specific compression logic is limited to a single function (one function per file type):

```rust
pub fn compress(data: Vec<u8>, args: &Args) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    // ...
}
```

This takes the contents of the original file as a `Vec<u8>` and the command-line arguments, and returns the compressed contents as a `Vec<u8>`. It may also throw an error if needed. All other functionality, such as determining whether to write to a file, task distribution, and globbing are handled outside this function, in the core code.

This makes it easy to extend `cmprs` to support additional file types.

## See Also

- [oxipng](https://github.com/shssoichiro/oxipng): Provides a multithreaded CLI tool to compress PNG files.
- [pngstrip](https://github.com/AidanSun05/pngstrip): Strips all non-critical PNG chunks; serves as a simple example of processing PNG files in C++ without dependencies. Does not actually compress files.
