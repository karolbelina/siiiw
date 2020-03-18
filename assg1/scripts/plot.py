import sys
import csv
import matplotlib as mpl
import matplotlib.pyplot as plt

mpl.use('pgf')
mpl.rcParams.update({
    'pgf.texsystem': 'pdflatex',
    'font.family': 'serif',
    'text.usetex': True,
    'pgf.rcfonts': False,
})
mpl.rcParams['lines.linewidth'] = 1
mpl.rcParams['axes.linewidth'] = 0.4

with open(sys.argv[1], 'r') as file:
    reader = csv.reader(file, delimiter = ';')
    bests, avgs, worsts = zip(*map(lambda row: map(float, row[1:]), reader))

    fig = plt.figure()
    plt.axis([0, len(bests) - 1, int(sys.argv[3]), int(sys.argv[4])])
    plt.plot(worsts, color='b')
    plt.plot(avgs, color='dimgray')
    plt.plot(bests, color='g')
    plt.xlabel('Nr pokolenia')
    plt.ylabel('Wartość funkcji przystosowania')
    plt.legend(['worst', 'avg', 'best'], loc='upper right')
    plt.subplots_adjust(left=0.15, right=0.85, top=0.98, bottom=0.15)
    fig.set_size_inches(w=5.39749, h=3)
    fig.savefig(sys.argv[2])