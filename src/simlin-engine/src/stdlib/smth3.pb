
stdlib⁚smth3@>
flow_1"unit/time unitB$
"
 (input - Stock_1)/(delay_time/3)\
Z
stock_1"unit*flow_1BA
?
=if isModuleInput(initial_value) then initial_value else input\
Z
stock_2"unit*flow_2BA
?
=if isModuleInput(initial_value) then initial_value else inputB@
flow_2"unit/time unitB&
$
"(Stock_1 - Stock_2)/(delay_time/3)[
Y
output"unit*flow_3BA
?
=if isModuleInput(initial_value) then initial_value else inputA?
flow_3"unit/time unitB%
#
!(Stock_2 - Output)/(delay_time/3) 

delay_time"	time unit2

1
input"unit2

0 
initial_value"unit2

NAN"�"!�Q��[^@"	!�G�z�U@"	! �rh�}\@"
!�A`��6b@"!     �V@LJ
flow 1-����d@!-��離d@(2	Zd;�O�Y@-��離d@2	-���rk@-��離d@
Stock 1-���Bn@!-��離d@
Stock 2���S-s@!-���
o@LJ
flow 2	-���"m@!-���
o@(2	-���zg@-���
o@2	���S�q@-���
o@
Output
���S}x@!���S�s@LJ
flow 3���S-s@!���St@(2	-���
m@���St@2	���Sw@���St@
$
"

delay time     �]@!���S-q@(

input     �Z@!     @]@("!(\���C@"!���{�r@"	!Y�� v@"!�n���/@'
%
initial value-���Bn@!      K@(:Zd;�O�Y@!-��離d@:	-���zg@!-���
o@:-���
m@!���St@" )      �?