
===
addi r2 = r0, 0x100
beq r0, (r0, r0) -> @main.global

@main.global
subi r2 = r2, 4
sw r2[0] = r3
add r3 = r0, r2
subi r2 = r2, 8
sw r3[-4] = r1
sw r3[-8] = r6
subi r2 = r2, 12
add r6 = r0, r2
subi r2 = r2, 4
addi r4 = r0, 12
sw r2[0] = r4
addi r4 = r0, 10
subi r2 = r2, 4
sw r2[0] = r4
lw r4 = r2[0]
addi r2 = r2, 4
sw r6[0] =  r4
addi r4 = r0, 20
subi r2 = r2, 4
sw r2[0] = r4
lw r4 = r2[0]
addi r2 = r2, 4
sw r6[4] =  r4
lw r4 = r6[0]
subi r2 = r2, 4
sw r2[0] = r4
lw r4 = r6[4]
subi r2 = r2, 4
sw r2[0] = r4
lw r5 = r2[0]
addi r2 = r2, 4
lw r4 = r2[0]
addi r2 = r2, 4
add r4 = r4, r5
subi r2 = r2, 4
sw r2[0] = r4
lw r4 = r2[0]
addi r2 = r2, 4
sw r6[8] =  r4
addi r4 = r0, 0
subi r2 = r2, 4
sw r2[0] = r4
lw r4 = r2[0]
addi r2 = r2, 4
addi r10 = r0, 0
add r10 = r10, r4
lw r4 = r2[0]
add r2 = r2, r4
addi r2 = r2, 4
lw r1 = r3[-4]
lw r6 = r3[-8]
addi r2 = r2, 8
lw r3 = r2[0]
addi r2 = r2, 4
jal r0, r1[0]
lw r4 = r2[0]
add r2 = r2, r4
addi r2 = r2, 4
lw r1 = r3[-4]
lw r6 = r3[-8]
addi r2 = r2, 8
lw r3 = r2[0]
addi r2 = r2, 4
jal r0, r1[0]
