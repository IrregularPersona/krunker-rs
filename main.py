total = 1
for (i = 1; i <= 3; i++) {
    for (j = 6; j >= 2; j--) {
        if ((j % 2 == 0) && (i != 1)) {
            continue;
        }
        total = total * ( i + j )
    }
}

if (total > 100) {
    total = total / 2
}


OP SOURCE DESTINATION

