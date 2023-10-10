/*

the plan:
R0 stores the value 0
R1 stores current loop count
R2 stores max/target loop count

*/

MOV R0 #0
MOV R1 #0
MOV R2 #500000

(loop_start)



ADD R1 R0 #1



/*
led_state = 0  # off
count_num = 0

loop:
    if count_num == TARGET:  # 500_000
        mem[led] = led_state
        led_state = ~led_state

    count_num += 1
*/
