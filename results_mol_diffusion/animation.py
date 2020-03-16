import numpy as np
import matplotlib.pyplot as plt
from matplotlib.animation import FuncAnimation

N = 100
data = [ np.genfromtxt('results_{}.out'.format(i)) for i in range(N) ]

fig, ax = plt.subplots()
plt.xlabel(r"$x$ [cm]") 
plt.ylabel(r"$u$") 
plt.title("Diffusion with recombination") 
t_text = ax.text(0.70, 0.80, '', transform=ax.transAxes)

x = data[0][1:,0]
y_max0 = np.max(data[0][:,1])
x_max = np.max(x)

def anal(scale):
    x[0] = 1e-20
    a = np.sin(np.pi*np.array(x)/25.0)/np.array(x)
    return scale*a/(np.pi/25)

ln_a, = ax.plot(x, anal(y_max0), 'r--', label='anal.: $n=0$')
ln, = ax.plot([], [], lw=2, color='k')

def update(num):
    y = data[num][1:,1]
    y_max = np.max(y)
    ln.set_data(x, y)
    ln_a.set_data(x, anal(y_max))
    ax.set_ylim(0, 1.1*y_max)
    t = data[num][0,0]
    t_text.set_text("time = {:.5f}".format(t))
    ax.figure.canvas.draw()
    return ln,

def init_f():
    y = np.genfromtxt('results_0.out')[1:,1]
    ax.set_ylim(0, 1.1*np.max(y))
    ax.set_xlim(0, x_max)
    plt.legend()
    return ln,

line_ani = FuncAnimation(fig, update, init_func=init_f, frames=range(N), blit=True)
line_ani.save('animation.gif', writer='imagemagick', fps=30)
plt.show()
