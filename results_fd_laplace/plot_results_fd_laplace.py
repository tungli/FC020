import matplotlib.pyplot as plt
import numpy as np

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

plt.figure()
for i in [50, 100, 200]:
    f = "solution_size_{}.mat".format(i)
    m = read_nalgebra_matrix(f)
    plt.plot(np.linspace(0, 1, i), m[:,int(i/2)], label=str(i))
    plt.xlabel("X")
    plt.ylabel("u")
    plt.title("u(x, 0.5)")
plt.legend()
plt.savefig("middle_plot_x.png")

plt.clf()

for i in [50, 100, 200]:
    f = "solution_size_{}.mat".format(i)
    m = read_nalgebra_matrix(f)
    plt.plot(np.linspace(0, 1, i), m[int(i/2),:], label=str(i))
    plt.xlabel("Y")
    plt.ylabel("u")
    plt.title("u(0.5, y)")
plt.legend()
plt.savefig("middle_plot_y.png")

plt.clf()
    
for i in [50, 100, 200]:
    f = "solution_size_{}.mat".format(i)
    m = read_nalgebra_matrix(f)
    plt.plot(np.linspace(0, 1, i), m[:,int(i/5)], label=str(i))
    plt.xlabel("X")
    plt.ylabel("u")
    plt.title("u(x, 0.2)")
plt.legend()
plt.savefig("close_end_x.png")

plt.clf()

for i in [50, 100, 200]:
    f = "solution_size_{}.mat".format(i)
    m = read_nalgebra_matrix(f)
    plt.plot(np.linspace(0, 1, i), m[int(i/5),:], label=str(i))
    plt.xlabel("Y")
    plt.ylabel("u")
    plt.title("u(0.2, y)")
plt.legend()
plt.savefig("close_end_y.png")

plt.clf()

for i in [50, 100, 200]:
    f = "solution_size_{}.mat".format(i)
    m = read_nalgebra_matrix(f)
    plt.plot(np.linspace(0, 1, i), m[:,int(3*i/4)], label=str(i))
    plt.xlabel("X")
    plt.ylabel("u")
    plt.title("u(x, 0.75)")
plt.legend()
plt.savefig("far_end_x.png")

plt.clf()

for i in [50, 100, 200]:
    f = "solution_size_{}.mat".format(i)
    m = read_nalgebra_matrix(f)
    plt.plot(np.linspace(0, 1, i), m[int(3*i/4),:], label=str(i))
    plt.xlabel("Y")
    plt.ylabel("u")
    plt.title("u(0.75, y)")
plt.legend()
plt.savefig("far_end_y.png")

