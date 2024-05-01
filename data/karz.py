#!/usr/bin/python2.4
# Copyright 2008 Google Inc. All Rights Reserved.

"""One-line documentation for karz module.

A detailed description of karz.
"""

__author__ = 'cosmin@google.com (Silvestru Negruseri)'

import random


def print_header(left, right, edges):
    #  print >>file,
    global file
    print >>file, "c %d left nodes, %d right nodes" % (left, right)
    print >>file, "p max %d %d" % (left+right+2, edges + left + right)
    print >>file, "n %d s" % (left+right+1)
    print >>file, "n %d t" % (left+right+2)

def capacity():
    return random.randint(0, 2**24)

# d edges go from the right side to the first side
def HiLo(n1, n2, d):
    capacity1 = [0 for i in range(n1)]
    capacity2 = [0 for i in range(n2)]

    perm1 = [ i + 1 for i in range(n1) ]
    perm2 = [ i + 1 for i in range(n2) ]

    random.shuffle(perm1)
    random.shuffle(perm2)
    n = n1 + n2
    s = n + 1
    t = n + 2
    edges = []
    for i in range(n2):
        j = i % n1
        cap = capacity()
        capacity1[j] += cap
        capacity2[i] = cap
        for k in range(d):
            edges.append((j, i, cap))
            j -= 1
            if j < 0: break

    print_graph(n1, n2, edges, capacity1, capacity2)


def print_graph(n1, n2, edges, capacity1, capacity2):
    s = n1 + n2 + 1
    t = s + 1

    perm1 = [ i + 1 for i in range(n1) ]
    perm2 = [ i + 1 for i in range(n2) ]
    random.shuffle(perm1)
    random.shuffle(perm2)

    random.shuffle(edges)

    print_header(n1, n2, len(edges))

    global file
    for (v, u, f) in edges:
        print >>file, str(perm1[v]) + " " + str(n1 + perm2[u]) + " " + str(f)

    for i in range(n1):
        print >>file, str(s) + " " + str(perm1[i]) + " " + str(capacity1[i])

    for i in range(n2):
        print >>file, str(perm2[i] + n1) + " " + str(t) + " " + str(capacity2[i])



def add_edges_max(v1, v2, u1, u2, edges, capacity1, capacity2):
    j = v1
    for i in range(u1, u2 + 1):
        cap = capacity()
        edges.append((j, i, cap))
        capacity1[j] += cap
        capacity2[i] += cap
        j += 1
        if j > v2: j = v1
    return 0

def add_edges_random(v1, v2, u1, u2, edges, capacity1, capacity2):
    for i in range(u1, u2 + 1):
        vertexes = random.sample(xrange(v1, v2 + 1), v2 - v1)
        for j in vertexes:
            edges.append((j, i, capacity()))
    return 0

""" n1 and n2 multiples of d, average degree d """
def Rope(n1, n2, d):
    t = n1 / d
    if t == 0:
        print "error " + str(n1) + " " + str(n2) + " " + str(d)
        return 0
    d1 = n2 / t
    n1 = d * t
    n2 = d1 * t


    edges = []
    capacity1 = [0 for i in range(n1)]
    capacity2 = [0 for i in range(n2)]


    #  print "here" + " " + str(t)
    step = 0
    while step <= t - 1:
        v1, v2, v3, v4 = step * d, (step + 1) * d - 1, (step + 1) * d, (step + 2) * d - 1
        u1, u2, u3, u4 = step * d1, (step + 1) * d1 - 1, (step + 1) * d1, (step + 2) * d1 - 1
        if step + 1 <= t - 1:
            if step % 2 == 0:
                #        print "a"
                add_edges_max(v1, v2, u3, u4, edges, capacity1, capacity2)
                add_edges_max(v3, v4, u1, u2, edges, capacity1, capacity2)
            else:
                #        print "b"
                add_edges_random(v1, v2, u3, u4, edges, capacity1, capacity2)
                add_edges_random(v3, v4, u1, u2, edges, capacity1, capacity2)
        else:
            #      print "c"
            if step % 2 == 0: add_edges_max(v1, v2, u1, u2, edges, capacity1, capacity2)
            else: add_edges_random(v1, v2, u1, u2, edges, capacity1, capacity2)
        step += 1

    print_graph(n1, n2, edges, capacity1, capacity2)

    return 0

""" We print (n1 + n2) * d edges. Edge i-j has a probability of appearence
proportional to 1/(ij) """

def ZipF(n1, n2, d):
    sump = [ 0 for i in range(max(n1, n2) + 1) ]
    for i in range(1, n2 + 1):
        sump[i] = sump[i - 1] + 1.0 / i

    total_edges = d * (n1 + n2) // 2

    edge_set = set()
    edge_list = []


    for i in range(total_edges):
        finished = False
        while not finished:
            finished = True
            x = random.random()
            x = x * sump[n1]
            v = -1
            u = -1
            v1, v2 = 0, n1
            """ v1 can't point to the right node while v2 can. """
            while v2 - v1 > 1:
                mid = (v2 + v1) / 2
                if x > sump[mid]:
                    v1 = mid
                else: v2 = mid
            v = v2

            x = random.random() * sump[n2]
            u1, u2 = 0, n2
            while u2 - u1 > 1:
                mid = (u2 + u1) / 2
                if x > sump[mid]:
                    u1 = mid
                else:
                    u2 = mid
            u = u2
            if (str(v) + " " + str(u)) in edge_set:
                finished = False

        edge_list.append((v, u))
        edge_set.add(str(v) + " " + str(u))

    deg = [0 for i in range(n1 + n2 + 1)]

    capacity1 = [0 for i in range(n1)]
    capacity2 = [0 for i in range(n2)]

    edges = []

    for ed in edge_list:
        v, u = map(int, ed)
        deg[v] += 1
        deg[u + n1] += 1

        cap = capacity()

        capacity1[v - 1] += cap
        capacity2[u - 1] += cap
        edges.append((v - 1, u - 1, cap))

    for i in range(len(capacity1)): capacity1[i] = random.randint(0, capacity1[i])
    for i in range(len(capacity2)): capacity2[i] = random.randint(0, capacity2[i])

    print_graph(n1, n2, edges, capacity1, capacity2)

    return 0

for n in [20000, 30000, 40000]:
    #[10000, 20000, 30000, 50000, 100000, 200000, 250000]:
    print n
    for ratio in [5, 1000]:
        for d in [2, 10]:
            n1 = n / (ratio + 1)
            n2 = n - n1
            for method, name in [(ZipF, 'zipf'), (HiLo, 'hilo'), (Rope, 'rope')]:
                file = open('nodes-' + str(n) + '-ratio-' + str(ratio) + '-density-' + str(d) + '-' + name + '.in', 'w')
                print file.name
                method(n1, n2, d)
                file.close()
