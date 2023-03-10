import csv
import numpy as np

import scipy.optimize as opt
import matplotlib.pyplot as plt

xs = []
ys = []
zs = []

with open("pts.csv", "r") as f:
    reader = csv.reader(f, delimiter=",")
    for line in reader:
        xs.append(float(line[0]))
        ys.append(float(line[1]))
        zs.append(float(line[2]))


data = np.vstack((xs,ys))
vals = np.array(zs)

def paraBolEqn(data,a,x0,b,y0,c,z0):
    x,y = data
    return c*((((x-x0)/a)**2+((y-y0)/b)**2))+z0


fig = plt.figure()
ax = fig.add_subplot(projection='3d')

# plt.show()

popt,pcov=opt.curve_fit(paraBolEqn,data,vals,p0=[1.5,-1050.0,1.5,30,-1,-580], maxfev=1000000)



x = np.linspace(min(xs), max(xs), 30)
y = np.linspace(min(ys), max(ys), 30)

X, Y = np.meshgrid(x, y)
Z = paraBolEqn((X, Y), *popt)
print(Z)

ax.scatter(xs[0::100], ys[0::100], zs[0::100])
print(popt[1], popt[3])
ax.scatter([popt[1]], [popt[3]], [popt[5]])
ax.contour3D(X, Y, Z, 50, cmap='binary')
plt.show()

print(popt)

