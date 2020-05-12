import argparse

parser = argparse.ArgumentParser(description='Converts a bunch of Wikipedia articles into a single ARFF file.')
parser.add_argument('input', help='Input directory')
parser.add_argument('output', help='Output file')
args = parser.parse_args()

import os, unicodedata, unidecode

print('Reading files... ', end='', flush=True)
root, _, filenames = next(os.walk(args.input, topdown=False))
print('done')

with open(os.path.join(os.getcwd(), args.output), 'wb') as output_file:
    article_classes = {filename.split('_')[0] for filename in filenames}

    print('Writing the header section... ', end='', flush=True)
    output_file.write(b'@RELATION articles\n\n')
    output_file.write(b'@ATTRIBUTE article  STRING\n')
    output_file.write('@ATTRIBUTE class    {{{}}}\n\n'.format(','.join(sorted(article_classes))).encode())
    print('done')

    print('Writing the data section... ', end='', flush=True)
    output_file.write(b'@DATA\n')

    for filename in filenames:
        with open(os.path.join(os.getcwd(), root, filename), 'r') as file:
            article_content = unidecode.unidecode(''.join(file.readlines())).encode() \
                .replace(b'\n', b' ') \
                .replace(b"'", b"\\'") \
                .replace(b'&nbsp;', b' ') \
                .strip()
            article_class = filename.split('_')[0].encode()
            output_file.write(b"'%b', %b\n" % (article_content, article_class))
    
    print('done')
