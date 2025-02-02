#!/usr/bin/env python3
import sys
from PIL import Image

def downscale_image(input_path, output_path):
    try:
        with Image.open(input_path) as img:
            # Convert image to RGB (in case it's in a different mode)
            if img.mode != 'RGB':
                img = img.convert('RGB')

            # Check if the image is square; if not, crop the center square.
            if img.width != img.height:
                print("Warning: Input image is not square. Cropping to square...")
                side = min(img.width, img.height)
                left = (img.width - side) // 2
                top = (img.height - side) // 2
                right = left + side
                bottom = top + side
                img = img.crop((left, top, right, bottom))

            # Resize image to 128x128 pixels using high-quality downsampling.
            img = img.resize((128, 128), Image.LANCZOS)
            
            # Save the downscaled image as JPEG.
            img.save(output_path, "JPEG")
            print(f"Downscaled image saved to {output_path}")
    except Exception as e:
        print("Error processing image:", e)
        sys.exit(1)

if __name__ == "__main__":
    if len(sys.argv) != 3:
        print("Usage: python downscale.py input.jpg output.jpg")
        sys.exit(1)

    input_file = sys.argv[1]
    output_file = sys.argv[2]
    downscale_image(input_file, output_file)
