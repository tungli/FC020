import numpy as np
import matplotlib.pyplot as plt


def plot_diffusion(ts): 
    styles = reversed(['-' ,'--','-.',':'])

    for i,s in zip(ts, styles): 
        data = np.genfromtxt('results_{}.out'.format(i)) 
        t = data[0][0] 
        x, y = zip(*data[1:]) 
        plt.plot(x, np.array(y)/np.max(y), 'k{}'.format(s), label=r"$t=${:.1e} s".format(t)) 

    plt.plot(x, 25/np.pi*np.sin(np.pi*np.array(x)/25.0)/np.array(x), 'r--', label='anal.: $n=0$')

    plt.xlabel(r"$x$ [cm]") 
    plt.ylabel(r"$u/u_{max}$") 
    plt.title("Diffusion") 
    plt.legend() 
     

plot_diffusion([0,1,2,3,4])
plt.savefig("norm_diffusion.png")
