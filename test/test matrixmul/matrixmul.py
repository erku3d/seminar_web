A = [3,2,1,1,0,2]
B = [1,2,4,5,0,1,3,2,4,0,0,1]
C = [0,0,0,0,0,0,0,0]

l = 2  #Zeilen A
m = 3  #Zeilen B, Spalten A
n = 4  #Spalten B

r = l * n  #groesse von C

for i in range(r):
    #hier parallel
    for j in range(m):
        C[i] = C[i] + A[ i / n * m + j] * B[(i % n) + j * n]

print C
