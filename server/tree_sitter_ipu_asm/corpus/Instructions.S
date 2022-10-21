  ld32       $aSCRATCH, $mzero, $OUT_PTR,0
  {
    sub        $OUT_SIZE, $OUT_SIZE, 1
    sort4x16lo $aSCRATCH, $aSCRATCH, $azero
  }
