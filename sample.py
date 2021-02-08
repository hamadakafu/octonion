import numpy as np
from icecream import ic

# quaternion multiplication
def qmult(x, y):
    return np.array(
        [
            x[0] * y[0] - x[1] * y[1] - x[2] * y[2] - x[3] * y[3],
            x[0] * y[1] + x[1] * y[0] + x[2] * y[3] - x[3] * y[2],
            x[0] * y[2] - x[1] * y[3] + x[2] * y[0] + x[3] * y[1],
            x[0] * y[3] + x[1] * y[2] - x[2] * y[1] + x[3] * y[0],
        ]
    )


# quaternion conjugate
def qstar(x):
    return x * np.array([1, -1, -1, -1])


# octonion multiplication
def omult(x, y):
    # Split octonions into pairs of quaternions
    a, b = x[:4], x[4:]
    c, d = y[:4], y[4:]

    z = np.zeros(8)
    z[:4] = qmult(a, c) - qmult(qstar(d), b)
    z[4:] = qmult(d, a) + qmult(b, qstar(c))
    return z


a = [
    0,
    0,
    0,
    2,
    2,
    1,
    4,
    0,
]
b = [
    4,
    2,
    4,
    3,
    1,
    2,
    2,
    0,
]
ic(omult(b, a))
