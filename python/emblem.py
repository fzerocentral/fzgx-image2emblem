# Requirements:
# - Python version late enough to support argparse (2.7+ or 3.2+)
# - Pillow 2.7+
#   - Windows note: may need to install using easy_install instead of pip
#   - Linux note: may need to install the libraries for any image
#     format(s) you'll use, such as libpng and zlib for PNG

import datetime
import math
import os
import struct
from PIL import Image

YEAR_2000 = datetime.datetime(2000, 1, 1)

# Find this script's directory, then navigate from there to the common
# directory, which has some files we need to read.
# This way of specifying the directory is better than '../common' since the
# script's directory doesn't have to be the current working directory.
# http://stackoverflow.com/a/9350788/
script_dir = os.path.dirname(os.path.realpath(__file__))
common_dir = os.path.join(script_dir, os.pardir, 'common')


def short_filename(filename, seconds_since_start_of_2000):
    if not filename:
       return "fze0200002000{:14X}.dat".format(int(seconds_since_start_of_2000 * 40500000))

    return "fze1-" + filename + ".dat"


def full_filename(filename):
    return "8P-GFZE-" + filename + ".gci"


def checksum(post_checksum_bytes):
    checksum = 0xFFFF
    generator_polynomial = 0x8408

    for byte_as_number in post_checksum_bytes:
        checksum = checksum ^ byte_as_number

        for i in xrange(8):
            if checksum & 1 == 1:
                checksum = (checksum >> 1) ^ generator_polynomial
            else:
                checksum = checksum >> 1

    # Flip all the bits
    checksum = checksum ^ 0xFFFF

    return bytearray(struct.pack(">H", checksum))

def setup_header_bytes(emblem_short_filename, seconds_since_start_of_2000):
    header_bytes = bytearray()

    # Constant bytes
    header_bytes += bytearray("GFZE8P")
    header_bytes += bytearray([0xFF, 2])

    # Short filename followed by 0 padding until 32 bytes
    header_bytes += bytearray(emblem_short_filename)
    header_bytes += bytearray(32 - len(emblem_short_filename))

    # Timestamp
    header_bytes += bytearray(
        struct.pack(">I", int(seconds_since_start_of_2000)))

    # Constant bytes
    header_bytes += bytearray([0, 0, 0, 0x60, 0, 2, 0, 3, 4])

    # Copy count (1 byte)
    header_bytes += bytearray([0])

    # Start block (2 bytes)
    #
    # TODO: Check if there is a better value to use here besides 0.
    # Want to avoid the following error when we try to delete the file from a
    # memcard in Dolphin: "Order of files in the File Directory do not match
    # the block order[.] Right click and export all of the saves, and import
    # the saves to a new memcard"
    header_bytes += bytearray(struct.pack(">H", 0))
    # Constant bytes
    header_bytes += bytearray([0, 3, 0xFF, 0xFF, 0, 0, 0, 4])

    return header_bytes

def setup_more_info_bytes(now, additional_comment):
    more_info_bytes = bytearray()
    # Constant bytes
    more_info_bytes += bytearray([4, 1])
    # Game title followed by 0 padding until 32 bytes
    more_info_bytes += bytearray("F-ZERO GX")
    more_info_bytes += bytearray(32 - len("F-ZERO GX"))
    # File comment followed by 0 padding until 60 bytes
    comment_str = now.strftime("%y/%m/%d %H:%M")

    if additional_comment:
        comment_str += " (Created using third party code)"

    more_info_bytes += bytearray(comment_str)
    more_info_bytes += bytearray(60 - len(comment_str))

    return more_info_bytes


def crop_square(img):
    """Crop to a square.

    img.size gives a tuple of (width, height).
    Left is inclusive, right is not inclusive; same for upper and lower.
    """
    width, height = img.size
    min_dimension = min(width, height)
    crop_left = width - min_dimension
    crop_right = crop_left + min_dimension
    crop_upper = height - min_dimension
    crop_lower = crop_upper + min_dimension
    img = img.crop((crop_left, crop_upper, crop_right, crop_lower))

def emblem(img64_data, alpha_threshold):
    """Emblem (64x64)

    # Go through the pixels in 4x4 blocks, left to right and top to
    # bottom. This is the order that the emblem data must be stored in.
    """
    emblem_pixel_bytes = bytearray()

    for block_row in xrange(16):
        for block_col in xrange(16):
            for pixel_row in xrange(4):
                # Get the corresponding pixels in the 64x64 emblem, which just
                # goes row by row.
                first_i = block_row*64*4 + pixel_row*64 + block_col*4
                pixel_data = [img64_data[i] for i in range(first_i, first_i+4)]
                for rgba in pixel_data:
                    if rgba[3] >= alpha_threshold:
                        red = int(math.floor(rgba[0] / 8.0))
                        green = int(math.floor(rgba[1] / 8.0))
                        blue = int(math.floor(rgba[2] / 8.0))
                        alpha = 1
                        value = 32768*alpha + 1024*red + 32*green + blue
                    else:
                        value = 0
                    emblem_pixel_bytes += bytearray(struct.pack(">H", value))
    return emblem_pixel_bytes

def banner(img32, alpha_threshold):
    """Banner (96x32)

    emblem_banner_base is a pre-existing file that contains the left 2/3rds
    of an F-Zero GX emblem file's banner, in the same pixel format as any
    emblem file. (The left 2/3rds of the banner are the same for
    every emblem.)
    """
    banner_base_file = open(
        os.path.join(common_dir, 'emblem_banner_base'), 'rb')
    banner_bytes = bytearray()
    img32_data = img32.getdata()

    # We now have the banner with blank pixels in the emblem preview. Now
    # we'll fill in that emblem preview.
    for block_row in xrange(8):
        banner_bytes += banner_base_file.read(0x200)
        for block_col in xrange(8):
            for pixel_row in xrange(4):
                # Get the corresponding pixels in the 32x32 emblem version.
                first_i = block_row*32*4 + pixel_row*32 + block_col*4
                pixel_data = [img32_data[i] for i in range(first_i, first_i+4)]
                for rgba in pixel_data:
                    if rgba[3] >= alpha_threshold:
                        red = int(math.floor(rgba[0] / 8.0))
                        green = int(math.floor(rgba[1] / 8.0))
                        blue = int(math.floor(rgba[2] / 8.0))
                        alpha = 1
                        value = 32768*alpha + 1024*red + 32*green + blue
                    else:
                        value = 0
                    banner_bytes += bytearray(struct.pack(">H", value))
    return banner_bytes


def icon():
    """Icon (32x32)

    emblem_icon is a pre-existing file that contains an F-Zero GX
    emblem file's icon, in the same pixel format as any emblem file.
    (The icon is the same for every emblem.)
    """
    return open(os.path.join(common_dir, 'emblem_icon'), 'rb').read()

def edge_options(img, edge_option):
    # Image.LANCZOS constant requires Pillow 2.7 or higher.
    if edge_option == 'resize62':
        # Resize to 62x62, then paste into the middle of an empty 64x64 image.
        img62 = img.resize((62,62), Image.LANCZOS)
        img64 = Image.new("RGBA", (64,64), (0,0,0,0))
        img64.paste(img62, box=(1,1))
    elif edge_option == 'crop':
        # Resize to 64x64 and replace the edges with empty pixels.
        img64 = img.resize((64,64), Image.LANCZOS)
        for i in xrange(64):
            img64.putpixel((0,i), (0,0,0,0))
            img64.putpixel((63,i), (0,0,0,0))
            img64.putpixel((i,0), (0,0,0,0))
            img64.putpixel((i,63), (0,0,0,0))
    elif edge_option == 'resize64':
        # Resize to 64x64.
        img64 = img.resize((64,64), Image.LANCZOS)

    return img64;

def seconds_since_2000(now):
    return (now - YEAR_2000).total_seconds()

def image(image_filename):
    img = Image.open(image_filename).convert(mode="RGBA")
    crop_square(img)

    return img

def emblem_maker(args):
    now = datetime.datetime.now()
    seconds_since_start_of_2000 = seconds_since_2000(now)
    alpha_threshold = args.alpha_threshold
    icon_bytes = icon()
    emblem_short_filename = short_filename(args.emblem_filename, seconds_since_start_of_2000)

    emblem_full_filename = full_filename(emblem_short_filename)

    header_bytes = setup_header_bytes(emblem_short_filename, seconds_since_start_of_2000)
    more_info_bytes = setup_more_info_bytes(now, args.additional_comment)

    img = image(args.image_filename)

    # TODO: Test non-RGBA stuff going through crop or resize64.
    # (That, or know when to tell the user to resize/convert themselves...)
    img64 = edge_options(img, args.edge_option)

    # TODO: Check how the 64 to 32 resize is done by the game. Not a
    # big deal though, it just means the banner may look slightly different
    # than it should in a memcard manager.
    img32 = img.resize((32,32), Image.LANCZOS)
    img64_data = img64.getdata()

    emblem_pixel_bytes = emblem(img64_data, alpha_threshold)
    banner_bytes = banner(img32, alpha_threshold)

    # A bunch of zeros until the end of 3 Gamecube memory blocks
    end_padding_bytes = bytearray(0x6040 - 0x40A0)

    post_checksum_bytes = more_info_bytes + banner_bytes \
      + icon_bytes + emblem_pixel_bytes + end_padding_bytes

    checksum_bytes = checksum(post_checksum_bytes)

    emblem_file = open(emblem_full_filename, 'wb')
    emblem_file.write(header_bytes + checksum_bytes + post_checksum_bytes)
    emblem_file.close()
