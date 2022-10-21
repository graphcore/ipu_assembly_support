* `f16v4hihoslic $aDst0, $aSrc0:Src0+1, $aSrc1, enumFlags`

*Half-precision* floating-point vector slim convolution.

Input partial-sums are *half-precision*. Results are *half-precision*

::: tabularcolumns
p{dimexpr 0.07linewidth-2tabcolsep}\>{centering}p{dimexpr
0.11linewidth-2tabcolsep}\>{centering}p{dimexpr
0.11linewidth-2tabcolsep}\>{centering}p{dimexpr
0.11linewidth-2tabcolsep}\>{centering}p{dimexpr
0.11linewidth-2tabcolsep}\>{centering}p{dimexpr
0.11linewidth-2tabcolsep}\>{centering}p{dimexpr
0.11linewidth-2tabcolsep}\>{centering}p{dimexpr
0.11linewidth-2tabcolsep}\>{centering}p{dimexpr
0.11linewidth-2tabcolsep}p{dimexpr 0.05linewidth-2tabcolsep}

::: rst-class
fullwidth

  ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
  \$aSrc0\|1    \$AACC\[14\]   \$AACC\[10\]           \$AACC\[6\]         \$AACC\[2\]        \$AACC\[12\]   \$AACC\[8\]            \$AACC\[4\]         \$AACC\[0\]        \$aDst0
  ------------- -------------- ---------------------- ------------------- ------------------ -------------- ---------------------- ------------------- ------------------ ---------
  x0\|P0,P1     **-**          R1=x0.**CW**5,0+P1     **-**               **-**              **-**          R0=x0.**CW**4,0+P0     **-**               **-**              **-**

  x1\|P2,P3     **-**          R3=x1.**CW**5,0+P3     R1+=x1.**CW**3,0    **-**              **-**          R2=x1.**CW**4,0+P2     R0+=x1.**CW**2,0    **-**              **-**

  x2\|P4,P5     **-**          R5=x2.**CW**5,0+P5     R3+=x2.**CW**3,0    R1+=x2.**CW**1,0   **-**          R4=x2.**CW**4,0+P4     R2+=x2.**CW**2,0    R0+=x2.**CW**0,0   **-**

  x3\|P6,P7     **-**          R7=x3.**CW**5,0+P7     R5+=x3.**CW**3,0    R3+=x3.**CW**1,0   **-**          R6=x3.**CW**4,0+P6     R4+=x3.**CW**2,0    R2+=x3.**CW**0,0   R0,R1

  x4\|P8,P9     **-**          R9=x4.**CW**5,0+P9     R7+=x4.**CW**3,0    R5+=x4.**CW**1,0   **-**          R8=x4.**CW**4,0+P8     R6+=x4.**CW**2,0    R4+=x4.**CW**0,0   R2,R3

  x5\|P10,P11   **-**          R11=x5.**CW**5,0+P11   R9+=x5.**CW**3,0    R7+=x5.**CW**1,0   **-**          R10=x5.**CW**4,0+P10   R8+=x5.**CW**2,0    R6+=x5.**CW**0,0   R4,R5

  x6\|P12,P13   **-**          R13=x6.**CW**5,0+P13   R11+=x6.**CW**3,0   R9+=x6.**CW**1,0   **-**          R12=x6.**CW**4,0+P12   R10+=x6.**CW**2,0   R8+=x6.**CW**0,0   R6,R7
  ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------

  : f16v4hihoslic, 2x1x3x4 example sequence

::: tabularcolumns
p{dimexpr 0.07linewidth-2tabcolsep}\>{centering}p{dimexpr
0.11linewidth-2tabcolsep}\>{centering}p{dimexpr
0.11linewidth-2tabcolsep}\>{centering}p{dimexpr
0.11linewidth-2tabcolsep}\>{centering}p{dimexpr
0.11linewidth-2tabcolsep}\>{centering}p{dimexpr
0.11linewidth-2tabcolsep}\>{centering}p{dimexpr
0.11linewidth-2tabcolsep}\>{centering}p{dimexpr
0.11linewidth-2tabcolsep}\>{centering}p{dimexpr
0.11linewidth-2tabcolsep}p{dimexpr 0.05linewidth-2tabcolsep}

::: rst-class
fullwidth

  -----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
  \$aSrc0\|1    \$AACC\[14\]           \$AACC\[10\]        \$AACC\[6\]        \$AACC\[2\]        \$AACC\[12\]           \$AACC\[8\]         \$AACC\[4\]        \$AACC\[0\]        \$aDst0
  ------------- ---------------------- ------------------- ------------------ ------------------ ---------------------- ------------------- ------------------ ------------------ ---------
  x0\|P0,P1     R1=x0.**CW**7,0+P1     **-**               **-**              **-**              R0=x0.**CW**6,0+P0     **-**               **-**              **-**              **-**

  x1\|P2,P3     R3=x1.**CW**7,0+P3     R1+=x1.**CW**5,0    **-**              **-**              R2=x1.**CW**6,0+P2     R0+=x1.**CW**4,0    **-**              **-**              **-**

  x2\|P4,P5     R5=x2.**CW**7,0+P5     R3+=x2.**CW**5,0    R1+=x2.**CW**3,0   **-**              R4=x2.**CW**6,0+P4     R2+=x2.**CW**4,0    R0+=x2.**CW**2,0   **-**              **-**

  x3\|P6,P7     R7=x3.**CW**7,0+P7     R5+=x3.**CW**5,0    R3+=x3.**CW**3,0   R1+=x3.**CW**1,0   R6=x3.**CW**6,0+P6     R4+=x3.**CW**4,0    R2+=x3.**CW**2,0   R0+=x3.**CW**0,0   **-**

  x4\|P8,P9     R9=x4.**CW**7,0+P9     R7+=x4.**CW**5,0    R5+=x4.**CW**3,0   R3+=x4.**CW**1,0   R8=x4.**CW**6,0+P8     R6+=x4.**CW**4,0    R4+=x4.**CW**2,0   R2+=x4.**CW**0,0   R0,R1

  x5\|P10,P11   R11=x5.**CW**7,0+P11   R9+=x5.**CW**5,0    R7+=x5.**CW**3,0   R5+=x5.**CW**1,0   R10=x5.**CW**6,0+P10   R8+=x5.**CW**4,0    R6+=x5.**CW**2,0   R4+=x5.**CW**0,0   R2,R3

  x6\|P12,P13   R13=x6.**CW**7,0+P13   R11+=x6.**CW**5,0   R9+=x6.**CW**3,0   R7+=x6.**CW**1,0   R12=x6.**CW**6,0+P12   R10+=x6.**CW**4,0   R8+=x6.**CW**2,0   R6+=x6.**CW**0,0   R4,R5
  -----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------

  : f16v4hihoslic, 1x4 example sequence

Pn is *half-precision* input partial-sum *n* xn is an *f16v4* input
vector **CW**m,n is the common weight state **\$CWEI**\_*m*\_*n* Rn is
the final *half-precision* result of successive dot-product
accumulations that began with Pn
