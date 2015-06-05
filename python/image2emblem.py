# Requirements:
# - Python version late enough to support argparse (2.7+ or 3.2+)
# - Pillow 2.7+
#   - Windows note: may need to install using easy_install instead of pip
#   - Linux note: may need to install the libraries for any image
#     format(s) you'll use, such as libpng and zlib for PNG

import argparse
import datetime
import math
import struct
from PIL import Image

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
    print emblem_short_filename
    print seconds_since_start_of_2000
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

if __name__ == '__main__':

    # Parse command line arguments.
    arg_parser = argparse.ArgumentParser(
        description=""
    )
    arg_parser.add_argument(
        'image_filename',
        type=str,
        help=(
            "Filename of the image file."
        ),
    )
    arg_parser.add_argument(
        '--emblem-filename',
        dest='emblem_filename',
        type=str,
        help=(
            "Specify a custom emblem filename to put in place"
            " of the default timestamp."
        ),
    )
    arg_parser.add_argument(
        '--edge-option',
        dest='edge_option',
        type=str,
        default='resize62',
        help=(
            'Specify what to do about the edges of the 64x64 emblem:'
            ' "resize62" (default, resize to 62x62 and add empty edges),'
            ' "crop" (resize to 64x64 and replace the edges with empty pixels),'
            ' or "resize64" (resize to 64x64; having non-empty edges will make'
            ' the emblem edges stretch out to cover the entire machine face).'
        ),
    )
    arg_parser.add_argument(
        '--alpha-threshold',
        dest='alpha_threshold',
        type=int,
        default=1,
        help=(
            'Minimum alpha that will be accepted as a non-blank pixel.'
            ' Acceptable range is 1 to 255. Default is 1.'
        ),
    )
    arg_parser.add_argument(
        '--additional-comment',
        dest='additional_comment',
        action='store_true',
        help=(
            "Make the comment field include a note saying it was"
            " created using third party code. (To distinguish from emblems"
            " created in-game.)"
        ),
    )
    args = arg_parser.parse_args()
    
    
    now = datetime.datetime.now()
    start_of_2000 = datetime.datetime(2000, 1, 1)
    seconds_since_start_of_2000 = (now - start_of_2000).total_seconds()
    
    
    if args.emblem_filename:
        if len(args.emblem_filename) > 18:
            raise ValueError("emblem-filename should be 18 characters or less.")
        emblem_short_filename = "fze1-" + args.emblem_filename + ".dat"
    else:
        emblem_short_filename = "fze0200002000{:14X}.dat".format(
            int(seconds_since_start_of_2000 * 40500000)
        )
    emblem_full_filename = "8P-GFZE-" + emblem_short_filename + ".gci"
    
    
    more_info_bytes = bytearray()
    # Constant bytes
    more_info_bytes += bytearray([4, 1])
    # Game title followed by 0 padding until 32 bytes
    more_info_bytes += bytearray("F-ZERO GX")
    more_info_bytes += bytearray(32 - len("F-ZERO GX"))
    # File comment followed by 0 padding until 60 bytes
    comment_str = now.strftime("%y/%m/%d %H:%M")
    if args.additional_comment:
        comment_str += " (Created using third party code)"
    more_info_bytes += bytearray(comment_str)
    more_info_bytes += bytearray(60 - len(comment_str))
    
    
    header_bytes = setup_header_bytes(emblem_short_filename, seconds_since_start_of_2000)
    img = Image.open(args.image_filename)
    
    # Convert the image to RGBA.
    img = img.convert(mode="RGBA")
    
    # Crop to a square.
    # img.size gives a tuple of (width, height).
    # Left is inclusive, right is not inclusive; same for upper and lower.
    min_dimension = min(img.size[0], img.size[1])
    crop_left = img.size[0] - min_dimension
    crop_right = crop_left + min_dimension
    crop_upper = img.size[1] - min_dimension
    crop_lower = crop_upper + min_dimension
    img = img.crop((crop_left, crop_upper, crop_right, crop_lower))
    
    # TODO: Test non-RGBA stuff going through crop or resize64.
    # (That, or know when to tell the user to resize/convert themselves...)
    
    # Image.LANCZOS constant requires Pillow 2.7 or higher.
    if args.edge_option == 'resize62':
        # Resize to 62x62, then paste into the middle of an empty 64x64 image.
        img62 = img.resize((62,62), Image.LANCZOS)
        img64 = Image.new("RGBA", (64,64), (0,0,0,0))
        img64.paste(img62, box=(1,1))
    elif args.edge_option == 'crop':
        # Resize to 64x64 and replace the edges with empty pixels.
        img64 = img.resize((64,64), Image.LANCZOS)
        for i in xrange(64):
            img64.putpixel((0,i), (0,0,0,0))
            img64.putpixel((63,i), (0,0,0,0))
            img64.putpixel((i,0), (0,0,0,0))
            img64.putpixel((i,63), (0,0,0,0))
    elif args.edge_option == 'resize64':
        # Resize to 64x64.
        img64 = img.resize((64,64), Image.LANCZOS)
    else:
        raise ValueError("Invalid edge-option.")
        
    if args.alpha_threshold < 1 or args.alpha_threshold > 255:
        raise ValueError("Invalid alpha-threshold.")
    alpha_threshold = args.alpha_threshold
    
    # TODO: Check how the 64 to 32 resize is done by the game. Not a
    # big deal though, it just means the banner may look slightly different
    # than it should in a memcard manager.
    img32 = img.resize((32,32), Image.LANCZOS)
    
    emblem_pixel_bytes = bytearray()
    img64_data = img64.getdata()
    
    # Emblem (64x64)
    # Go through the pixels in 4x4 blocks, left to right and top to
    # bottom. This is the order that the emblem data must be stored in.
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
    
    
    # Banner (96x32)
    
    # emblem_banner_base is a pre-existing file that contains the left 2/3rds
    # of an F-Zero GX emblem file's banner, in the same pixel format as any
    # emblem file. (The left 2/3rds of the banner are the same for
    # every emblem.)
    banner_base_file = open("../common/emblem_banner_base", 'rb')
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
    
    
    # Icon (32x32)
    
    # emblem_icon is a pre-existing file that contains an F-Zero GX
    # emblem file's icon, in the same pixel format as any emblem file.
    # (The icon is the same for every emblem.)
    icon_file = open("../common/emblem_icon", 'rb')
    icon_bytes = icon_file.read()
    
    
    # A bunch of zeros until the end of 3 Gamecube memory blocks
    end_padding_bytes = bytearray(0x6040 - 0x40A0)
    
    post_checksum_bytes = more_info_bytes + banner_bytes \
      + icon_bytes + emblem_pixel_bytes + end_padding_bytes

    checksum_bytes = checksum(post_checksum_bytes)

    emblem_file = open(emblem_full_filename, 'wb')
    emblem_file.write(header_bytes + checksum_bytes + post_checksum_bytes)
    emblem_file.close()
