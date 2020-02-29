import matplotlib.pyplot as plt
import numpy as np

def anal(x, y):
    return np.cos(2*x)*np.exp(-2*y)

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

def get_middle_val(i):
    f = "solution_size_{}.mat".format(i)
    m = read_nalgebra_matrix(f)
    return m[int(i/2), int(i/2)]

sizes = [25, 51, 101, 201, 401]
a = np.array([ get_middle_val(i) for i in sizes ])
print(a)
print(anal(0.5, 0.5))
print(a - anal(0.5, 0.5))

x = np.log(sizes)
y = np.log(np.abs(a - anal(0.5, 0.5)))
plt.scatter(x, y)

from scipy.optimize import curve_fit
p, _ = curve_fit( lambda x,a,b: a*x + b, x, y,)
print(p)

plt.plot(x, p[0]*x + p[1], label=r"$y = {:.3f} \cdot x {:.3f}$".format(p[0], p[1]))

plt.legend()
plt.xlabel(r"$log(N)$")
plt.ylabel(r"$log(|u - u_{analytical}|)$")
plt.title("Error convergence")
plt.savefig("error.png")

