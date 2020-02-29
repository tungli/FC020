import matplotlib.pyplot as plt
import numpy as np
from itertools import zip_longest

def read_nalgebra_matrix(matrix_file):
    with open(matrix_file) as f:
        s = f.read()

    values = []
    for line in s.split('\n'):
        values.append([])
        for word in line.split():
            try:
                val = float(word)
                values[-1].append(val)
            except ValueError:
                pass
    values = list(filter(lambda x: x != [], values))

    return np.array(values)

def anal(x, y):
    return np.cos(2*x)*np.exp(-2*y)

def plot_slice(ii, where): 
    plt.figure()
    styles = ['k:', 'k-.', 'k--']
    for i, s in zip_longest(ii, styles, fillvalue='-'):
        x = np.linspace(0, 1, i)
        f = "solution_size_{}.mat".format(i)
        m = read_nalgebra_matrix(f)
        a = int(where*i)
        da = where*i - a 
        plt.plot(x, da*m[:,a + 1] + (1-da)*m[:,a + 1], s, label=str(i))
        plt.xlabel("X")
        plt.ylabel("u")
        plt.title("u(x, {})".format(where))
    plt.plot(x, anal(x, where), 'k', label="analytical")
    plt.legend()
    plt.savefig("slice_plot_x_{}.png".format(where))

    plt.clf()

    plt.figure()
    for i, s in zip_longest(ii, styles, fillvalue='-'):
        x = np.linspace(0, 1, i)
        f = "solution_size_{}.mat".format(i)
        m = read_nalgebra_matrix(f)
        a = int(where*i)
        da = where*i - a 
        plt.plot(x, da*m[a + 1, :] + (1-da)*m[a + 1, :], s, label=str(i))
        plt.xlabel("Y")
        plt.ylabel("u")
        plt.title("u({}, y)".format(where))
    plt.plot(x, anal(where, x), 'k', label="analytical")
    plt.legend()
    plt.savefig("slice_plot_y_{}.png".format(where))


ii = [50, 100, 200]
points = [0.2, 0.5, 0.8]

for i in points:
    plot_slice(ii, i)



