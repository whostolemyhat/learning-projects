# algorithm from http://stackoverflow.com/questions/14008521/please-explain-this-algorithm-to-get-all-permutations-of-a-string
def permutations(word):
    if len(word)<=1:
        return [word]

    #get all permutations of length N-1
    perms=permutations(word[1:])
    char=word[0]
    result=[]
    #iterate over all permutations of length N-1
    for perm in perms:
        #insert the character into every possible location
        for i in range(len(perm)+1):
            print(perm, i, len(perm)+1)
            print('{} {} {}'.format(perm[:i], char, perm[i:]))

            result.append(perm[:i] + char + perm[i:])
    return result

print(permutations("bar"))