# Requirements:
# - Python version late enough to support argparse (2.7+ or 3.2+)
# - Pillow 2.7+
#   - Windows note: may need to install using easy_install instead of pip
#   - Linux note: may need to install the libraries for any image
#     format(s) you'll use, such as libpng and zlib for PNG

import argparse

from emblem import emblem_maker

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

    emblem_maker(args)
