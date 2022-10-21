* `f32sisoslic $aDst0:Dst0+1, $aSrc0, $aSrc1:Src1+1, enumFlags`

*Single-precision* floating-point **sli**m **c**onvolution. Input
partial-sums are *single-precision*. Results are *single-precision*.

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

  --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
  \$aSrc0\|1     \$AACC\[14\]   \$AACC\[10\]             \$AACC\[6\]          \$AACC\[2\]          \$AACC\[12\]   \$AACC\[8\]              \$AACC\[4\]          \$AACC\[0\]          \$aDst0
  -------------- -------------- ------------------------ -------------------- -------------------- -------------- ------------------------ -------------------- -------------------- ---------
  x0L\|P0,P1     **-**          R1=x0L×**CW**5,0L+P1     **-**                **-**                **-**          R0=x0L×**CW**4,0L+P0     **-**                **-**                **-**

  x0U\|D0,D1     **-**          R1+=x0U×**CW**5,0U       **-**                **-**                **-**          R0+=x0U×**CW**4,0U       **-**                **-**                **-**

  x1L\|P2,P3     **-**          R3=x1L×**CW**5,0L+P3     R1+=x1L×**CW**3,0L   **-**                **-**          R2=x1L×**CW**4,0L+P2     R0+=x1L×**CW**2,0L   **-**                **-**

  x1U\|D2,D3     **-**          R3+=x1U×**CW**5,0U       R1+=x1U×**CW**3,0U   **-**                **-**          R2+=x1U×**CW**4,0U       R0+=x1U×**CW**2,0U   **-**                **-**

  x2L\|P4,P5     **-**          R5=x2L×**CW**5,0L+P5     R3+=x2L×**CW**3,0L   R1+=x2L×**CW**1,0L   **-**          R4=x2L×**CW**4,0L+P4     R2+=x2L×**CW**2,0L   R0+=x2L×**CW**0,0L   **-**

  x2U\|D4,D5     **-**          R5+=x2U×**CW**5,0U       R3+=x2U×**CW**3,0U   R1+=x2U×**CW**1,0U   **-**          R4+=x2U×**CW**4,0U       R2+=x2U×**CW**2,0U   R0+=x2U×**CW**0,0U   **-**

  x3L\|P6,P7     **-**          R7=x3L×**CW**5,0L+P7     R5+=x3L×**CW**3,0L   R3+=x3L×**CW**1,0L   **-**          R6=x3L×**CW**4,0L+P6     R4+=x3L×**CW**2,0L   R2+=x3L×**CW**0,0L   R0,R1

  x3U\|D6,D7     **-**          R7+=x3U×**CW**5,0U       R5+=x3U×**CW**3,0U   R3+=x3U×**CW**1,0U   **-**          R6+=x3U×**CW**4,0U       R4+=x3U×**CW**2,0U   R2+=x3U×**CW**0,0U   **-**

  x4L\|P8,P9     **-**          R9=x4L×**CW**5,0L+P9     R7+=x4L×**CW**3,0L   R5+=x4L×**CW**1,0L   **-**          R8=x4L×**CW**4,0L+P8     R6+=x4L×**CW**2,0L   R4+=x4L×**CW**0,0L   R2,R3

  x4U\|D8,D9     **-**          R9+=x4U×**CW**5,0U       R7+=x4U×**CW**3,0U   R5+=x4U×**CW**1,0U   **-**          R8+=x4U×**CW**4,0U       R6+=x4U×**CW**2,0U   R4+=x4U×**CW**0,0U   **-**

  x5L\|P10,P11   **-**          R11=x5L×**CW**5,0L+P11   R9+=x5L×**CW**3,0L   R7+=x5L×**CW**1,0L   **-**          R10=x5L×**CW**4,0L+P10   R8+=x5L×**CW**2,0L   R6+=x5L×**CW**0,0L   R4,R5

  x5U\|D10,D11   **-**          R11+=x5U×**CW**5,0U      R9+=x5U×**CW**3,0U   R7+=x5U×**CW**1,0U   **-**          R10+=x5U×**CW**4,0U      R8+=x5U×**CW**2,0U   R6+=x5U×**CW**0,0U   **-**
  --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------

  : f32sisoslic, 2x1x3x2 example sequence

Pn is *single-precision* input partial-sum *n* Dn is 0 under normal
circumstances (`$a14:15`) xnL is the 1st element (element 0) of a
*f32v2* input vector xnU is the 2nd element (element 1) of a *f32v2*
input vector **CW**m,nL is the least significant 32-bits of common
weight state **\$CWEI**\_*m*\_*n* **CW**m,nU is the most significant
32-bits of common weight state **\$CWEI**\_*m*\_*n* Rn is the final
*single-precision* result of successive multiply-accumulations that
began with Pn

enumFlags format:

![f32sisoslic immediate
format](images/autogen/F32SLIC_ENUMFLAGS.*){.align-center}

2 output channels are processed/produced.
