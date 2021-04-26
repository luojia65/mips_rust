	.section .text.entry 
	.align 4 
	.global entry 
#entry: 
	#la $a2, __cpu_init 
	#jr $a2 
	#mtc0 $zero, c0_count 
	#nop 

#__cpu_init: 
	#mfc0 $s1, c0_status 
	#ext $s1, $s1, 19, 1
	#beqz $s1, init_resources 
	#nop 
	#sdbbp 

#__init_resource: 
	#la $a2, init_cp0 
	#jalr $a2 
	#nop 

entry: 
	la $sp, boot_stack_top 

	.cprestore 4
	jal main 
	nop 

infinite_loop: 
	j infinite_loop 

	.section .bss.stack 
	.global boot_stack 
boot_stack: 
	.space 4096 * 4

	.global boot_stack_top 
boot_stack_top: 
