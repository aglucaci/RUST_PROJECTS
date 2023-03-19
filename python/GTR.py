from Bio.Seq import Seq
from Bio import AlignIO
from collections import Counter
import numpy as np
import math
import os

# Load alignment from file
alignment = AlignIO.read(os.path.join("..",
                                        "data",
                                        "SPIKE",
                                        "VOI_SPIKE.fasta"), "fasta")

# Define the nucleotides and their frequencies
nucleotides = ['A', 'C', 'G', 'T']

#nuc_counts = Counter(alignment)



# Create the nucleotide frequency matrix
nuc_freqs = []

for nuc1 in nucleotides:
    row = []
    for nuc2 in nucleotides:
        row.append(nuc_counts[nuc1] * nuc_counts[nuc2])
    nuc_freqs.append(row)
    #end for
#end for
nuc_freqs = np.array(nuc_freqs)
nuc_freqs = nuc_freqs / np.sum(nuc_freqs)

# Calculate the transition/transversion ratio (kappa)
ts = nuc_counts['A'] * nuc_counts['G'] + nuc_counts['C'] * nuc_counts['T']
tv = nuc_counts['A'] * (nuc_counts['C'] + nuc_counts['T']) + \
     nuc_counts['G'] * (nuc_counts['C'] + nuc_counts['T'])
kappa = tv / ts

# Calculate the base frequencies (pi)
pi = nuc_counts.values() / np.sum(list(nuc_counts.values()))

# Calculate the rate matrix (Q)
Q = np.zeros((4,4))

for i in range(4):
    for j in range(4):
        if i != j:
            Q[i][j] = nuc_freqs[i][j] * kappa
    Q[i][i] = -np.sum(Q[i])

# Calculate the equilibrium frequencies (pi_star)
pi_star = np.linalg.inv(Q) @ pi
pi_star = pi_star / np.sum(pi_star)

# Calculate the GTR model parameters
alpha = -1 / (2 * (pi_star @ Q @ pi))
beta = -1 / (2 * (pi @ np.linalg.inv(Q) @ pi_star))
gamma = -1 / (2 * (pi_star @ np.linalg.inv(Q) @ pi))
delta = -math.log(np.sqrt(pi_star @ Q @ pi * pi @ np.linalg.inv(Q) @ pi_star))

print("alpha =", alpha)
print("beta =", beta)
print("gamma =", gamma)
print("delta =", delta)

