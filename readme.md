# Convert to SVG Script

Convert Image (JPG, PNG, WEBP) to SVG Script

**Command**
```
convert-to-svg -i <filename.xxx>
```

**Example**
```
convert-to-svg -i xxx.jpg \
    -o yyy.svg \
    --color-mode color
    --color-precision 255 \
    --path-precision 20 \
    --filter-speckle 0 \
    --corner-threshold 0 \
    --layer-difference 1 \
    --length-threshold 0.0 \
    --splice-threshold 0 \
    --max-iterations 100 \
    --mode none
```

## Parameters
| Parameter            | Short | Default                | Description                                                        |
| -------------------- | ----- | ---------------------- | ------------------------------------------------------------------ |
| `--input`            | `-i`  |                        | Path to the file to convert (supports jpg, png, webp)              |
| `--output`           | `-o`  | `<input filename>.svg` | Output file path                                                   |
| `--color-mode`       |       | `color`                | Output color mode (color = color, binary = black and white)        |
| `--color-precision`  |       | `8`                    | Color separation - higher value increases colors (0-255)           |
| `--path-precision`   |       | `10`                   | Path detail - higher value increases detail (1-20)                 |
| `--filter-speckle`   |       | `1`                    | Filter tiny specks - lower value increases detail (0-10)           |
| `--corner-threshold` |       | `20`                   | Preserve sharp corners - lower value increases sharpness (0-180)   |
| `--layer-difference` |       | `5`                    | More color layers - lower value increases layers (1-255)           |
| `--length-threshold` |       | `1.0`                  | Preserve all short lines - lower value increases detail (0.0-10.0) |
| `--splice-threshold` |       | `20`                   | Preserve connection points - lower value increases detail (0-180)  |
| `--max-iterations`   |       | `20`                   | Maximum refinement - higher value increases quality (1-100)        |
| `--mode`             |       | `none`                 | Complexity mode (none, polygon, spline)                            |

